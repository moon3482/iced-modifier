mod common;

use iced::{Color, Shadow, Vector, mouse};
use iced_modifier::prelude::*;

use common::{E, Msg};

#[test]
fn button_basic() {
    let _: E = Button::new(text("Click")).on_press(Msg::A).into();
}

#[test]
fn button_helper() {
    let _: E = button(text("Click")).on_press(Msg::A).into();
}

#[test]
fn button_with_styling() {
    let _: E = Button::new(text("Click"))
        .on_press(Msg::A)
        .padding(12)
        .background_color(Color::WHITE)
        .corner_radius(8)
        .into();
}

#[test]
fn button_with_hover() {
    let _: E = Button::new(text("Hover"))
        .on_press(Msg::A)
        .on_enter(Msg::Hover(true))
        .on_exit(Msg::Hover(false))
        .cursor(mouse::Interaction::Pointer)
        .into();
}

#[test]
fn button_on_press_maybe_none() {
    let _: E = Button::new(text("Disabled"))
        .on_press_maybe(None::<Msg>)
        .padding(10)
        .into();
}

#[test]
fn button_native_padding() {
    let _: E = Button::new(text("Click"))
        .on_press(Msg::A)
        .button_padding(8)
        .padding(4) // Container padding
        .into();
}

#[test]
fn button_with_styled_text() {
    let _: E = Button::new(Text::new("Styled").font_size(16).color(Color::WHITE))
        .on_press(Msg::A)
        .padding(12)
        .background_color(Color::from_rgb(0.2, 0.4, 0.8))
        .corner_radius(8)
        .into();
}

#[test]
fn button_hidden() {
    let _: E = Button::new(text("Hidden"))
        .on_press(Msg::A)
        .hidden(true)
        .into();
}

// ── Button style applied directly to iced Button ──

#[test]
fn button_background_color_applied() {
    let _: E = Button::new(text("Styled"))
        .on_press(Msg::A)
        .background_color("#1976D2")
        .into();
}

#[test]
fn button_corner_radius_applied() {
    let _: E = Button::new(text("Rounded"))
        .on_press(Msg::A)
        .corner_radius(12)
        .into();
}

#[test]
fn button_border_applied() {
    let _: E = Button::new(text("Bordered"))
        .on_press(Msg::A)
        .border(("#E74C3C", 2.0, 8.0))
        .into();
}

#[test]
fn button_shadow_applied() {
    let _: E = Button::new(text("Shadow"))
        .on_press(Msg::A)
        .shadow((0.0, 4.0, 8.0, "#00000040"))
        .into();
}

#[test]
fn button_text_color_applied() {
    let _: E = Button::new(text("Colored"))
        .on_press(Msg::A)
        .text_color("#FFF")
        .into();
}

#[test]
fn button_full_style_chain() {
    let _: E = Button::new(text("Full"))
        .on_press(Msg::A)
        .button_padding(12)
        .background_color("#1976D2")
        .corner_radius(8)
        .border(("#FFF", 1.0, 8.0))
        .shadow(Shadow {
            color: Color::BLACK,
            offset: Vector::new(0.0, 2.0),
            blur_radius: 6.0,
        })
        .text_color("#FFFFFF")
        .into();
}

#[test]
fn button_style_with_layout() {
    let _: E = Button::new(text("Layout"))
        .on_press(Msg::A)
        .background_color(Color::WHITE)
        .corner_radius(8)
        .padding(16)
        .fill_width()
        .max_width(300)
        .into();
}

#[test]
fn button_style_with_interactions() {
    let _: E = Button::new(text("Interact"))
        .on_press(Msg::A)
        .background_color("#43A047")
        .corner_radius(6)
        .on_enter(Msg::Hover(true))
        .on_exit(Msg::Hover(false))
        .cursor(mouse::Interaction::Pointer)
        .into();
}

#[test]
fn button_style_with_tooltip() {
    let _: E = Button::new(text("Tip"))
        .on_press(Msg::A)
        .background_color("#FF5722")
        .corner_radius(4)
        .tooltip_text("Click me", iced::widget::tooltip::Position::Top)
        .into();
}

#[test]
fn button_style_with_layer() {
    let _: E = Button::new(text("Layered"))
        .on_press(Msg::A)
        .background_color("#1976D2")
        .corner_radius(8)
        .layer()
        .padding(8)
        .background_color("#E3F2FD")
        .corner_radius(12)
        .into();
}

#[test]
fn button_style_no_style_passthrough() {
    // No style set — should not apply custom .style() to iced Button
    let _: E = Button::new(text("Plain"))
        .on_press(Msg::A)
        .padding(10)
        .into();
}

#[test]
fn button_modify_if_style() {
    let is_active = true;
    let _: E = Button::new(text("Conditional"))
        .on_press(Msg::A)
        .modify_if(is_active, |b| b.background_color("#1976D2").corner_radius(8))
        .modify_if(!is_active, |b| b.background_color("#E0E0E0"))
        .into();
}
