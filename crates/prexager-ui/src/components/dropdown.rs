// SPDX-FileCopyrightText: 2025 Mikhail Suvorau (Suvorrow) <suvoraum@gmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use dioxus::prelude::*;

/// Represents a single item in a dropdown menu
#[derive(Clone, PartialEq)]
pub struct DropdownItem {
    pub label: String,
    pub action: Option<EventHandler<()>>,
    pub enabled: bool,
}

/// Props for the reusable DropdownPanel component
#[derive(Props, Clone, PartialEq)]
pub struct DropdownPanelProps {
    /// The menu items to display
    pub items: Vec<DropdownItem>,

    /// Callback when an item is selected (optional, items can have their own actions)
    #[props(default)]
    pub on_select: Option<EventHandler<DropdownItem>>,

    /// Whether the dropdown is currently visible
    pub is_open: bool,

    /// Callback to request closing the dropdown (e.g., on outside click)
    #[props(default)]
    pub on_close: EventHandler<()>,

    /// Optional CSS class for custom styling
    #[props(default)]
    pub class: Option<String>,

    /// Optional ID for testing/automation
    #[props(default)]
    pub id: Option<String>,
}

/// Dropdown panel component
#[component]
pub fn DropdownPanel(props: DropdownPanelProps) -> Element {
    if !props.is_open {
        return rsx! {};
    }

    let class = props.class.unwrap_or_default();
    let id = props.id.unwrap_or_default();

    let items = props.items.clone();

    rsx! {
        div {
            id: "{id}",
            class: "dropdown-panel {class}",
            tabindex: "-1",
            role: "menu",
            //Close on blur (click outside)
            onblur: move |_| props.on_close.call(()),
            // Prevent closing when hovering over the panel
            onmouseenter: |_| {},
            onmouseleave: |_| {},
            onmousedown: move |evt| evt.stop_propagation(),

            // Render each menu item
            for item in items {
                button {
                    class: "dropdown-item nav-btn",
                    role: "menuitem",
                    disabled: !item.enabled,
                    onclick: move |evt| {
                        evt.stop_propagation();
                        if let Some(action) = &item.action {
                            action.call(());
                        }
                        // Call general on_select callback
                        if let Some(on_select) = &props.on_select {
                            on_select.call(item.clone());
                        }
                        // Always close after selection
                        props.on_close.call(());
                    },
                    "{item.label}"
                }
            }
        }
    }
}

/// Helper function to create a simple enabled item with an action
pub fn dropdown_item(label: impl Into<String>, action: EventHandler<()>) -> DropdownItem {
    let label = label.into();
    DropdownItem {
        label,
        action: Some(action),
        enabled: true,
    }
}

pub fn dropdown_header(label: impl Into<String>) -> DropdownItem {
    DropdownItem {
        label: label.into(),
        action: None,
        enabled: false,
    }
}
