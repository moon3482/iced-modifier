mod common;

use iced::widget::{column, row, text, tooltip};
use iced::Color;
use iced_modifier::prelude::*;

use common::{E, Msg};

#[test]
fn scrollable_vertical() {
    let _: E = column![text("a")].modify(Modifier::new().height(100).scrollable());
}

#[test]
fn scrollable_horizontal() {
    let _: E = row![text("a")].modify(Modifier::new().width(100).scrollable_x());
}

#[test]
fn scrollable_both() {
    let _: E = text("a").modify(Modifier::new().width(100).height(100).scrollable_xy());
}

#[test]
fn scrollable_direction_overwrite() {
    let _: E = text("a").modify(Modifier::new().scrollable().scrollable_x());
}

#[test]
fn scrollable_with_interaction() {
    let _: E = text("a").modify(Modifier::new().height(100).on_press(Msg::A).scrollable());
}

#[test]
fn scrollable_with_tooltip() {
    let _: E = text("a").modify(
        Modifier::new().height(100).tooltip_text("scroll", tooltip::Position::Top).scrollable(),
    );
}

#[test]
fn scrollable_with_styling() {
    let _: E = column![text("a"), text("b")].modify(
        Modifier::new().height(100).padding(8).background_color(Color::WHITE).corner_radius(6).scrollable(),
    );
}

#[test]
fn scrollable_no_size_constraint() {
    let _: E = column![text("a")].modify(Modifier::new().scrollable());
}
