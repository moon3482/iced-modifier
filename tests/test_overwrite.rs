mod common;

use iced::widget::text;
use iced::{Border, Color, Fill, Length, Shadow, Vector};
use iced_modifier::prelude::*;

use common::E;

#[test]
fn padding_overwrite_last_wins() {
    let _: E = text("hello").modify(Modifier::new().padding(10).padding(20));
}

#[test]
fn background_color_then_background_overwrites() {
    let _: E = text("hello").modify(
        Modifier::new()
            .background_color(Color::WHITE)
            .background(iced::Background::Color(Color::BLACK)),
    );
}

#[test]
fn background_then_background_color_overwrites() {
    let _: E = text("hello").modify(
        Modifier::new()
            .background(iced::Background::Color(Color::BLACK))
            .background_color(Color::WHITE),
    );
}

#[test]
fn center_x_then_align_right_last_wins() {
    let _: E = text("hello").modify(Modifier::new().center_x(Fill).align_right(Fill));
}

#[test]
fn align_top_sets_vertical_alignment_and_height() {
    let _: E = text("hello").modify(Modifier::new().align_top(Fill));
}

#[test]
fn align_bottom_sets_vertical_alignment_and_height() {
    let _: E = text("hello").modify(Modifier::new().align_bottom(Fill));
}

#[test]
fn center_y_then_align_bottom_last_wins() {
    let _: E = text("hello").modify(Modifier::new().center_y(Fill).align_bottom(Fill));
}

#[test]
fn width_fixed_then_fill_width_last_wins() {
    let _: E = text("hello").modify(
        Modifier::new().width(Length::Fixed(100.0)).fill_width(),
    );
}

#[test]
fn fill_width_then_width_fixed_last_wins() {
    let _: E = text("hello").modify(
        Modifier::new().fill_width().width(Length::Fixed(100.0)),
    );
}

#[test]
fn border_then_corner_radius_partial_modify() {
    let _: E = text("hello").modify(
        Modifier::new()
            .border(Border { color: Color::BLACK, width: 2.0, radius: 0.into() })
            .corner_radius(8),
    );
}

#[test]
fn corner_radius_then_border_overwrites_all() {
    let _: E = text("hello").modify(
        Modifier::new()
            .corner_radius(8)
            .border(Border { color: Color::BLACK, width: 2.0, radius: 0.into() }),
    );
}

#[test]
fn border_color_then_border_width_accumulate() {
    let _: E = text("hello").modify(
        Modifier::new().border_color(Color::BLACK).border_width(2.0),
    );
}

#[test]
fn shadow_overwrite() {
    let s1 = Shadow { color: Color::BLACK, offset: Vector::ZERO, blur_radius: 4.0 };
    let s2 = Shadow { color: Color::WHITE, offset: Vector::new(1.0, 1.0), blur_radius: 8.0 };
    let _: E = text("hello").modify(Modifier::new().shadow(s1).shadow(s2));
}

#[test]
fn text_color_overwrite() {
    let _: E = text("hello").modify(
        Modifier::new().text_color(Color::BLACK).text_color(Color::WHITE),
    );
}
