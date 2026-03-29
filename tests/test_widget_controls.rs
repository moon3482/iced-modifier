mod common;

use iced::Color;
use iced_modifier::prelude::*;
use iced_modifier::column;

use common::{E, Msg};

// ═══════════ TextInput ═══════════

#[test]
fn text_input_basic() {
    let _: E = TextInput::new("placeholder", "value")
        .on_input(|s| Msg::A)
        .into();
}

#[test]
fn text_input_helper() {
    let _: E = text_input("placeholder", "value")
        .on_input(|s| Msg::A)
        .into();
}

#[test]
fn text_input_with_font_size() {
    let _: E = TextInput::new("Search...", "")
        .on_input(|s| Msg::A)
        .font_size(16)
        .padding(12)
        .corner_radius(8)
        .into();
}

#[test]
fn text_input_with_submit() {
    let _: E = TextInput::new("Enter", "text")
        .on_input(|s| Msg::A)
        .on_submit(Msg::B)
        .into();
}

#[test]
fn text_input_secure() {
    let _: E = TextInput::new("Password", "")
        .on_input(|s| Msg::A)
        .secure(true)
        .padding(8)
        .into();
}

#[test]
fn text_input_native_padding() {
    let _: E = TextInput::new("ph", "val")
        .on_input(|s| Msg::A)
        .input_padding(8)  // iced native
        .padding(4)        // Container
        .into();
}

#[test]
fn text_input_hidden() {
    let _: E = TextInput::new("ph", "val")
        .on_input(|s| Msg::A)
        .hidden(true)
        .into();
}

// ═══════════ Checkbox ═══════════

#[test]
fn checkbox_basic() {
    let _: E = Checkbox::new(true)
        .label("Accept terms")
        .on_toggle(|b| Msg::A)
        .into();
}

#[test]
fn checkbox_helper() {
    let _: E = checkbox(false)
        .label("Option")
        .on_toggle(|b| Msg::A)
        .into();
}

#[test]
fn checkbox_with_styling() {
    let _: E = Checkbox::new(true)
        .label("Styled")
        .on_toggle(|b| Msg::A)
        .check_size(20)
        .check_spacing(8)
        .text_size(14)
        .padding(8)
        .background_color(Color::WHITE)
        .corner_radius(4)
        .into();
}

#[test]
fn checkbox_hidden() {
    let _: E = Checkbox::new(false)
        .label("Hidden")
        .on_toggle(|b| Msg::A)
        .hidden(true)
        .into();
}

// ═══════════ Toggler ═══════════

#[test]
fn toggler_basic() {
    let _: E = Toggler::new(false)
        .label("Dark mode")
        .on_toggle(|b| Msg::A)
        .into();
}

#[test]
fn toggler_helper() {
    let _: E = toggler(true)
        .label("Notifications")
        .on_toggle(|b| Msg::A)
        .into();
}

#[test]
fn toggler_with_styling() {
    let _: E = Toggler::new(true)
        .label("Toggle")
        .on_toggle(|b| Msg::A)
        .toggler_size(24)
        .toggler_spacing(10)
        .text_size(14)
        .padding(8)
        .into();
}

#[test]
fn toggler_hidden() {
    let _: E = Toggler::new(false)
        .on_toggle(|b| Msg::A)
        .hidden(true)
        .into();
}

// ═══════════ In layout ═══════════

#[test]
fn controls_in_column() {
    let _: E = column![
        TextInput::new("Name", "").on_input(|s| Msg::A).font_size(14).padding(8),
        Checkbox::new(true).label("Agree").on_toggle(|b| Msg::A).padding(4),
        Toggler::new(false).label("Enable").on_toggle(|b| Msg::A).padding(4),
        Button::new(text("Submit")).on_press(Msg::B).padding(12).corner_radius(8),
    ]
    .spacing(8)
    .padding(16)
    .into();
}
