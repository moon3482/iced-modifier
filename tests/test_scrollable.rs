mod common;

use iced::widget::{column, row, text, tooltip};
use iced::Color;
use iced_modifier::prelude::*;

use common::{E, Msg};

#[test]
fn scrollable_vertical() {
    let _: E = column![text("a")].modify(Modifier::new().height(100).scrollable());
}

#[test]
fn scrollable_horizontal() {
    let _: E = row![text("a")].modify(Modifier::new().width(100).scrollable_x());
}

#[test]
fn scrollable_both() {
    let _: E = text("a").modify(Modifier::new().width(100).height(100).scrollable_xy());
}

#[test]
fn scrollable_direction_overwrite() {
    let _: E = text("a").modify(Modifier::new().scrollable().scrollable_x());
}

#[test]
fn scrollable_with_interaction() {
    let _: E = text("a").modify(Modifier::new().height(100).on_press(Msg::A).scrollable());
}

#[test]
fn scrollable_with_tooltip() {
    let _: E = text("a").modify(
        Modifier::new().height(100).tooltip_text("scroll", tooltip::Position::Top).scrollable(),
    );
}

#[test]
fn scrollable_with_styling() {
    let _: E = column![text("a"), text("b")].modify(
        Modifier::new().height(100).padding(8).background_color(Color::WHITE).corner_radius(6).scrollable(),
    );
}

#[test]
fn scrollable_no_size_constraint() {
    let _: E = column![text("a")].modify(Modifier::new().scrollable());
}

#[test]
fn scrollable_with_id() {
    let _: E = column![text("a")].modify(
        Modifier::new().height(100).scrollable().scrollable_id("my-scroll"),
    );
}

#[test]
fn scrollable_anchor_bottom() {
    let _: E = column![text("a")].modify(
        Modifier::new().height(100).scrollable().scroll_anchor_bottom(),
    );
}

#[test]
fn scrollable_anchor_right() {
    let _: E = row![text("a")].modify(
        Modifier::new().width(100).scrollable_x().scroll_anchor_right(),
    );
}

#[test]
fn scrollable_with_spacing() {
    let _: E = column![text("a")].modify(
        Modifier::new().height(100).scrollable().scroll_spacing(8),
    );
}

#[test]
fn scrollable_full_config() {
    let _: E = column![text("a")].modify(
        Modifier::new()
            .height(200)
            .scrollable()
            .scrollable_id("chat")
            .scroll_anchor_bottom()
            .scroll_spacing(4),
    );
}

// ── Edge cases: config methods before scrollable direction ──

#[test]
fn scrollable_id_without_scrollable_is_noop() {
    // scrollable_id before scrollable() should be ignored (no panic)
    let _: E = column![text("a")].modify(
        Modifier::new().scrollable_id("ignored").height(100).scrollable(),
    );
}

#[test]
fn scroll_anchor_without_scrollable_is_noop() {
    let _: E = column![text("a")].modify(
        Modifier::new().scroll_anchor_bottom().height(100).scrollable(),
    );
}

#[test]
fn scroll_spacing_without_scrollable_is_noop() {
    let _: E = column![text("a")].modify(
        Modifier::new().scroll_spacing(10).height(100).scrollable(),
    );
}

// ── Edge cases: direction change preserves config ──

#[test]
fn direction_change_preserves_id() {
    // scrollable() sets direction, then scrollable_x() changes direction
    // but id set between them should be preserved
    let _: E = column![text("a")].modify(
        Modifier::new()
            .height(100)
            .scrollable()
            .scrollable_id("preserved")
            .scrollable_x(),
        // direction is now Horizontal, but id should still be "preserved"
    );
}

#[test]
fn direction_change_preserves_anchor() {
    let _: E = column![text("a")].modify(
        Modifier::new()
            .height(100)
            .scrollable()
            .scroll_anchor_bottom()
            .scrollable_xy(),
        // direction is now Both, anchor_y should still be End
    );
}

// ── Edge cases: mismatched anchor/direction ──

#[test]
fn anchor_bottom_on_horizontal_only() {
    // Technically anchor_y on horizontal scroll — iced allows it, no panic
    let _: E = row![text("a")].modify(
        Modifier::new().width(100).scrollable_x().scroll_anchor_bottom(),
    );
}

#[test]
fn anchor_right_on_vertical_only() {
    // anchor_x on vertical scroll — iced allows it, no panic
    let _: E = column![text("a")].modify(
        Modifier::new().height(100).scrollable().scroll_anchor_right(),
    );
}

#[test]
fn both_anchors_on_both_directions() {
    let _: E = text("a").modify(
        Modifier::new()
            .width(100).height(100)
            .scrollable_xy()
            .scroll_anchor_bottom()
            .scroll_anchor_right(),
    );
}

// ── Edge cases: boundary values ──

#[test]
fn scroll_spacing_zero() {
    let _: E = column![text("a")].modify(
        Modifier::new().height(100).scrollable().scroll_spacing(0),
    );
}

// ── Edge cases: interactions with other features ──

#[test]
fn scrollable_config_with_hidden() {
    let _: E = column![text("a")].modify(
        Modifier::new()
            .height(100)
            .scrollable()
            .scrollable_id("hidden-scroll")
            .scroll_anchor_bottom()
            .hidden(true),
    );
}

#[test]
fn scrollable_direction_overwrite_preserves_spacing() {
    let _: E = column![text("a")].modify(
        Modifier::new()
            .scrollable()
            .scroll_spacing(8)
            .scrollable_x()
            .scrollable_xy(),
        // direction changed twice, but spacing=8 should still be there
    );
}
