// SPDX-FileCopyrightText: 2025 Mikhail Suvorau (Suvorrow) <suvoraum@gmail.com>
// SPDX-License-Identifier: GPL-3.0-only

//! Type definitions for file dialog filters.

use std::borrow::Cow;

/// Afile extension filter for native file dialogs.
///
/// Uses 'Cow<'static, str>' to allow both static literals and runtime-generated
/// names without unnecessary allocations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileFilter {
    /// Human-readable label shown in the dialog (e.g., "COOL files").
    pub name: Cow<'static, str>,
    /// File extensions to match (e.g., '["cool"]').
    pub extensions: Cow<'static, [&'static str]>,
}

impl FileFilter {
    /// Creates a 'FileFilter' from static strings with zero runtime allocations.
    ///
    /// # Arguments
    /// * 'name' - Human-readable label shown in the file dialog (e.g. "COOL files").
    /// * 'extensions' - File extensions to match (e.g., '&["cool"]'). Use '&["*"]' for all files.
    #[inline]
    #[must_use]
    pub const fn new_static(name: &'static str, extensions: &'static [&'static str]) -> Self {
        Self {
            name: Cow::Borrowed(name),
            extensions: Cow::Borrowed(extensions),
        }
    }

    /// Create a new filter from owned or mixed strings (allocates if needed).
    #[inline]
    #[must_use]
    pub fn new(name: impl Into<Cow<'static, str>>, extensions: Vec<&'static str>) -> Self {
        Self {
            name: name.into(),
            extensions: Cow::Owned(extensions),
        }
    }

    /// Convert to rfd-compatible format.
    #[inline]
    pub(crate) fn as_rfd_filter(&self) -> (&str, &[&str]) {
        (self.name.as_ref(), self.extensions.as_ref())
    }
}

//---------------------------------------------------------------------------------------------
// Pre-built filters for Prexager supported formats
//---------------------------------------------------------------------------------------------

impl FileFilter {
    /// COOL format: genomic contact matrices.
    #[inline]
    #[must_use]
    pub const fn cool() -> Self {
        Self::new_static("COOL files", &["cool"])
    }

    /// HIC format: Hi-C contact matrices.
    #[inline]
    #[must_use]
    pub const fn hic() -> Self {
        Self::new_static("HIC files", &["hic"])
    }

    /// CSV format: comma-separated values.
    #[inline]
    #[must_use]
    pub const fn csv() -> Self {
        Self::new_static("CSV files", &["csv"])
    }

    /// NPY format: NumPy binary arrays.
    #[inline]
    #[must_use]
    pub const fn npy() -> Self {
        Self::new_static("NPY files", &["npy"])
    }

    /// HDF5 format: hierarchical data.
    #[inline]
    #[must_use]
    pub const fn hdf5() -> Self {
        Self::new_static("HDF5 files", &["h5", "hdf5"])
    }

    /// Bincode format: Rust-native serialization.
    #[inline]
    #[must_use]
    pub const fn bincode() -> Self {
        Self::new_static("Bincode files", &["bin"])
    }

    /// All supported genomic input formats.
    #[inline]
    #[must_use]
    pub fn genomic_inputs() -> &'static [Self; 2] {
        static FILTERS: [FileFilter; 2] = [FileFilter::cool(), FileFilter::hic()];
        &FILTERS
    }

    /// All supported output/serialization formats.
    #[inline]
    #[must_use]
    pub fn serialization_outputs() -> &'static [Self; 4] {
        static FILTERS: [FileFilter; 4] = [
            FileFilter::csv(),
            FileFilter::npy(),
            FileFilter::hdf5(),
            FileFilter::bincode(),
        ];
        &FILTERS
    }

    /// All formats supported by Prexager (inputs + outputs).
    #[inline]
    #[must_use]
    pub fn all_supported() -> &'static [Self; 6] {
        static FILTERS: [FileFilter; 6] = [
            FileFilter::cool(),
            FileFilter::hic(),
            FileFilter::csv(),
            FileFilter::npy(),
            FileFilter::hdf5(),
            FileFilter::bincode(),
        ];
        &FILTERS
    }

    /// Fallback: show all files.
    #[inline]
    #[must_use]
    pub const fn all_files() -> Self {
        Self::new_static("All files", &["*"])
    }
}

//---------------------------------------------------------------------------------------------
// Convenience filter sets for common UI patterns
//---------------------------------------------------------------------------------------------

/// Filter set for picking an input genomic file (.cool or .hic).
#[inline]
#[must_use]
pub fn input_genomic_filters() -> &'static [FileFilter; 3] {
    static FILTERS: [FileFilter; 3] = [
        FileFilter::cool(),
        FileFilter::hic(),
        FileFilter::all_files(),
    ];
    &FILTERS
}

/// Filter set for picking an output serialization file.
#[inline]
#[must_use]
pub fn output_serialization_filters() -> &'static [FileFilter; 5] {
    static FILTERS: [FileFilter; 5] = [
        FileFilter::csv(),
        FileFilter::npy(),
        FileFilter::hdf5(),
        FileFilter::bincode(),
        FileFilter::all_files(),
    ];
    &FILTERS
}

/// Filter set for picking any Prexager compatible file.
#[inline]
#[must_use]
pub fn all_actual_filters() -> &'static [FileFilter; 7] {
    static FILTERS: [FileFilter; 7] = [
        FileFilter::cool(),
        FileFilter::hic(),
        FileFilter::csv(),
        FileFilter::npy(),
        FileFilter::hdf5(),
        FileFilter::bincode(),
        FileFilter::all_files(),
    ];
    &FILTERS
}
