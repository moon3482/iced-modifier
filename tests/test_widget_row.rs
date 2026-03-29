mod common;

use iced::{Color, mouse};
use iced_modifier::prelude::*;
use iced_modifier::{column, row};

use common::{E, Msg};

#[test]
fn row_with_push() {
    let _: E = Row::new().push(text("A")).push(text("B")).into();
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
    let _: E = row![text("A").font_size(14), text("B").font_size(18),]
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
    let _: E = row![text("A"), text("B")].width(50).scrollable_x().into();
}

// ── Edge cases ──

#[test]
fn row_push_after_styling() {
    let _: E = Row::new()
        .padding(10)
        .background_color(Color::WHITE)
        .push(text("A"))
        .push(text("B"))
        .into();
}

#[test]
fn row_with_interactive_children() {
    let _: E = row![
        Text::new("click").on_press(Msg::A).padding(4),
        text("plain").padding(4),
    ]
    .spacing(8)
    .into();
}

#[test]
fn row_deeply_nested() {
    let _: E = row![row![row![text("deep")].padding(2),].padding(4),]
        .padding(8)
        .into();
}

#[test]
fn row_in_column_with_interactions() {
    let _: E = column![
        row![text("A"), text("B")]
            .spacing(4)
            .on_press(Msg::A)
            .cursor(mouse::Interaction::Pointer),
    ]
    .spacing(8)
    .into();
}
