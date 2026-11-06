// SPDX-FileCopyrightText: 2025 Mikhail Suvorau (Suvorrow) <suvoraum@gmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use dioxus::prelude::*;

#[component]
pub fn MainContent() -> Element {
    let mut target = use_signal(|| GenomicTarget {
        chromosome: "chr5".to_string(),
        position: 1_234_567,
        sequence: "ATCG".to_string(),
    });

    let risc_score = use_memo(move || compute_topological_risk(&target()));

    rsx! {
        div { class: "main-content",
            h2 { "Genomic Risk Analysis" }
            div { class: "input-selection",
                label { "Chromosome:" }
                input {
                    value: "{target().chromosome}",
                    oninput: move |evt| {
                        let mut t = target.write();
                        t.chromosome = evt.value.clone();
                    },
                }
            }
            div { class: "result-section",
                h3 { "Topological Risc Score" }
                p { "Score: {risk_score()}" }
            }
        }
    }
}
