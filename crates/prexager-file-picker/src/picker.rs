// SPDX-FileCopyrightText: 2025 Mikhail Suvorau (Suvorrow) <suvoraum@gmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use rfd::AsyncFileDialog;
use std::path::PathBuf;

use crate::FileFilter;

/// Pick a single file with optional filters.
///
/// Returns 'None' if the user cancels the dialog.
#[inline]
pub async fn pick_file(filters: &[FileFilter]) -> Option<PathBuf> {
    let mut dialog = AsyncFileDialog::new();

    for filter in filters {
        let (name, exts) = filter.as_rfd_filter();
        dialog = dialog.add_filter(name, exts);
    }

    dialog
        .pick_file()
        .await
        .map(|handle| handle.path().to_path_buf())
}

/// Pick a single directory.
#[inline]
pub async fn pick_directory() -> Option<PathBuf> {
    AsyncFileDialog::new()
        .pick_folder()
        .await
        .map(|handle| handle.path().to_path_buf())
}

/// Pick multiple files with optional filters.
#[inline]
pub async fn pick_files(filters: &[FileFilter]) -> Option<Vec<PathBuf>> {
    let mut dialog = AsyncFileDialog::new();

    for filter in filters {
        let (name, exts) = filter.as_rfd_filter();
        dialog = dialog.add_filter(name, exts);
    }

    let handles = dialog.pick_files().await?;
    Some(
        handles
            .into_iter()
            .map(|h| h.path().to_path_buf())
            .collect(),
    )
}

/// Pick a file to save (with optional default name and filters).
///
/// Returns 'None' if the user cancels.
#[inline]
pub async fn pick_save_file(default_name: Option<&str>, filters: &[FileFilter]) -> Option<PathBuf> {
    let mut dialog = AsyncFileDialog::new();

    if let Some(name) = default_name {
        dialog = dialog.set_file_name(name);
    }

    for filter in filters {
        let (name, exts) = filter.as_rfd_filter();
        dialog = dialog.add_filter(name, exts);
    }

    dialog
        .save_file()
        .await
        .map(|handle| handle.path().to_path_buf())
}
