mod common;

use iced::widget::{text, tooltip};
use iced::{Color, Length, Shadow, Vector};
use iced_modifier::prelude::*;

use common::E;

#[test]
fn padding_zero() {
    let _: E = text("hello").modify(Modifier::new().padding(0));
}

#[test]
fn width_zero() {
    let _: E = text("hello").modify(Modifier::new().width(Length::Fixed(0.0)));
}

#[test]
fn height_zero() {
    let _: E = text("hello").modify(Modifier::new().height(Length::Fixed(0.0)));
}

#[test]
fn fill_portion_zero() {
    let _: E = text("hello").modify(Modifier::new().fill_portion(0));
}

#[test]
fn fill_portion_one() {
    let _: E = text("hello").modify(Modifier::new().fill_portion(1));
}

#[test]
fn fill_portion_max() {
    let _: E = text("hello").modify(Modifier::new().fill_portion(u16::MAX));
}

#[test]
fn max_width_zero() {
    let _: E = text("hello").modify(Modifier::new().max_width(0.0));
}

#[test]
fn max_height_zero() {
    let _: E = text("hello").modify(Modifier::new().max_height(0.0));
}

#[test]
fn border_width_zero() {
    let _: E = text("hello").modify(Modifier::new().border_width(0.0));
}

#[test]
fn transparent_background() {
    let _: E = text("hello").modify(Modifier::new().background_color(Color::TRANSPARENT));
}

#[test]
fn transparent_text_color() {
    let _: E = text("hello").modify(Modifier::new().text_color(Color::TRANSPARENT));
}

#[test]
fn transparent_border_color() {
    let _: E = text("hello").modify(
        Modifier::new()
            .border_color(Color::TRANSPARENT)
            .border_width(1.0),
    );
}

#[test]
fn empty_string_tooltip() {
    let _: E = text("hello").modify(Modifier::new().tooltip_text("", tooltip::Position::Top));
}

#[test]
fn long_string_tooltip() {
    let long = "a".repeat(10000);
    let _: E = text("hello").modify(Modifier::new().tooltip_text(long, tooltip::Position::Top));
}

#[test]
fn shadow_zero_blur() {
    let _: E = text("hello").modify(Modifier::new().shadow(Shadow {
        color: Color::BLACK,
        offset: Vector::ZERO,
        blur_radius: 0.0,
    }));
}

#[test]
fn large_padding() {
    let _: E = text("hello").modify(Modifier::new().padding(10000.0));
}

#[test]
fn corner_radius_from_different_types() {
    let _: E = text("a").modify(Modifier::new().corner_radius(8u8));
    let _: E = text("b").modify(Modifier::new().corner_radius(8u32));
    let _: E = text("c").modify(Modifier::new().corner_radius(8i32));
    let _: E = text("d").modify(Modifier::new().corner_radius(8.0f32));
}

#[test]
fn padding_from_different_types() {
    let _: E = text("a").modify(Modifier::new().padding(10u16));
    let _: E = text("b").modify(Modifier::new().padding(10.0f32));
    let _: E = text("c").modify(Modifier::new().padding([10.0, 20.0]));
}
