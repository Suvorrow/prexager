//! Anti-Corruption Layer (ACL) for `.cool` (HDF5-based) genomic contact matrices.
//!
//! This module isolates the `prexager-core` domain model from the specific
//! implementation details and error types of the `cooler-rs` / `hdf5` ecosystem.
//!
//! ## Path Classification (Variant B — Cold Path)
//! This is a **cold path** (executes at startup/data load). Idiomatic allocations
//! (`String`, `Vec`) are permitted here. Optimization requires evidence, not assumption.
//! However, strict capacity validation is enforced to protect the 8 GiB memory budget.

use std::path::Path;
use thiserror::Error;

use prexager_core::genome::{ChromosomeId, ChromosomeSpec, CoolerMetadata};

//---------------------------------------------------------------------------------------------
// 1. Error Domain
//---------------------------------------------------------------------------------------------

#[derive(Debug, Error)]
pub enum CoolError {
    #[error("file not found or inaccessible at '{path}': {reason}")]
    FileNotFound { path: String, reason: String },

    #[error("I/O error reading '{path}': {reason}")]
    IoError { path: String, reason: String },

    #[error("resolution mismatch: expected {expected} bp, file contains {found} bp")]
    ResolutionMismatch { expected: u32, found: u32 },

    #[error("memory budget exceeded: file requires {found} bins, capacity limit is {limit}")]
    CapacityExceeded { found: u64, limit: u64 },

    #[error("malformed cooler schema: {0}")]
    MalformedSchema(String),

    #[error("invalid chromosome at index {index}: {reason}")]
    InvalidChromosome { index: usize, reason: String },

    #[error("integer overflow during calculation: {operation}")]
    Overflow { operation: String },
}

//---------------------------------------------------------------------------------------------
// 2. Constants
//---------------------------------------------------------------------------------------------

/// Maximum reasonable chromosome count (human has 24 distinct types;
/// metagenomics assemblies may have thousands of contigs).
const MAX_CHROMOSOMES: usize = 10_000;

//---------------------------------------------------------------------------------------------
// 3. Public API
//---------------------------------------------------------------------------------------------

/// Reads the metadata and chromosome topology from a `.cool` file.
///
/// # Arguments
/// * `path` — Path to the `.cool` (HDF5) file.
/// * `expected_resolution` — The required bin size in base pairs.
/// * `capacity_bins` — Maximum number of bins the system may allocate (must be > 0).
///
/// # Errors
/// Returns [`CoolError`] if the file is missing, malformed, or exceeds capacity.
pub fn load_cool_metadata<P: AsRef<Path>>(
    path: P,
    expected_resolution: u32,
    capacity_bins: u32,
) -> Result<CoolerMetadata, CoolError> {
    let path_ref = path.as_ref();
    let path_str = path_ref.to_string_lossy().into_owned();

    if capacity_bins == 0 {
        return Err(CoolError::MalformedSchema(
            "capacity_bins must be greater than zero".into(),
        ));
    }

    // 1. Open the cooler file (cold path: blocking I/O is acceptable).
    let cooler = open_cooler_file(path_ref, &path_str)?;

    // TODO: Validate format attribute ("HDF5::Cooler"), format-version (3),
    // and storage-mode ("symmetric-upper") once cooler-rs exposes these
    // as public methods. Currently they are internal HDF5 attributes
    // without public accessors.

    // 2. Validate capacity (memory budget enforcement).
    let file_bin_count = extract_bin_count(&cooler, &path_str)?;
    if file_bin_count > capacity_bins as u64 {
        return Err(CoolError::CapacityExceeded {
            found: file_bin_count,
            limit: capacity_bins as u64,
        });
    }

    // TODO: Validate actual file resolution against `expected_resolution`
    // once cooler-rs exposes a resolution/binsize accessor. Currently the
    // caller-provided value is used as authoritative.

    // 3. Extract chromosome topology.
    let chromosomes =
        extract_chromosome_specs(&cooler, expected_resolution, file_bin_count, &path_str)?;

    Ok(CoolerMetadata::new(
        expected_resolution,
        file_bin_count as u32,
        chromosomes,
    ))
}

