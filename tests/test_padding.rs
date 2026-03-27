mod common;

use iced::widget::text;
use iced_modifier::prelude::*;

use common::E;

#[test]
fn padding_x_then_padding_y_independent_axes() {
    let _: E = text("hello").modify(Modifier::new().padding_x(10.0).padding_y(5.0));
}

#[test]
fn padding_y_then_padding_x_independent_axes() {
    let _: E = text("hello").modify(Modifier::new().padding_y(5.0).padding_x(10.0));
}

#[test]
fn padding_then_padding_x_partial_override() {
    let _: E = text("hello").modify(Modifier::new().padding(20).padding_x(10.0));
}

#[test]
fn padding_then_padding_y_partial_override() {
    let _: E = text("hello").modify(Modifier::new().padding(20).padding_y(5.0));
}

#[test]
fn padding_x_zero() {
    let _: E = text("hello").modify(Modifier::new().padding_x(0.0));
}

#[test]
fn padding_y_zero() {
    let _: E = text("hello").modify(Modifier::new().padding_y(0.0));
}
