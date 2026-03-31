mod common;

use iced::widget::tooltip;
use iced::{Color, mouse};
use iced_modifier::prelude::*;

use common::{E, Msg};

// ═══════════════════════════════════════════════════════════════
// Direct chaining (new API)
// ═══════════════════════════════════════════════════════════════

#[test]
fn direct_font_size() {
    let _: E = Text::new("hello").font_size(16).into();
}

#[test]
fn direct_font_size_with_styling() {
    let _: E = Text::new("hello")
        .font_size(20)
        .padding(12)
        .background_color(Color::WHITE)
        .corner_radius(8)
        .into();
}

#[test]
fn direct_chaining_no_modifier() {
    let _: E = Text::new("hello")
        .font_size(16)
        .padding(10)
        .background_color(Color::from_rgb(0.9, 0.9, 1.0))
        .corner_radius(8)
        .shadow(iced::Shadow {
            color: Color::BLACK,
            offset: iced::Vector::new(0.0, 2.0),
            blur_radius: 4.0,
        })
        .into();
}

#[test]
fn direct_text_helper() {
    let _: E = text("hello").font_size(16).padding(10).into();
}

// ═══════════════════════════════════════════════════════════════
// Interaction type transition (Text → InteractiveText)
// ═══════════════════════════════════════════════════════════════

#[test]
fn on_press_transitions_type() {
    let _: E = Text::new("click me").padding(10).on_press(Msg::A).into();
}

#[test]
fn styling_after_interaction() {
    let _: E = Text::new("click me")
        .on_press(Msg::A)
        .padding(10)
        .background_color(Color::WHITE)
        .corner_radius(8)
        .into();
}

#[test]
fn styling_before_and_after_interaction() {
    let _: E = Text::new("card")
        .font_size(16)
        .padding(12)
        .on_press(Msg::A)
        .background_color(Color::WHITE)
        .on_enter(Msg::B)
        .cursor(mouse::Interaction::Pointer)
        .into();
}

#[test]
fn multiple_interactions() {
    let _: E = Text::new("multi")
        .on_press(Msg::A)
        .on_right_press(Msg::B)
        .on_enter(Msg::Hover(true))
        .on_exit(Msg::Hover(false))
        .cursor(mouse::Interaction::Pointer)
        .into();
}

#[test]
fn all_interaction_methods() {
    let _: E = Text::new("all")
        .on_press(Msg::A)
        .on_release(Msg::A)
        .on_double_click(Msg::A)
        .on_right_press(Msg::A)
        .on_right_release(Msg::A)
        .on_middle_press(Msg::A)
        .on_middle_release(Msg::A)
        .on_enter(Msg::A)
        .on_exit(Msg::A)
        .on_scroll(|_| Msg::A)
        .on_move(|_| Msg::A)
        .cursor(mouse::Interaction::Pointer)
        .into();
}

// ═══════════════════════════════════════════════════════════════
// Widget-specific methods
// ═══════════════════════════════════════════════════════════════

#[test]
fn size_method() {
    let _: E = text("hello").size(16).padding(10).into();
}

#[test]
fn color_method() {
    let _: E = text("hello")
        .color(Color::from_rgb(1.0, 0.0, 0.0))
        .padding(10)
        .into();
}

#[test]
fn text_center_method() {
    let _: E = text("hello").text_center().padding(10).into();
}

#[test]
fn font_size_on_interactive() {
    let _: E = Text::new("hello").on_press(Msg::A).font_size(20).into();
}

// ═══════════════════════════════════════════════════════════════
// Backward compatible .modify()
// ═══════════════════════════════════════════════════════════════

#[test]
fn modify_with_modifier() {
    let _: E = text("hello").modify(Modifier::new().font_size(16).padding(12));
}

#[test]
fn modify_with_interactor() {
    let _: E = text("hello").modify(Modifier::new().font_size(16).padding(12).on_press(Msg::A));
}

#[test]
fn modify_merges_self_data() {
    // font_size on self, padding on modifier — both applied
    let _: E = text("hello")
        .font_size(16)
        .modify(Modifier::new().padding(12));
}

// ═══════════════════════════════════════════════════════════════
// ModifyBase methods on Text
// ═══════════════════════════════════════════════════════════════

#[test]
fn modify_base_layer() {
    let _: E = Text::new("hello")
        .padding(10)
        .background_color(Color::WHITE)
        .layer()
        .padding(5)
        .background_color(Color::BLACK)
        .into();
}

#[test]
fn modify_base_tooltip() {
    let _: E = Text::new("hello")
        .padding(10)
        .tooltip_text("tip", tooltip::Position::Top)
        .tooltip_gap(8.0)
        .into();
}

#[test]
fn modify_base_scrollable() {
    let _: E = Text::new("hello")
        .padding(10)
        .height(50)
        .scrollable()
        .scroll_anchor_bottom()
        .into();
}

#[test]
fn modify_base_hidden() {
    let _: E = Text::new("hello")
        .font_size(16)
        .padding(10)
        .hidden(true)
        .into();
}

#[test]
fn modify_base_conditional() {
    let _: E = Text::new("hello")
        .padding(10)
        .modify_if(true, |t| t.background_color(Color::WHITE))
        .into();
}

// ═══════════════════════════════════════════════════════════════
// From<Text> for Element — auto conversion
// ═══════════════════════════════════════════════════════════════

#[test]
fn into_element_plain() {
    let _: E = Text::new("hello").into();
}

#[test]
fn into_element_with_styling() {
    let _: E = Text::new("hello")
        .padding(10)
        .background_color(Color::WHITE)
        .into();
}

