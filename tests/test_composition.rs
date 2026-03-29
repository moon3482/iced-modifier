mod common;

use iced::widget::{column, text};
use iced::{Color, Shadow, Vector};
use iced_modifier::prelude::*;

use common::{E, Msg};

#[test]
fn then_partial_override() {
    let base = Modifier::new().padding(10).background_color(Color::WHITE);
    let other = Modifier::new().background_color(Color::BLACK);
    let _: E = text("hello").modify(base.then(other));
}

#[test]
fn then_chain_three() {
    let a = Modifier::new().padding(10);
    let b = Modifier::new().background_color(Color::WHITE);
    let c = Modifier::new().corner_radius(8);
    let _: E = text("hello").modify(a.then(b).then(c));
}

#[test]
fn then_with_layered_modifier() {
    let base = Modifier::new().padding(10);
    let layered = Modifier::new()
        .background_color(Color::WHITE)
        .layer()
        .padding(5);
    let _: E = text("hello").modify(base.then(layered));
}

#[test]
fn then_both_empty() {
    let _: E = text("hello").modify(Modifier::new().then(Modifier::new()));
}

#[test]
fn then_self_clone() {
    let m = Modifier::new().padding(10).background_color(Color::WHITE);
    let _: E = text("hello").modify(m.clone().then(m));
}

#[test]
fn reusable_modifier_fn_with_clone() {
    fn card() -> Modifier {
        Modifier::new()
            .padding(16)
            .background_color(Color::WHITE)
            .corner_radius(12)
    }

    let _: E = text("a").modify(card());
    let _: E = text("b").modify(card());
    let _: E = text("c").modify(card().then(Modifier::new().shadow(Shadow {
        color: Color::BLACK,
        offset: Vector::ZERO,
        blur_radius: 4.0,
    })));
    let _: E = text("d").modify(card().on_press(Msg::A));
}

#[test]
fn modifier_clone_independence() {
    let m1 = Modifier::new().padding(10).background_color(Color::WHITE);
    let m2 = m1.clone();
    let _: E = text("a").modify(m1.corner_radius(8));
    let _: E = text("b").modify(m2);
}

#[test]
fn conditional_on_interactor() {
    let _: E = text("hello").modify(
        Modifier::new()
            .on_press(Msg::A)
            .modify_if(true, |m| m.background_color(Color::WHITE))
            .modify_if(false, |m| m.padding(999)),
    );
}

#[test]
fn conditional_if_else_on_interactor() {
    let _: E = text("hello").modify(Modifier::new().padding(10).on_press(Msg::A).modify_if_else(
        true,
        |m| m.background_color(Color::from_rgb(0.0, 1.0, 0.0)),
        |m| m.background_color(Color::from_rgb(0.5, 0.5, 0.5)),
    ));
}

#[test]
fn modify_if_true() {
    let _: E = text("hello").modify(
        Modifier::new()
            .padding(10)
            .modify_if(true, |m| m.background_color(Color::from_rgb(1.0, 0.0, 0.0))),
    );
}

#[test]
fn modify_if_false() {
    let _: E = text("hello").modify(Modifier::new().padding(10).modify_if(false, |m| {
        m.background_color(Color::from_rgb(1.0, 0.0, 0.0))
    }));
}

#[test]
fn modify_if_else_branches() {
    let _: E = text("status").modify(Modifier::new().modify_if_else(
        true,
        |m| m.background_color(Color::from_rgb(0.8, 1.0, 0.8)),
        |m| m.background_color(Color::from_rgb(1.0, 0.8, 0.8)),
    ));
}
