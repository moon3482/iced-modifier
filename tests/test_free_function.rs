mod common;

use iced::Color;
use iced::widget::{text, tooltip};
use iced_modifier::prelude::*;

use common::{E, Msg};

#[test]
fn free_fn_with_modifier() {
    let _: E = iced_modifier::modify(
        text("hello"),
        Modifier::new().padding(10).background_color(Color::WHITE),
    );
}

#[test]
fn free_fn_with_interactor() {
    let _: E = iced_modifier::modify(text("hello"), Modifier::new().padding(10).on_press(Msg::A));
}

#[test]
fn free_fn_empty_modifier() {
    let _: E = iced_modifier::modify(text("hello"), Modifier::new());
}

#[test]
fn free_fn_with_all_features() {
    let _: E = iced_modifier::modify(
        text("hello"),
        Modifier::new()
            .padding(10)
            .background_color(Color::WHITE)
            .on_press(Msg::A)
            .tooltip_text("tip", tooltip::Position::Top)
            .scrollable(),
    );
}
