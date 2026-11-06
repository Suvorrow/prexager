// SPDX-FileCopyrightText: 2025 Mikhail Suvorau (Suvorrow) <suvoraum@gmail.com>
// SPDX-License-Identifier: GPL-3.0-only

mod prexager_app;

use crate::prexager_app::App;
use anyhow::{Context, Result};
use dioxus::LaunchBuilder;
use dioxus::desktop::{Config, LogicalSize, WindowBuilder};

//---------------------------------------------------------------------------------------------
// Constants
//---------------------------------------------------------------------------------------------
const APP_TITLE: &str = "Prexager";
const WINDOW_WIDTH: f64 = 1536.0;
const WINDOW_HEIGHT: f64 = 864.0;

// ---- Main Entry Point ----
fn main() -> Result<()> {
    run()
}

fn run() -> Result<()> {
    init_logging().context("Failed to initialize logging subsystem")?;
    setup().context("Failed to initialize application UI")?;

    Ok(())
}

// ---- Initialization Functions ----
fn init_logging() -> Result<()> {
    // TODO: For production build configuring levels, output format, or file targets.
    tracing_subscriber::fmt::init();
    Ok(())
}

// ---- Application Setup ----
fn setup() -> Result<()> {
    // Get the embedded CSS content from the ui crate
    let css_content = prexager_ui::get_compiled_css();

    // Configure the desktop window
    let window = WindowBuilder::new()
        .with_title(APP_TITLE)
        .with_inner_size(LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
        .with_decorations(false)
        .with_transparent(true)
        .with_minimizable(true)
        .with_maximizable(true)
        .with_resizable(true);

    let config = Config::default()
        .with_window(window)
        .with_custom_head(format!(
            r#"
        <title>{}</title>
        <style>{}</style>
        "#,
            APP_TITLE, css_content
        ));

    LaunchBuilder::new().with_cfg(config).launch(App);

    Ok(())
}
