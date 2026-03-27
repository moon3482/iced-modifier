mod common;

use iced::widget::{button, column, container, row, text};
use iced::{Color, Padding};
use iced_modifier::prelude::*;

use common::E;

#[test]
fn same_modifier_on_text() {
    let m = Modifier::new().padding(10).background_color(Color::WHITE).corner_radius(8);
    let _: E = text("hello").modify(m);
}

#[test]
fn same_modifier_on_button() {
    let m = Modifier::new().padding(10).background_color(Color::WHITE).corner_radius(8);
    let _: E = button("click").modify(m);
}

#[test]
fn same_modifier_on_column() {
    let m = Modifier::new().padding(10).background_color(Color::WHITE).corner_radius(8);
    let _: E = column![text("a"), text("b")].modify(m);
}

#[test]
fn same_modifier_on_row() {
    let m = Modifier::new().padding(10).background_color(Color::WHITE).corner_radius(8);
    let _: E = row![text("a"), text("b")].modify(m);
}

#[test]
fn same_modifier_on_container() {
    let m = Modifier::new().padding(10).background_color(Color::WHITE).corner_radius(8);
    let _: E = container(text("inner")).modify(m);
}

#[test]
fn empty_column() {
    let _: E = column![].modify(Modifier::new().padding(10));
}

#[test]
fn empty_row() {
    let _: E = row![].modify(Modifier::new().padding(10));
}

#[test]
fn nested_modify() {
    let inner: E = text("hello").modify(Modifier::new().padding(10));
    let _: E = inner.modify(Modifier::new().background_color(Color::WHITE));
}

#[test]
fn nested_modify_three_levels() {
    let l1: E = text("hello").modify(Modifier::new().padding(5));
    let l2: E = l1.modify(Modifier::new().background_color(Color::WHITE));
    let _: E = l2.modify(Modifier::new().margin(Padding::from(8)));
}
