// SPDX-FileCopyrightText: 2025 Mikhail Suvorau (Suvorrow) <suvoraum@gmail.com>
// SPDX-License-Identifier: GPL-3.0-only

pub mod components;
pub mod routes;
pub mod state;

const COMPILED_CSS: &str = include_str!("../assets/compiled_styles.css");

pub fn get_compiled_css() -> &'static str {
    COMPILED_CSS
}

pub use components::{DataView, HomeView, SettingsView, ToolsView};
pub use routes::Route;
pub use state::{ActiveToolType, DropdownType, UiState};
