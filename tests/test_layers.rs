mod common;

use iced::widget::text;
use iced::{Color, Padding};
use iced_modifier::prelude::*;

use common::E;

#[test]
fn three_layers_deeply_nested() {
    let _: E = text("hello").modify(
        Modifier::new()
            .padding(5)
            .background_color(Color::from_rgb(1.0, 0.0, 0.0))
            .layer()
            .padding(10)
            .background_color(Color::from_rgb(0.0, 1.0, 0.0))
            .layer()
            .padding(15)
            .background_color(Color::from_rgb(0.0, 0.0, 1.0)),
    );
}

#[test]
fn four_layers() {
    let _: E = text("hello").modify(
        Modifier::new()
            .padding(1)
            .layer()
            .padding(2)
            .layer()
            .padding(3)
            .layer()
            .padding(4),
    );
}

#[test]
fn layer_then_margin_only() {
    let _: E = text("hello").modify(
        Modifier::new()
            .padding(10)
            .background_color(Color::WHITE)
            .layer()
            .margin(Padding::from(8)),
    );
}

#[test]
fn same_property_before_and_after_layer() {
    let _: E = text("hello").modify(Modifier::new().padding(10).layer().padding(20));
}

#[test]
fn background_before_and_after_layer() {
    let _: E = text("hello").modify(
        Modifier::new()
            .background_color(Color::from_rgb(1.0, 0.0, 0.0))
            .layer()
            .background_color(Color::from_rgb(0.0, 0.0, 1.0)),
    );
}

#[test]
fn layer_with_only_style_no_layout() {
    let _: E = text("hello").modify(
        Modifier::new()
            .background_color(Color::WHITE)
            .layer()
            .background_color(Color::BLACK),
    );
}

#[test]
fn layer_with_only_layout_no_style() {
    let _: E = text("hello").modify(Modifier::new().padding(10).layer().padding(20));
}

#[test]
fn margin_on_multiple_layers() {
    let _: E = text("hello").modify(
        Modifier::new()
            .padding(5)
            .margin(Padding::from(3))
            .layer()
            .padding(10)
            .margin(Padding::from(6)),
    );
}
