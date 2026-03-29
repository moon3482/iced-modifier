mod common;

use iced::Color;
use iced::widget::{text, tooltip};
use iced_modifier::prelude::*;

use common::{E, Msg};

#[test]
fn hidden_true_ignores_styling() {
    let _: E = text("hello").modify(
        Modifier::new()
            .padding(10)
            .background_color(Color::WHITE)
            .corner_radius(8)
            .hidden(true),
    );
}

#[test]
fn hidden_true_with_interactions() {
    let _: E = text("hello").modify(Modifier::new().on_press(Msg::A).hidden(true));
}

#[test]
fn hidden_toggle_last_wins_false() {
    let _: E = text("hello").modify(Modifier::new().hidden(true).hidden(false).padding(10));
}

#[test]
fn hidden_toggle_last_wins_true() {
    let _: E = text("hello").modify(Modifier::new().hidden(false).hidden(true));
}

#[test]
fn hidden_with_tooltip() {
    let _: E = text("hello").modify(
        Modifier::new()
            .tooltip_text("tip", tooltip::Position::Top)
            .hidden(true),
    );
}

#[test]
fn hidden_with_scrollable() {
    let _: E = text("hello").modify(Modifier::new().scrollable().hidden(true));
}
