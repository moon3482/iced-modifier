mod common;

use iced::widget::tooltip;
use iced::Color;
use iced_modifier::prelude::*;

use common::{E, Msg};

// ── Basic usage ──

#[test]
fn text_with_font_size() {
    let _: E = text("hello").modify(Modifier::new().font_size(16));
}

#[test]
fn text_with_font_size_and_styling() {
    let _: E = text("hello").modify(
        Modifier::new()
            .font_size(20)
            .padding(12)
            .background_color(Color::WHITE)
            .corner_radius(8),
    );
}

#[test]
fn text_without_font_size() {
    // font_size not set — should work like regular text
    let _: E = text("hello").modify(Modifier::new().padding(10));
}

#[test]
fn text_with_size_and_font_size() {
    // .size() on widget + font_size in modifier — font_size should override
    let _: E = text("hello").size(12).modify(Modifier::new().font_size(20));
}

// ── Text methods forwarding ──

#[test]
fn text_with_size_method() {
    let _: E = text("hello").size(16).modify(Modifier::new().padding(10));
}

#[test]
fn text_with_color() {
    let _: E = text("hello").color(Color::from_rgb(1.0, 0.0, 0.0)).modify(Modifier::new().padding(10));
}

#[test]
fn text_with_alignment() {
    let _: E = text("hello").center().modify(Modifier::new().padding(10));
}

// ── Interactions ──

#[test]
fn text_with_font_size_and_interactions() {
    let _: E = text("hello").modify(
        Modifier::new()
            .font_size(16)
            .padding(10)
            .on_press(Msg::A),
    );
}

#[test]
fn text_with_font_size_and_tooltip() {
    let _: E = text("hello").modify(
        Modifier::new()
            .font_size(14)
            .tooltip_text("tip", tooltip::Position::Top),
    );
}

#[test]
fn text_with_font_size_and_scrollable() {
    let _: E = text("hello").modify(
        Modifier::new()
            .font_size(14)
            .height(50)
            .scrollable(),
    );
}

// ── Edge cases ──

#[test]
fn text_font_size_zero() {
    let _: E = text("hello").modify(Modifier::new().font_size(0));
}

#[test]
fn text_font_size_large() {
    let _: E = text("hello").modify(Modifier::new().font_size(200));
}

#[test]
fn text_empty_modifier() {
    let _: E = text("hello").modify(Modifier::new());
}

#[test]
fn text_hidden_with_font_size() {
    let _: E = text("hello").modify(Modifier::new().font_size(16).hidden(true));
}

#[test]
fn text_font_size_overwrite() {
    // Last font_size wins
    let _: E = text("hello").modify(
        Modifier::new().font_size(12).font_size(24),
    );
}

// ── Into<Element> without modify ──

#[test]
fn text_into_element_without_modify() {
    let _: E = text("hello").into();
}

#[test]
fn text_with_size_into_element() {
    let _: E = text("hello").size(16).into();
}

// ── Conditional ──

#[test]
fn text_font_size_with_modify_if() {
    let _: E = text("hello").modify(
        Modifier::new()
            .font_size(14)
            .modify_if(true, |m| m.font_size(20).background_color(Color::WHITE)),
    );
}
