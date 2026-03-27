mod common;

use iced::widget::{column, text, tooltip};
use iced::{mouse, Color};
use iced_modifier::prelude::*;

use common::{E, Msg};

#[test]
fn cursor_only_no_click() {
    let _: E = text("hello").modify(Modifier::new().cursor::<Msg>(mouse::Interaction::Pointer));
}

#[test]
fn on_enter_exit_without_press() {
    let _: E = text("hello").modify(
        Modifier::new().on_enter(Msg::Hover(true)).on_exit(Msg::Hover(false)),
    );
}

#[test]
fn all_interactions_set() {
    let _: E = text("hello").modify(
        Modifier::new()
            .on_press(Msg::A).on_release(Msg::A).on_double_click(Msg::A)
            .on_right_press(Msg::A).on_right_release(Msg::A)
            .on_middle_press(Msg::A).on_middle_release(Msg::A)
            .on_enter(Msg::A).on_exit(Msg::A)
            .cursor(mouse::Interaction::Pointer),
    );
}

#[test]
fn interactor_styling_before_interaction() {
    let _: E = text("hello").modify(
        Modifier::new().padding(10).background_color(Color::WHITE).corner_radius(8).on_press(Msg::A),
    );
}

#[test]
fn interactor_styling_after_interaction() {
    let _: E = text("hello").modify(
        Modifier::new().on_press(Msg::A).padding(10).background_color(Color::WHITE).corner_radius(8),
    );
}

#[test]
fn interactor_styling_mixed_with_interactions() {
    let _: E = text("hello").modify(
        Modifier::new()
            .padding(10).on_press(Msg::A)
            .background_color(Color::WHITE).on_enter(Msg::Hover(true))
            .corner_radius(8).on_exit(Msg::Hover(false)),
    );
}

#[test]
fn interactor_with_layer() {
    let _: E = text("hello").modify(
        Modifier::new()
            .padding(10).background_color(Color::from_rgb(1.0, 0.0, 0.0))
            .layer()
            .background_color(Color::from_rgb(0.0, 0.0, 1.0))
            .on_press(Msg::A),
    );
}

#[test]
fn interactor_with_multiple_layers() {
    let _: E = text("hello").modify(
        Modifier::new().padding(5).layer().padding(10).layer().background_color(Color::WHITE).on_press(Msg::A),
    );
}

#[test]
fn interactor_with_tooltip() {
    let _: E = text("hello").modify(
        Modifier::new().padding(8).on_press(Msg::A).tooltip_text("Click me", tooltip::Position::Top),
    );
}

#[test]
fn interactor_with_scrollable() {
    let _: E = column![text("a"), text("b")].modify(
        Modifier::new().height(100).on_press(Msg::A).scrollable(),
    );
}

#[test]
fn interactor_with_tooltip_and_scrollable() {
    let _: E = column![text("a"), text("b")].modify(
        Modifier::new().height(100).on_press(Msg::A).tooltip_text("info", tooltip::Position::Bottom).scrollable(),
    );
}

#[test]
fn interactor_with_id() {
    let _: E = text("hello").modify(Modifier::new().id("clickable").on_press(Msg::A));
}

#[test]
fn interactor_overwrite_on_press() {
    let _: E = text("hello").modify(Modifier::new().on_press(Msg::A).on_press(Msg::B));
}
