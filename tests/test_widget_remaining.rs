mod common;

use iced::Color;
use iced_modifier::prelude::*;
use iced_modifier::column;

use common::{E, Msg};

// ═══════════ Radio ═══════════

#[test]
fn radio_basic() {
    let _: E = Radio::new("Option A", 1, Some(1), |v| Msg::A)
        .padding(8)
        .into();
}

#[test]
fn radio_with_styling() {
    let _: E = Radio::new("Option B", 2, Some(1), |v| Msg::A)
        .radio_size(20)
        .radio_spacing(8)
        .text_size(14)
        .padding(8)
        .background_color(Color::WHITE)
        .corner_radius(4)
        .into();
}

#[test]
fn radio_helper() {
    let _: E = radio("Choice", 1, Some(1), |v| Msg::A)
        .padding(4)
        .into();
}

#[test]
fn radio_in_column() {
    let _: E = column![
        Radio::new("A", 1, Some(1), |v| Msg::A).padding(4),
        Radio::new("B", 2, Some(1), |v| Msg::A).padding(4),
        Radio::new("C", 3, Some(1), |v| Msg::A).padding(4),
    ]
    .spacing(4)
    .into();
}

// ═══════════ Slider ═══════════

#[test]
fn slider_basic() {
    let _: E = Slider::new(0.0..=100.0, 50.0, |v| Msg::A)
        .padding(8)
        .into();
}

#[test]
fn slider_with_options() {
    let _: E = Slider::new(0..=100, 50, |v| Msg::A)
        .step(5)
        .on_release(Msg::B)
        .slider_width(200)
        .padding(8)
        .into();
}

#[test]
fn slider_helper() {
    let _: E = slider(0.0..=1.0, 0.5, |v| Msg::A)
        .padding(4)
        .into();
}

// ═══════════ TextEditor ═══════════

#[test]
fn text_editor_basic() {
    let content = iced::widget::text_editor::Content::<iced::Renderer>::new();
    let _: E = TextEditor::new(&content)
        .on_action(|_| Msg::A)
        .padding(8)
        .into();
}

#[test]
fn text_editor_with_font_size() {
    let content = iced::widget::text_editor::Content::<iced::Renderer>::new();
    let _: E = TextEditor::new(&content)
        .on_action(|_| Msg::A)
        .font_size(14)
        .padding(8)
        .corner_radius(4)
        .into();
}

#[test]
fn text_editor_helper() {
    let content = iced::widget::text_editor::Content::<iced::Renderer>::new();
    let _: E = text_editor(&content)
        .on_action(|_| Msg::A)
        .padding(4)
        .into();
}

// ═══════════ PickList ═══════════

#[test]
fn pick_list_basic() {
    let options = vec!["A", "B", "C"];
    let _: E = PickList::new(options, Some("A"), |v| Msg::A)
        .padding(8)
        .into();
}

#[test]
fn pick_list_with_options() {
    let options = vec!["One", "Two", "Three"];
    let _: E = PickList::new(options, None::<&str>, |v| Msg::A)
        .placeholder("Select...")
        .text_size(14)
        .list_padding(8)
        .padding(8)
        .corner_radius(4)
        .into();
}

// ═══════════ All in layout ═══════════

#[test]
fn all_remaining_in_column() {
    let content = iced::widget::text_editor::Content::<iced::Renderer>::new();
    let _: E = column![
        Radio::new("Radio", 1, Some(1), |v| Msg::A).padding(4),
        Slider::new(0.0..=100.0, 50.0, |v| Msg::A).padding(4),
        TextEditor::new(&content).on_action(|_| Msg::A).font_size(14).padding(4),
        PickList::new(vec!["A", "B"], Some("A"), |v| Msg::A).padding(4),
    ]
    .spacing(8)
    .padding(16)
    .into();
}
