mod common;

use iced::widget::{column, text, tooltip};
use iced::{Border, Color, Length, Padding, Shadow, Vector, mouse};
use iced_modifier::prelude::*;

use common::{E, Msg};

#[test]
fn all_style_properties_at_once() {
    let _: E = text("hello").modify(
        Modifier::new()
            .background_color(Color::WHITE)
            .border(Border { color: Color::BLACK, width: 1.0, radius: 4.into() })
            .shadow(Shadow { color: Color::BLACK, offset: Vector::new(0.0, 2.0), blur_radius: 4.0 })
            .text_color(Color::from_rgb(0.2, 0.2, 0.2)),
    );
}

#[test]
fn all_layout_properties_at_once() {
    let _: E = text("hello").modify(
        Modifier::new()
            .padding(10).margin(Padding::from(5))
            .width(Length::Fixed(200.0)).height(Length::Fixed(100.0))
            .max_width(400.0).max_height(300.0)
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
            .clip(true),
    );
}

#[test]
fn everything_combined() {
    let _: E = column![text("a"), text("b"), text("c")].modify(
        Modifier::new()
            .padding(16).background_color(Color::WHITE).corner_radius(12)
            .shadow(Shadow { color: Color::BLACK, offset: Vector::new(0.0, 2.0), blur_radius: 8.0 })
            .text_color(Color::BLACK)
            .width(Length::Fixed(300.0)).height(Length::Fixed(200.0)).max_width(500.0).clip(true)
            .id("main-card")
            .on_press(Msg::A)
            .on_enter(Msg::Hover(true)).on_exit(Msg::Hover(false))
            .cursor(mouse::Interaction::Pointer)
            .tooltip_text("Interactive scrollable card", tooltip::Position::Bottom)
            .scrollable(),
    );
}