//---------------------------------------------------------------------------------------------
// 4. Private Helpers
//---------------------------------------------------------------------------------------------

fn open_cooler_file(path: &Path, path_str: &str) -> Result<cooler_rs::Cooler, CoolError> {
    cooler_rs::Cooler::open(path).map_err(|e| {
        let error_msg = e.to_string();
        if error_msg.contains("not found") || error_msg.contains("No such file") {
            CoolError::FileNotFound {
                path: path_str.to_string(),
                reason: error_msg,
            }
        } else {
            CoolError::IoError {
                path: path_str.to_string(),
                reason: error_msg,
            }
        }
    })
}

fn extract_bin_count(cooler: &cooler_rs::Cooler, _path_str: &str) -> Result<u64, CoolError> {
    let bins = cooler
        .bins()
        .map_err(|e| CoolError::MalformedSchema(format!("cannot read bins table: {e}")))?;

    u64::try_from(bins.len()).map_err(|_| CoolError::Overflow {
        operation: format!("bin count {} exceeds u64::MAX", bins.len()),
    })
}

/// Iterates over the cooler chromosomes to build the dense `ChromosomeSpec` array.
///
/// Assigns dense `ChromosomeId`s (0..N) and calculates the global `start_bin`
/// offset for each chromosome using ceiling division of chromosome length by
/// resolution. This `start_bin` is critical for mapping 1D bin indices to 2D
/// contact pixels in the CSR matrix.
fn extract_chromosome_specs(
    cooler: &cooler_rs::Cooler,
    resolution_bp: u32,
    total_bins: u64,
    _path_str: &str,
) -> Result<Vec<ChromosomeSpec>, CoolError> {
    let chroms = cooler
        .chroms()
        .map_err(|e| CoolError::MalformedSchema(format!("cannot read chromosome table: {e}")))?;

    let mut specs = Vec::with_capacity(chroms.len());
    let mut current_start_bin: u64 = 0;
    let mut chr_id_counter: u32 = 0;

    for (index, chr) in chroms.iter().enumerate() {
        if index >= MAX_CHROMOSOMES {
            return Err(CoolError::MalformedSchema(format!(
                "chromosome count exceeds reasonable maximum ({MAX_CHROMOSOMES})"
            )));
        }

        // Chrom fields are accessed directly (struct fields, not methods),
        // consistent with the cooler-rs write API: Chrom { name, length }.
        let name = chr.name.clone();
        if name.is_empty() {
            return Err(CoolError::InvalidChromosome {
                index,
                reason: "chromosome name is empty".into(),
            });
        }

        // TODO: Verify the integer type of Chrom::length against cooler-rs source.
        let length_bp = chr.length as u64;
        if length_bp == 0 {
            return Err(CoolError::InvalidChromosome {
                index,
                reason: "chromosome length must be positive".into(),
            });
        }

        // Compute bin count for this chromosome via ceiling division.
        let chr_bin_count = (length_bp + resolution_bp as u64 - 1) / resolution_bp as u64;

        specs.push(ChromosomeSpec::new(
            ChromosomeId(chr_id_counter),
            name,
            length_bp,
            current_start_bin as u32,
        ));

        current_start_bin = current_start_bin
            .checked_add(chr_bin_count)
            .ok_or_else(|| CoolError::Overflow {
                operation: format!("cumulative bin count overflowed at chromosome {index}"),
            })?;

        chr_id_counter = chr_id_counter
            .checked_add(1)
            .ok_or_else(|| CoolError::Overflow {
                operation: "chromosome ID counter overflowed u32".into(),
            })?;
    }

    // Sanity check: computed bin sum must match the file's total bin count.
    if current_start_bin != total_bins {
        return Err(CoolError::MalformedSchema(format!(
            "computed chromosome bin sum ({current_start_bin}) does not match \
             file bin_count ({total_bins}); the file may use a non-standard \
             binning scheme"
        )));
    }

    Ok(specs)
}

