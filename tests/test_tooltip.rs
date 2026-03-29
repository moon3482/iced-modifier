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
    let _: E =
        text("t").modify(Modifier::new().tooltip_text("tip", tooltip::Position::FollowCursor));
}

#[test]
fn tooltip_overwrite_last_wins() {
    let _: E = text("t").modify(
        Modifier::new()
            .tooltip_text("first", tooltip::Position::Top)
            .tooltip_text("second", tooltip::Position::Bottom),
    );
}

#[test]
fn tooltip_with_gap() {
    let _: E = text("t").modify(
        Modifier::new()
            .tooltip_text("tip", tooltip::Position::Top)
            .tooltip_gap(8.0),
    );
}

#[test]
fn tooltip_with_padding() {
    let _: E = text("t").modify(
        Modifier::new()
            .tooltip_text("tip", tooltip::Position::Bottom)
            .tooltip_padding(6),
    );
}

#[test]
fn tooltip_with_snap() {
    let _: E = text("t").modify(
        Modifier::new()
            .tooltip_text("tip", tooltip::Position::Left)
            .tooltip_snap(true),
    );
}

#[test]
fn tooltip_full_config() {
    let _: E = text("t").modify(
        Modifier::new()
            .tooltip_text("tip", tooltip::Position::Right)
            .tooltip_gap(12.0)
            .tooltip_padding(4)
            .tooltip_snap(false),
    );
}

#[test]
fn tooltip_gap_without_tooltip_is_noop() {
    // tooltip_gap before tooltip_text should be ignored (no panic)
    let _: E = text("t").modify(
        Modifier::new()
            .tooltip_gap(10.0)
            .tooltip_text("tip", tooltip::Position::Top),
    );
}

// ── Edge cases: config methods before tooltip_text ──

#[test]
fn tooltip_padding_without_tooltip_is_noop() {
    let _: E = text("t").modify(
        Modifier::new()
            .tooltip_padding(10)
            .tooltip_text("tip", tooltip::Position::Top),
    );
}

#[test]
fn tooltip_snap_without_tooltip_is_noop() {
    let _: E = text("t").modify(
        Modifier::new()
            .tooltip_snap(true)
            .tooltip_text("tip", tooltip::Position::Top),
    );
}

// ── Edge cases: overwrite resets config ──

#[test]
fn tooltip_overwrite_resets_config() {
    // Second tooltip_text call creates fresh TooltipConfig, so gap from first is lost
    let _: E = text("t").modify(
        Modifier::new()
            .tooltip_text("first", tooltip::Position::Top)
            .tooltip_gap(20.0)
            .tooltip_text("second", tooltip::Position::Bottom),
        // gap is None now because tooltip_text creates a new config
    );
}

// ── Edge cases: boundary values ──

#[test]
fn tooltip_gap_zero() {
    let _: E = text("t").modify(
        Modifier::new()
            .tooltip_text("tip", tooltip::Position::Top)
            .tooltip_gap(0.0),
    );
}

#[test]
fn tooltip_gap_negative() {
    let _: E = text("t").modify(
        Modifier::new()
            .tooltip_text("tip", tooltip::Position::Top)
            .tooltip_gap(-5.0),
    );
}

#[test]
fn tooltip_padding_zero() {
    let _: E = text("t").modify(
        Modifier::new()
            .tooltip_text("tip", tooltip::Position::Top)
            .tooltip_padding(0),
    );
}

#[test]
fn empty_tooltip_with_full_config() {
    let _: E = text("t").modify(
        Modifier::new()
            .tooltip_text("", tooltip::Position::Top)
            .tooltip_gap(8.0)
            .tooltip_padding(4)
            .tooltip_snap(true),
    );
}

// ── Edge cases: interactions with other features ──

#[test]
fn tooltip_config_with_hidden_true() {
    // hidden takes priority — tooltip config is irrelevant
    let _: E = text("t").modify(
        Modifier::new()
            .tooltip_text("tip", tooltip::Position::Top)
            .tooltip_gap(8.0)
            .tooltip_snap(true)
            .hidden(true),
    );
}

#[test]
fn tooltip_config_with_modify_if() {
    let _: E = text("t").modify(
        Modifier::new()
            .tooltip_text("tip", tooltip::Position::Top)
            .modify_if(true, |m| m.tooltip_gap(12.0).tooltip_snap(true))
            .modify_if(false, |m| m.tooltip_padding(100)),
    );
}
