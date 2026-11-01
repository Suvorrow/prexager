// SPDX-FileCopyrightText: 2025 Mikhail Suvorau (Suvorrow) <suvoraum@gmail.com>
// SPDX-License-Identifier: GPL-3.0-only

#![allow(non_snake_case)]
use crate::components::dropdown::{DropdownPanel, dropdown_header, dropdown_item};
use crate::components::{DataView, HomeView, SettingsView, ToolsView};
use crate::state::{ActiveToolType, DropdownType, UiState};
use dioxus::desktop::{use_window, window};
use dioxus::prelude::*;
use dioxus_router::{Outlet, Routable};

#[derive(Clone, Copy, PartialEq)]
enum ActiveNavButton {
    Home,
    Tools,
    Data,
    Settings,
    //Go,
    //Run,
    //Terminal,
    //Help,
}

#[component]
pub fn AppLayout() -> Element {
    let drg_window = use_window();
    let mut ui_state = use_signal(|| UiState::None);
    let navigator = use_navigator();

    let mut is_maximized = use_signal(|| false);

    let mut active_nav = use_signal(|| ActiveNavButton::Home);

    // Close any open dropdown
    let mut close_dropdown = move || {
        if matches!(ui_state(), UiState::DropdownOpen(_)) {
            ui_state.set(UiState::None);
        }
    };

    // Click handler for container: close dropdown if clicking outside
    let handle_container_click = move |_| {
        close_dropdown();
    };

    // Generic handler for opening a specific dropdown type
    let mut toggle_dropdown = move |dropdown_type: DropdownType| {
        match ui_state() {
            UiState::DropdownOpen(current) if current == dropdown_type => {
                ui_state.set(UiState::None); // Close if already open
            }
            _ => {
                ui_state.set(UiState::DropdownOpen(dropdown_type));
            }
        }
    };

    let tools_dropdown_items = vec![
        dropdown_item(
            "Parser",
            EventHandler::new(move |_| {
                ui_state.set(UiState::ActiveTool(ActiveToolType::Parser));
            }),
        ),
        dropdown_header(" ---------------- "),
        dropdown_item(
            "Settings",
            EventHandler::new(move |_| {
                navigator.push(Route::Settings {});
            }),
        ),
    ];

    rsx! {
        div { class: "app-container", onclick: handle_container_click,
            // Сustom navigation bar
            div {
                class: "top-nav-bar",
                onmousedown: move |_| {
                    let _ = drg_window.drag_window();
                },
                div { class: "nav-spacer" }
                div { class: "button-container",
                    button {
                        id: "home-button",
                        class: if active_nav() == ActiveNavButton::Home { "nav-btn active" } else { "nav-btn" },
                        onclick: move |_| {
                            active_nav.set(ActiveNavButton::Home);
                            navigator.push(Route::Home {});
                        },
                        onmousedown: move |evt| evt.stop_propagation(),
                        "Home"
                    }

                    // Tools button with dropdown anchor
                    div { class: "dropdown-anchor",
                        button {
                            id: "tools-button",
                            class: if active_nav() == ActiveNavButton::Tools { "nav-btn active" } else { "nav-btn" },
                            onclick: move |evt| {
                                evt.stop_propagation();
                                active_nav.set(ActiveNavButton::Tools);
                                toggle_dropdown(DropdownType::Tools);
                            },
                            onmousedown: move |evt| evt.stop_propagation(),
                            onmouseenter: move |_| {},
                            "Tools"
                        }
                        if matches!(ui_state(), UiState::DropdownOpen(DropdownType::Tools)) {
                            DropdownPanel {
                                items: tools_dropdown_items.clone(),
                                is_open: true,
                                on_close: move |_| close_dropdown(),
                                class: "tools-dropdown",
                            }
                        }
                    }

                    button {
                        id: "data-button",
                        class: if active_nav() == ActiveNavButton::Data { "nav-btn active" } else { "nav-btn" },
                        onclick: move |_| {
                            active_nav.set(ActiveNavButton::Data);
                            navigator.push(Route::Data {});
                        },
                        onmousedown: move |evt| evt.stop_propagation(),
                        "Data"
                    }
                    button {
                        id: "settings-button",
                        class: if active_nav() == ActiveNavButton::Settings { "nav-btn active" } else { "nav-btn" },
                        onclick: move |_| {
                            active_nav.set(ActiveNavButton::Settings);
                            navigator.push(Route::Settings {});
                        },
                        onmousedown: move |evt| evt.stop_propagation(),
                        "Settings"
                    }
                    button { class: "nav-btn", onclick: move |_| {}, "Go" }
                    button { class: "nav-btn", onclick: move |_| {}, "Run" }
                    button { class: "nav-btn", onclick: move |_| {}, "Terminal" }
                    button { class: "nav-btn", onclick: move |_| {}, "Help" }
                }
                // Right side - window controls
                div { class: "window-controls",
                    button {
                        class: "window-btn minimize-btn",
                        onclick: move |_| window().set_minimized(true),
                        onmousedown: move |evt| evt.stop_propagation(),
                        "▭"
                    }
                    // Maximize/Restore button
                    button {
                        class: if is_maximized() { "window-btn maximize-btn maximized" } else { "window-btn maximize-btn" },
                        onclick: move |_| {
                            is_maximized.set(!is_maximized());
                            window().toggle_maximized();
                        },
                        onmousedown: move |evt| evt.stop_propagation(),
                        "△"
                    }
                    // Close button
                    button {
                        class: "window-btn close-btn",
                        onclick: move |_| window().close(),
                        onmousedown: move |evt| evt.stop_propagation(),
                        "✕"
                    }
                }
            }

            // Content area - Render based on the overall UI state (ui_state)
            div { class: "content-area",
                // Check the overall UI state first
                Outlet::<Route> {}
            }
        }
    }
}

#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppLayout)]
        #[route("/")]
        Home {},

        #[route("/tools")]
        Tools {},

        #[route("/data")]
        Data {},

        #[route("/settings")]
        Settings {},
}

#[component]
fn Home() -> Element {
    rsx! {
        HomeView {}

    }
}
#[component]
fn Tools() -> Element {
    rsx! {
        ToolsView {}
    }
}
#[component]
fn Data() -> Element {
    rsx! {
        DataView {}
    }
}
#[component]
fn Settings() -> Element {
    rsx! {
        SettingsView {}
    }
}
