// SPDX-FileCopyrightText: 2025 Mikhail Suvorau (Suvorrow) <suvoraum@gmail.com>
// SPDX-License-Identifier: GPL-3.0-only

//! Prexager: Topologal-Informational Validator for Genomic Instability
//!
//! Core mathematical framework for assessing gRNA safety through
//! topological and information-theoretic, and chaos-theoretic biomarkers.

use dioxus::prelude::*;
use dioxus_router::Router;
use ui::routes::Route;

// Define the root router
#[component]
pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
