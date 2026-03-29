mod common;

use iced::widget::tooltip;
use iced::{mouse, Color};
use iced_modifier::prelude::*;
use iced_modifier::column;

use common::{E, Msg};

// ── Direct chaining ──

#[test]
fn column_with_push() {
    let _: E = Column::new()
        .push(text("A"))
        .push(text("B"))
        .into();
}

#[test]
fn column_with_spacing_and_styling() {
    let _: E = Column::new()
        .push(text("A"))
        .push(text("B"))
        .spacing(8)
        .padding(12)
        .background_color(Color::WHITE)
        .corner_radius(8)
        .into();
}

#[test]
fn column_macro() {
    let _: E = column![
        text("A").font_size(14),
        text("B").font_size(18),
    ]
    .spacing(8)
    .padding(12)
    .into();
}

#[test]
fn column_macro_empty() {
    let _: E = column![].into();
}

#[test]
fn column_with_children() {
    let children: Vec<E> = vec![
        text("A").into(),
        text("B").into(),
    ];
    let _: E = Column::with_children(children)
        .spacing(4)
        .padding(8)
        .into();
}

// ── Interactions ──

#[test]
fn column_on_press() {
    let _: E = column![text("click")]
        .on_press(Msg::A)
        .padding(10)
        .into();
}

#[test]
fn column_multiple_interactions() {
    let _: E = column![text("multi")]
        .on_press(Msg::A)
        .on_enter(Msg::Hover(true))
        .on_exit(Msg::Hover(false))
        .cursor(mouse::Interaction::Pointer)
        .into();
}

// ── ModifyBase methods ──

#[test]
fn column_layer() {
    let _: E = column![text("A")]
        .padding(10)
        .background_color(Color::WHITE)
        .layer()
        .padding(5)
        .background_color(Color::BLACK)
        .into();
}

#[test]
fn column_tooltip() {
    let _: E = column![text("A")]
        .padding(10)
        .tooltip_text("info", tooltip::Position::Top)
        .into();
}

#[test]
fn column_scrollable() {
    let _: E = column![text("A"), text("B"), text("C")]
        .height(50)
        .scrollable()
        .scroll_anchor_bottom()
        .into();
}

#[test]
fn column_hidden() {
    let _: E = column![text("A")]
        .spacing(8)
        .hidden(true)
        .into();
}

#[test]
fn column_conditional() {
    let _: E = column![text("A")]
        .padding(10)
        .modify_if(true, |c| c.background_color(Color::WHITE))
        .into();
}

// ── Edge cases ──

#[test]
fn column_no_children() {
    let _: E = Column::new()
        .padding(10)
        .background_color(Color::WHITE)
        .into();
}

#[test]
fn column_spacing_zero() {
    let _: E = column![text("A"), text("B")]
        .spacing(0)
        .into();
}

#[test]
fn column_nested() {
    let _: E = column![
        column![text("A"), text("B")].spacing(4).padding(4),
        column![text("C"), text("D")].spacing(4).padding(4),
    ]
    .spacing(12)
    .into();
}
