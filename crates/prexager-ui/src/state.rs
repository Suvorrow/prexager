// SPDX-FileCopyrightText: 2025 Mikhail Suvorau (Suvorrow) <suvoraum@gmail.com>
// SPDX-License-Identifier: GPL-3.0-only

#[derive(Clone, PartialEq, Debug)]
pub enum UiState {
    None,
    DropdownOpen(DropdownType),
    ActiveTool(ActiveToolType),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DropdownType {
    Tools,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ActiveToolType {
    Parser,
}
