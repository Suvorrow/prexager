// SPDX-FileCopyrightText: 2025 Mikhail Suvorau (Suvorrow) <suvoraum@gmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use dioxus::prelude::*;

#[component]
pub fn HomeView() -> Element {
    rsx! {
        div { class: "view-container",
            h1 { "Welcome to Prexager" }
            p { "The platform that lets interactively explore 3D genome architecture from whole-nucleus
                chromosome territories down to individual molecular structures in real time."
            }
        }
    }
}
