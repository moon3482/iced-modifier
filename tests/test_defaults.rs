mod common;

use iced::widget::text;
use iced::Color;
use iced_modifier::prelude::*;

use common::{E, Msg};

#[test]
fn empty_modifier_passthrough() {
    let _: E = text("hello").modify(Modifier::new());
}

#[test]
fn default_modifier_passthrough() {
    let _: E = text("hello").modify(Modifier::default());
}

#[test]
fn layer_on_empty_current_is_noop() {
    let _: E = text("hello").modify(Modifier::new().layer());
}

#[test]
fn layer_consecutive_calls_no_crash() {
    let _: E = text("hello").modify(
        Modifier::new().padding(10).layer().layer().layer(),
    );
}

#[test]
fn then_with_empty_modifier() {
    let base = Modifier::new().padding(10).background_color(Color::WHITE);
    let _: E = text("hello").modify(base.then(Modifier::new()));
}

#[test]
fn then_empty_with_modifier() {
    let _: E = text("hello").modify(Modifier::new().then(Modifier::new().padding(10)));
}
