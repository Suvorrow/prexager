// SPDX-FileCopyrightText: 2025 Mikhail Suvorau (Suvorrow) <suvoraum@gmail.com>
// SPDX-License-Identifier: GPL-3.0-only

pub mod data;
pub mod dropdown;
pub mod home;
pub mod settings;
pub mod tools;

// Re-export components for access from other modules
pub use data::DataView;
pub use dropdown::{DropdownItem, DropdownPanel, dropdown_header, dropdown_item};
pub use home::HomeView;
pub use settings::SettingsView;
pub use tools::ToolsView;
