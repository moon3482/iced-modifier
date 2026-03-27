mod common;

use iced::widget::{text, tooltip};
use iced_modifier::prelude::*;

use common::E;

#[test]
fn tooltip_position_top() {
    let _: E = text("t").modify(Modifier::new().tooltip_text("tip", tooltip::Position::Top));
}

#[test]
fn tooltip_position_bottom() {
    let _: E = text("t").modify(Modifier::new().tooltip_text("tip", tooltip::Position::Bottom));
}

#[test]
fn tooltip_position_left() {
    let _: E = text("t").modify(Modifier::new().tooltip_text("tip", tooltip::Position::Left));
}

#[test]
fn tooltip_position_right() {
    let _: E = text("t").modify(Modifier::new().tooltip_text("tip", tooltip::Position::Right));
}

#[test]
fn tooltip_position_follow_cursor() {
    let _: E = text("t").modify(Modifier::new().tooltip_text("tip", tooltip::Position::FollowCursor));
}

#[test]
fn tooltip_overwrite_last_wins() {
    let _: E = text("t").modify(
        Modifier::new()
            .tooltip_text("first", tooltip::Position::Top)
            .tooltip_text("second", tooltip::Position::Bottom),
    );
}
