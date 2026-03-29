mod common;

use iced::{Color, mouse};
use iced_modifier::prelude::*;

use common::{E, Msg};

#[test]
fn button_basic() {
    let _: E = Button::new(text("Click")).on_press(Msg::A).into();
}

#[test]
fn button_helper() {
    let _: E = button(text("Click")).on_press(Msg::A).into();
}

#[test]
fn button_with_styling() {
    let _: E = Button::new(text("Click"))
        .on_press(Msg::A)
        .padding(12)
        .background_color(Color::WHITE)
        .corner_radius(8)
        .into();
}

#[test]
fn button_with_hover() {
    let _: E = Button::new(text("Hover"))
        .on_press(Msg::A)
        .on_enter(Msg::Hover(true))
        .on_exit(Msg::Hover(false))
        .cursor(mouse::Interaction::Pointer)
        .into();
}

#[test]
fn button_on_press_maybe_none() {
    let _: E = Button::new(text("Disabled"))
        .on_press_maybe(None::<Msg>)
        .padding(10)
        .into();
}

#[test]
fn button_native_padding() {
    let _: E = Button::new(text("Click"))
        .on_press(Msg::A)
        .button_padding(8)
        .padding(4) // Container padding
        .into();
}

#[test]
fn button_with_styled_text() {
    let _: E = Button::new(Text::new("Styled").font_size(16).color(Color::WHITE))
        .on_press(Msg::A)
        .padding(12)
        .background_color(Color::from_rgb(0.2, 0.4, 0.8))
        .corner_radius(8)
        .into();
}

#[test]
fn button_hidden() {
    let _: E = Button::new(text("Hidden"))
        .on_press(Msg::A)
        .hidden(true)
        .into();
}
