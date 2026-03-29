mod common;

use iced::{mouse, Color};
use iced_modifier::prelude::*;
use iced_modifier::{column, row};

use common::{E, Msg};

#[test]
fn row_with_push() {
    let _: E = Row::new()
        .push(text("A"))
        .push(text("B"))
        .into();
}

#[test]
fn row_with_spacing_and_styling() {
    let _: E = Row::new()
        .push(text("A"))
        .push(text("B"))
        .spacing(8)
        .padding(12)
        .background_color(Color::WHITE)
        .into();
}

#[test]
fn row_macro() {
    let _: E = row![
        text("A").font_size(14),
        text("B").font_size(18),
    ]
    .spacing(8)
    .padding(12)
    .into();
}

#[test]
fn row_macro_empty() {
    let _: E = row![].into();
}

#[test]
fn row_on_press() {
    let _: E = row![text("click")]
        .on_press(Msg::A)
        .padding(10)
        .cursor(mouse::Interaction::Pointer)
        .into();
}

#[test]
fn row_nested_in_column() {
    let _: E = column![
        row![text("A"), text("B")].spacing(8),
        row![text("C"), text("D")].spacing(8),
    ]
    .spacing(12)
    .into();
}

#[test]
fn row_hidden() {
    let _: E = row![text("A")].hidden(true).into();
}

#[test]
fn row_scrollable() {
    let _: E = row![text("A"), text("B")]
        .width(50)
        .scrollable_x()
        .into();
}
