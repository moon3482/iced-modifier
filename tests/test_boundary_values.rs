mod common;

use iced::widget::{text, tooltip};
use iced::{Border, Color, Length, Shadow, Vector};
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

// ── IntoColor: hex string and array support ──

#[test]
fn hex_color_6_digit() {
    let _: E = text("a").modify(Modifier::new().background_color("#FF5733"));
}

#[test]
fn hex_color_3_digit() {
    let _: E = text("a").modify(Modifier::new().text_color("#FFF"));
}

#[test]
fn hex_color_8_digit_rgba() {
    let _: E = text("a").modify(Modifier::new().background_color("#FF573380"));
}

#[test]
fn hex_color_no_hash() {
    let _: E = text("a").modify(Modifier::new().border_color("3388FF"));
}

#[test]
fn color_as_rgb_array() {
    let _: E = text("a").modify(Modifier::new().background_color([1.0, 0.5, 0.0]));
}

#[test]
fn color_as_rgba_array() {
    let _: E = text("a").modify(Modifier::new().background_color([1.0, 0.5, 0.0, 0.8]));
}

#[test]
fn color_still_accepts_iced_color() {
    let _: E = text("a").modify(Modifier::new().background_color(Color::WHITE));
    let _: E = text("a").modify(Modifier::new().text_color(Color::BLACK));
    let _: E = text("a").modify(Modifier::new().border_color(Color::TRANSPARENT));
}

#[test]
fn hex_color_on_widget_text() {
    let _: E = Text::new("hello").color("#FF5733").padding(10).into();
}

// ── IntoBorder: tuple shortcuts for border styling ──

#[test]
fn border_from_radius_only() {
    let _: E = text("a").modify(Modifier::new().border(8.0));
}

#[test]
fn border_from_color_width_tuple() {
    let _: E = text("a").modify(Modifier::new().border((Color::BLACK, 1.0)));
}

#[test]
fn border_from_hex_width_tuple() {
    let _: E = text("a").modify(Modifier::new().border(("#000", 1.0)));
}

#[test]
fn border_from_color_width_radius_tuple() {
    let _: E = text("a").modify(Modifier::new().border((Color::BLACK, 1.0, 8.0)));
}

#[test]
fn border_from_hex_width_radius_tuple() {
    let _: E = text("a").modify(Modifier::new().border(("#CCC", 1.0, 8.0)));
}

#[test]
fn border_from_struct_still_works() {
    let _: E = text("a").modify(Modifier::new().border(Border {
        color: Color::from_rgb(1.0, 0.0, 0.0),
        width: 2.0,
        radius: 4.0.into(),
    }));
}

// ── IntoShadow: tuple shortcuts for shadow styling ──

#[test]
fn shadow_from_blur_only() {
    let _: E = text("a").modify(Modifier::new().shadow(8.0));
}

#[test]
fn shadow_from_offset_blur_tuple() {
    let _: E = text("a").modify(Modifier::new().shadow((0.0, 4.0, 8.0)));
}

#[test]
fn shadow_from_full_tuple_color() {
    let _: E = text("a").modify(Modifier::new().shadow((0.0, 4.0, 8.0, Color::BLACK)));
}

#[test]
fn shadow_from_full_tuple_hex() {
    let _: E = text("a").modify(Modifier::new().shadow((0.0, 4.0, 8.0, "#00000040")));
}

#[test]
fn shadow_from_struct_still_works() {
    let _: E = text("a").modify(Modifier::new().shadow(Shadow {
        color: Color::BLACK,
        offset: Vector::new(0.0, 2.0),
        blur_radius: 4.0,
    }));
}
