// SPDX-FileCopyrightText: 2025 Mikhail Suvorau (Suvorrow) <suvoraum@gmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use dioxus::prelude::*;

#[component]
pub fn SettingsView() -> Element {
    rsx! {
        div { class: "view-container",
            h1 { "Configure application settings." }
            p { "Configure your preferences" }
        }
    }
}
