// SPDX-FileCopyrightText: 2025 Mikhail Suvorau (Suvorrow) <suvoraum@gmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use dioxus::prelude::*;

#[component]
pub fn ToolsView() -> Element {
    rsx! {
        div { class: "view-container",
            h1 { "Combine gRNA Risk Analysis Tools" }
            p { "Perform safety assessments here" }
        }
    }
}
