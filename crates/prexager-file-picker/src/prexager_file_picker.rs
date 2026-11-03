// SPDX-FileCopyrightText: 2025 Mikhail Suvorau (Suvorrow) <suvoraum@gmail.com>
// SPDX-License-Identifier: GPL-3.0-only

//! Native file dialog utilities for Prexager app.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs, rust_2018_idioms, unused_lifetimes)]
#![deny(unsafe_code)]

use std::path::PathBuf;

mod picker;
mod types;

pub use picker::{pick_directory, pick_file, pick_files, pick_save_file};
pub use types::{
    FileFilter, all_actual_filters, input_genomic_filters, output_serialization_filters,
};

/// Pick a single file using Prexager's default genomic input filters ('.cool', '.hic').
///
/// Equivalent to pick_file(input_genomic_filters())'.
#[inline]
pub async fn pick_genomic_input() -> Option<PathBuf> {
    pick_file(input_genomic_filters()).await
}

/// Pick a single file using Prexager's default serialization output filters.
///
/// Equivalent to 'pick_file(output_serialization_filters())'.
#[inline]
pub async fn pick_serialization_output() -> Option<PathBuf> {
    use rfd::AsyncFileDialog;

    let mut dialog = AsyncFileDialog::new()
        .set_title("Save Matrix Output")
        .set_file_name("output_matrix.csv");

    for filter in output_serialization_filters() {
        let (name, extensions) = filter.as_rfd_filter();
        dialog = dialog.add_filter(name, extensions);
    }

    dialog
        .save_file()
        .await
        .map(|handle| handle.path().to_path_buf())
}

/// Pick a single file using all Prexager supported formats.
///
/// Equivalent to 'pick_file(all_actual_filters())'.
#[inline]
pub async fn pick_any_prexager_file() -> Option<PathBuf> {
    pick_file(all_actual_filters()).await
}

/// Pick a directory for saving output files.
///
/// Equivalent to 'pick_directory()'.
#[inline]
pub async fn pick_output_directory() -> Option<PathBuf> {
    pick_directory().await
}
