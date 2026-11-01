//! Core domain types for genomic data representation in Prexager.
//!
//! These types are owned by `prexager-core` and form the contract between
//! the I/O layer (`prexager-io`) and the rendering/simulation layers.
//! No third-party type (e.g., `cooler-rs`) should appear in this module.

use thiserror::Error;

//---------------------------------------------------------------------------------------------
// 1. Identity Types (dense integer identity, no string keys in hot paths)
//---------------------------------------------------------------------------------------------

/// Dense integer identifier for a chromosome. Zero-based.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ChromosomeId(pub u32);

/// Dense integer identifier for a genomic bin. Zero-based, global across all chromosomes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BinId(pub u32);

//---------------------------------------------------------------------------------------------
// 2. Metadata & Topology
//---------------------------------------------------------------------------------------------

/// Topology and identity information for a single chromosome within a dataset.
///
/// `start_bin` is the global bin index where this chromosome begins; it enables
/// O(1) mapping from a `BinId` back to its owning chromosome in hot-path code
/// (e.g., picking, shader coloring) without binary search.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ChromosomeSpec {
    pub id: ChromosomeId,
    pub name: String,
    pub length_bp: u64,
    pub start_bin: u32,
}

impl ChromosomeSpec {
    /// Constructs a new `ChromosomeSpec`.
    #[must_use]
    pub fn new(id: ChromosomeId, name: impl Into<String>, length_bp: u64, start_bin: u32) -> Self {
        Self {
            id,
            name: name.into(),
            length_bp,
            start_bin,
        }
    }
}

/// Metadata for a single-resolution `.cool` contact matrix.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct CoolerMetadata {
    pub resolution_bp: u32,
    pub bin_count: u32,
    pub chromosomes: Vec<ChromosomeSpec>,
}

impl CoolerMetadata {
    /// Constructs a new `CoolerMetadata`.
    #[must_use]
    pub fn new(resolution_bp: u32, bin_count: u32, chromosomes: Vec<ChromosomeSpec>) -> Self {
        Self {
            resolution_bp,
            bin_count,
            chromosomes,
        }
    }
}

/// Index of a multi-resolution `.mcool` file.
///
/// Allows the user to select a resolution before loading the full contact
/// matrix into memory.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct McoolIndex {
    /// Available resolutions in base pairs, e.g. `[1000, 5000, 10000, 1_000_000]`.
    pub resolutions: Vec<u32>,
    /// Metadata for each corresponding resolution (parallel to `resolutions`).
    pub metadata_per_resolution: Vec<CoolerMetadata>,
}

impl McoolIndex {
    /// Constructs a new `McoolIndex`.
    ///
    /// # Panics
    ///
    /// Panics in debug mode if the two vectors have different lengths.
    #[must_use]
    pub fn new(resolutions: Vec<u32>, metadata_per_resolution: Vec<CoolerMetadata>) -> Self {
        debug_assert_eq!(
            resolutions.len(),
            metadata_per_resolution.len(),
            "McoolIndex: resolutions and metadata vectors must have equal length"
        );
        Self {
            resolutions,
            metadata_per_resolution,
        }
    }
}

//---------------------------------------------------------------------------------------------
// 3. Balancing Weights
//---------------------------------------------------------------------------------------------

/// ICE or KR normalization weights.
///
/// `weights[bin_id.0 as usize]` is the multiplier for that bin.
/// A weight of 0.0 or `NaN` typically indicates a blacklisted or unmappable bin.
#[derive(Debug, Clone)]
pub struct BinWeights {
    pub values: Vec<f32>,
}

impl BinWeights {
    /// Wraps an existing weight vector.
    #[must_use]
    pub fn new(values: Vec<f32>) -> Self {
        Self { values }
    }

    /// Returns the weight for a given bin, or `None` if the index is out of range.
    #[inline]
    #[must_use]
    pub fn get(&self, bin: BinId) -> Option<f32> {
        self.values.get(bin.0 as usize).copied()
    }
}

//---------------------------------------------------------------------------------------------
// 4. Contact Matrix: CSR (Compressed Sparse Row)
//---------------------------------------------------------------------------------------------

/// Errors raised when CSR invariants are violated.
#[derive(Debug, Error)]
pub enum CsrError {
    #[error("CSR offsets length ({offsets_len}) must be exactly bin_count + 1 ({expected})")]
    InvalidOffsetsLength { offsets_len: usize, expected: usize },

    #[error("CSR columns length ({columns_len}) must match values length ({values_len})")]
    MismatchedDataLengths {
        columns_len: usize,
        values_len: usize,
    },

    #[error("CSR offsets must be monotonically non-decreasing")]
    NonMonotonicOffsets,
}

/// Compressed Sparse Row representation of a Hi-C contact matrix.
///
/// Mirrors the `.cool` sparse-pixel schema.
/// For a matrix of `bin_count` bins:
/// - `offsets` has length `bin_count + 1`.
/// - The contacts for bin `i` are `columns[offsets[i]..offsets[i+1]]` with
///   matching values in `values[offsets[i]..offsets[i+1]]`.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ContactCSR {
    /// Length: `bin_count + 1`.
    pub offsets: Vec<u64>,
    /// Column indices (bin2_id) for each non-zero contact.
    pub columns: Vec<u32>,
    /// Contact frequencies (or balanced values) for each non-zero contact.
    pub values: Vec<f32>,
}

impl ContactCSR {
    /// Validates CSR invariants and constructs a `ContactCSR`.
    ///
    /// Prevents silent memory corruption and out-of-bounds reads in hot-path
    /// code that slices this matrix (determinism and stability).
    pub fn try_new(
        offsets: Vec<u64>,
        columns: Vec<u32>,
        values: Vec<f32>,
        bin_count: u32,
    ) -> Result<Self, CsrError> {
        let expected_offsets_len = bin_count as usize + 1;
        if offsets.len() != expected_offsets_len {
            return Err(CsrError::InvalidOffsetsLength {
                offsets_len: offsets.len(),
                expected: expected_offsets_len,
            });
        }
        if columns.len() != values.len() {
            return Err(CsrError::MismatchedDataLengths {
                columns_len: columns.len(),
                values_len: values.len(),
            });
        }

        // Verify monotonicity — critical for safe slicing.
        for window in offsets.windows(2) {
            if window[0] > window[1] {
                return Err(CsrError::NonMonotonicOffsets);
            }
        }

        // `Self { ... }` is valid here because we are inside the same crate
        // that defines the `#[non_exhaustive]` struct.
        Ok(Self {
            offsets,
            columns,
            values,
        })
    }

    /// Returns the slice of column indices belonging to `bin`.
    #[inline]
    #[must_use]
    pub fn columns_for_bin(&self, bin: BinId) -> &[u32] {
        let start = self.offsets[bin.0 as usize] as usize;
        let end = self.offsets[bin.0 as usize + 1] as usize;
        &self.columns[start..end]
    }

    /// Returns the slice of values belonging to `bin`.
    #[inline]
    #[must_use]
    pub fn values_for_bin(&self, bin: BinId) -> &[f32] {
        let start = self.offsets[bin.0 as usize] as usize;
        let end = self.offsets[bin.0 as usize + 1] as usize;
        &self.values[start..end]
    }
}