#[test]
fn into_element_interactive() {
    let _: E = Text::new("hello").on_press(Msg::A).padding(10).into();
}

// ═══════════════════════════════════════════════════════════════
// Edge cases
// ═══════════════════════════════════════════════════════════════

#[test]
fn font_size_zero() {
    let _: E = text("hello").font_size(0).into();
}

#[test]
fn font_size_large() {
    let _: E = text("hello").font_size(200).into();
}

#[test]
fn font_size_and_size_both() {
    // font_size in extras overrides inner.size during From conversion
    let _: E = text("hello").size(12).font_size(24).into();
}

#[test]
fn empty_text() {
    let _: E = Text::new("").padding(10).into();
}

#[test]
fn font_size_overwrite() {
    let _: E = text("hello").font_size(12).font_size(24).into();
}

#[test]
fn interactive_with_hidden() {
    let _: E = Text::new("hello")
        .on_press(Msg::A)
        .font_size(16)
        .hidden(true)
        .into();
}

#[test]
fn in_column_auto_convert() {
    use iced::widget::column;
    let _: E = column![
        Text::new("A").font_size(14).padding(4),
        Text::new("B").font_size(18).padding(4),
    ]
    .into();
}

// ═══════════════════════════════════════════════════════════════
// API conflict edge cases
// ═══════════════════════════════════════════════════════════════

#[test]
fn text_width_vs_container_width() {
    // .width() = ModifyBase (Container width)
    // .text_width() = iced native (text boundary width before wrapping)
    let _: E = text("long text that might wrap")
        .text_width(100) // text boundary
        .width(200) // container width (ModifyBase)
        .into();
}

#[test]
fn text_height_vs_container_height() {
    let _: E = text("hello")
        .text_height(50) // text boundary
        .height(100) // container height (ModifyBase)
        .into();
}

#[test]
fn font_size_always_overrides_size() {
    // font_size (extras) overwrites size (inner) during From conversion
    // regardless of call order
    let _: E = text("hello").size(12).font_size(24).into(); // → 24px
    let _: E = text("hello").font_size(24).size(12).into(); // → 24px (font_size wins)
}

#[test]
fn interactive_text_text_width() {
    // text_width should work on InteractiveText too
    let _: E = Text::new("hello")
        .on_press(Msg::A)
        .text_width(150)
        .padding(10)
        .into();
}

#[test]
fn interactive_text_in_column() {
    use iced::widget::column;
    let _: E = column![
        Text::new("A").on_press(Msg::A).padding(4),
        Text::new("B").font_size(14),
    ]
    .into();
}

#[test]
fn text_all_modifybase_methods_chain() {
    // Verify all ModifyBase methods compile and chain on Text
    let _: E = Text::new("hello")
        .padding(10)
        .padding_x(5)
        .padding_y(5)
        .margin(4)
        .width(200)
        .height(50)
        .max_width(300)
        .max_height(100)
        .fill_width()
        .background_color(Color::WHITE)
        .corner_radius(8)
        .border_width(1)
        .border_color(Color::BLACK)
        .text_color(Color::BLACK)
        .clip(true)
        .into();
}

#[test]
fn text_layer_with_font_size() {
    // font_size applies to inner Text, layers apply to Container wrapping
    let _: E = Text::new("hello")
        .font_size(20)
        .padding(10)
        .background_color(Color::WHITE)
        .layer()
        .padding(5)
        .background_color(Color::BLACK)
        .into();
}

#[test]
fn modify_if_on_interactive() {
    let _: E = Text::new("hello")
        .on_press(Msg::A)
        .modify_if(true, |t| t.background_color(Color::WHITE))
        .modify_if(false, |t| t.padding(999))
        .into();
}

// ═══════════════════════════════════════════════════════════════
// TextAlign enum
// ═══════════════════════════════════════════════════════════════

#[test]
fn text_align_start() {
    let _: E = Text::new("hello").text_align(TextAlign::Start).into();
}

#[test]
fn text_align_center() {
    let _: E = Text::new("hello").text_align(TextAlign::Center).into();
}

#[test]
fn text_align_end() {
    let _: E = Text::new("hello").text_align(TextAlign::End).into();
}

#[test]
fn text_align_justify() {
    let _: E = Text::new("hello world long text").text_align(TextAlign::Justify).into();
}

#[test]
fn text_align_left_legacy() {
    let _: E = Text::new("hello").text_align(TextAlign::Left).into();
}

#[test]
fn text_align_right_legacy() {
    let _: E = Text::new("hello").text_align(TextAlign::Right).into();
}

#[test]
fn text_align_y_top() {
    let _: E = Text::new("hello").text_align_y(TextAlignY::Top).text_height(100).into();
}

#[test]
fn text_align_y_bottom() {
    let _: E = Text::new("hello").text_align_y(TextAlignY::Bottom).text_height(100).into();
}

#[test]
fn text_align_with_container_align() {
    let _: E = Text::new("hello")
        .text_align(TextAlign::Center)
        .align_left(iced::Length::Fill)
        .into();
}

#[test]
fn text_align_on_interactive() {
    let _: E = Text::new("hello")
        .on_press(Msg::A)
        .text_align(TextAlign::End)
        .into();
}

#[test]
fn text_center_still_works() {
    let _: E = Text::new("hello").text_center().into();
}

#[test]
fn text_align_x_still_works() {
    let _: E = Text::new("hello")
        .text_align_x(iced::alignment::Horizontal::Center)
        .into();
}