//---------------------------------------------------------------------------------------------
// 5. Tests
//---------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use cooler_rs::{Chrom, CoolerWriter, Pixel};
    use tempfile::Builder as TempBuilder;

    /// Helper to create a temporary `.cool` file with a known, deterministic structure.
    ///
    /// Returns the `NamedTempFile` handle (which guarantees deletion on drop)
    /// and the path to the file.
    fn create_tiny_cool_file() -> (tempfile::NamedTempFile, std::path::PathBuf) {
        // Create a temporary file path with a `.cool` suffix.
        let temp_file = TempBuilder::new()
            .suffix(".cool")
            .tempfile()
            .expect("Failed to create temp file");

        let path = temp_file.path().to_path_buf();

        // `tempfile` creates an empty 0-byte file. `CoolerWriter::create` expects
        // to create the file itself, so we remove the empty placeholder first.
        // The `NamedTempFile` handle still tracks the path for cleanup on drop.
        let _ = std::fs::remove_file(&path);

        // 1. Define chromosomes: chr1 (1Mb) and chr2 (500kb)
        let chroms = vec![
            Chrom {
                name: "chr1".into(),
                length: 1_000_000,
            },
            Chrom {
                name: "chr2".into(),
                length: 500_000,
            },
        ];

        // 2. Write the file at 100kb resolution
        let writer =
            CoolerWriter::create(&path, &chroms, 100_000).expect("Failed to create CoolerWriter");

        writer
            .write_pixels(&[Pixel {
                bin1_id: 0,
                bin2_id: 3,
                count: 42.0,
            }])
            .expect("Failed to write pixel");

        // 3. Explicitly drop the writer to flush and close the HDF5 handle
        // before we attempt to read it back.
        drop(writer);

        (temp_file, path)
    }

    #[test]
    fn round_trip_metadata_success() {
        let (_temp_file, path) = create_tiny_cool_file();

        // chr1 (1Mb / 100kb) = 10 bins.
        // chr2 (500kb / 100kb) = 5 bins.
        // Total expected bins = 15.
        let meta = load_cool_metadata(&path, 100_000, 64).expect("Failed to load metadata");

        // Validate top-level metadata
        assert_eq!(meta.resolution_bp, 100_000);
        assert_eq!(meta.bin_count, 15);
        assert_eq!(meta.chromosomes.len(), 2);

        // Validate chr1 topology
        let chr1 = &meta.chromosomes[0];
        assert_eq!(chr1.id, ChromosomeId(0));
        assert_eq!(chr1.name, "chr1");
        assert_eq!(chr1.length_bp, 1_000_000);
        assert_eq!(chr1.start_bin, 0);

        // Validate chr2 topology (must start exactly where chr1 ended)
        let chr2 = &meta.chromosomes[1];
        assert_eq!(chr2.id, ChromosomeId(1));
        assert_eq!(chr2.name, "chr2");
        assert_eq!(chr2.length_bp, 500_000);
        assert_eq!(chr2.start_bin, 10);
    }

    #[test]
    fn capacity_exceeded_error() {
        let (_temp_file, path) = create_tiny_cool_file();

        // The file requires 15 bins. Set capacity limit to 10.
        let result = load_cool_metadata(&path, 100_000, 10);

        assert!(
            matches!(
                result,
                Err(CoolError::CapacityExceeded {
                    found: 15,
                    limit: 10
                })
            ),
            "Expected CapacityExceeded, got {:?}",
            result
        );
    }

    #[test]
    fn file_not_found_error() {
        let result = load_cool_metadata("/tmp/definitely_does_not_exist_12345.cool", 100_000, 64);

        assert!(result.is_err());
        let err = result.unwrap_err();

        // Depending on the exact HDF5 error string, it maps to FileNotFound or IoError.
        assert!(
            matches!(
                err,
                CoolError::FileNotFound { .. } | CoolError::IoError { .. }
            ),
            "Expected FileNotFound or IoError, got {:?}",
            err
        );
    }

    #[test]
    fn zero_capacity_rejected() {
        // Validate the upfront input sanity check
        let result = load_cool_metadata("/tmp/dummy.cool", 100_000, 0);
        assert!(
            matches!(result, Err(CoolError::MalformedSchema(_))),
            "Expected MalformedSchema for zero capacity, got {:?}",
            result
        );
    }
}
