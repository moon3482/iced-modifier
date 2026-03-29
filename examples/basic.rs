use iced::widget::{container, tooltip};
use iced::{Border, Color, Element, Length, Point, Shadow, Theme, Vector, mouse};
use iced_modifier::prelude::*;
use iced_modifier::{column, row};

fn main() -> iced::Result {
    iced::application(App::default, update, view)
        .title("iced_modifier demo")
        .theme(Theme::Light)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    CardClicked,
    Hovered(bool),
    RightClicked,
    Scrolled(String),
    MouseMoved(Point),
}

#[derive(Default)]
struct App {
    hover_count: u32,
    last_scroll: String,
    last_mouse_pos: Option<Point>,
}

fn update(state: &mut App, message: Message) -> iced::Task<Message> {
    match message {
        Message::CardClicked => println!("Card clicked!"),
        Message::Hovered(entered) => {
            if entered {
                state.hover_count += 1;
                println!("Hover #{}", state.hover_count);
            }
        }
        Message::RightClicked => println!("Right clicked!"),
        Message::Scrolled(info) => {
            state.last_scroll = info;
        }
        Message::MouseMoved(pos) => {
            state.last_mouse_pos = Some(pos);
        }
    }
    iced::Task::none()
}

fn view(state: &App) -> Element<'_, Message> {
    // ═══════════════════════════════════════════════════
    // 1. Direct Chaining (NEW — SwiftUI style)
    // ═══════════════════════════════════════════════════

    let direct_card = Text::new("Direct Chaining Card")
        .font_size(16)
        .padding(20)
        .background_color(Color::from_rgb(0.93, 0.94, 0.98))
        .corner_radius(12);

    let direct_shadow = Text::new("Shadow + Border")
        .font_size(16)
        .padding(20)
        .background_color(Color::WHITE)
        .corner_radius(8)
        .border(Border { color: Color::from_rgb(0.8, 0.8, 0.85), width: 1.0, ..Border::default() })
        .shadow(Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 8.0,
        });

    let direct_clickable = Text::new("Click me!")
        .font_size(16)
        .padding(20)
        .background_color(Color::from_rgb(0.9, 0.95, 1.0))
        .corner_radius(8)
        .on_press(Message::CardClicked)
        .cursor(mouse::Interaction::Pointer);

    // ═══════════════════════════════════════════════════
    // 2. Column / Row with direct chaining
    // ═══════════════════════════════════════════════════

    let styled_column = column![
        Text::new("Item A").font_size(14).padding(8).background_color(Color::from_rgb(1.0, 0.95, 0.95)),
        Text::new("Item B").font_size(14).padding(8).background_color(Color::from_rgb(0.95, 1.0, 0.95)),
        Text::new("Item C").font_size(14).padding(8).background_color(Color::from_rgb(0.95, 0.95, 1.0)),
    ]
    .spacing(4)
    .padding(12)
    .background_color(Color::WHITE)
    .corner_radius(8);

    let styled_row = row![
        Text::new("Left").font_size(14).fill_portion(1).padding(8).background_color(Color::from_rgb(1.0, 0.9, 0.9)),
        Text::new("Right").font_size(14).fill_portion(2).padding(8).background_color(Color::from_rgb(0.9, 0.9, 1.0)),
    ]
    .spacing(4)
    .padding(8)
    .background_color(Color::from_rgb(0.97, 0.97, 0.97))
    .corner_radius(6);

    // ═══════════════════════════════════════════════════
    // 3. Interactive Column (clickable card with children)
    // ═══════════════════════════════════════════════════

    let interactive_column = column![
        Text::new("Clickable Column").font_size(16).color(Color::from_rgb(0.2, 0.2, 0.4)),
        Text::new("The whole area is clickable").font_size(12).color(Color::from_rgb(0.5, 0.5, 0.6)),
    ]
    .spacing(4)
    .padding(16)
    .background_color(Color::WHITE)
    .corner_radius(8)
    .on_press(Message::CardClicked)
    .on_enter(Message::Hovered(true))
    .on_exit(Message::Hovered(false))
    .cursor(mouse::Interaction::Pointer);

    // ═══════════════════════════════════════════════════
    // 4. Hover detection (direct chaining)
    // ═══════════════════════════════════════════════════

    let hover_text = format!("Hover me! (count: {})", state.hover_count);
    let hover_card = Text::new(hover_text)
        .font_size(14)
        .padding(16)
        .background_color(Color::from_rgb(1.0, 0.95, 0.9))
        .corner_radius(8)
        .on_enter(Message::Hovered(true))
        .on_exit(Message::Hovered(false));

    // ═══════════════════════════════════════════════════
    // 5. Tooltip (direct chaining)
    // ═══════════════════════════════════════════════════

    let with_tooltip = Text::new("Hover for tooltip")
        .font_size(14)
        .padding(12)
        .background_color(Color::from_rgb(0.85, 0.92, 0.85))
        .corner_radius(6)
        .tooltip_text("Tooltip with gap=12, snap=true", tooltip::Position::Top)
        .tooltip_gap(12.0)
        .tooltip_snap(true);

    // ═══════════════════════════════════════════════════
    // 6. Visibility toggle
    // ═══════════════════════════════════════════════════

    let hidden_item = Text::new("You can't see me")
        .padding(10)
        .hidden(true);

    let visible_item = Text::new("I'm visible")
        .font_size(14)
        .padding(10)
        .background_color(Color::from_rgb(0.9, 1.0, 0.9))
        .corner_radius(4);

    // ═══════════════════════════════════════════════════
    // 7. Layering (direct chaining)
    // ═══════════════════════════════════════════════════

    let layered = Text::new("Layered (padding then bg)")
        .font_size(14)
        .padding(20)
        .layer()
        .background_color(Color::from_rgb(1.0, 0.9, 0.8))
        .corner_radius(8);

    // ═══════════════════════════════════════════════════
    // 8. Scrollable Column (direct chaining)
    // ═══════════════════════════════════════════════════

    let scrollable_content = column![
        Text::new("Scroll me").font_size(14),
        text("Line 1"), text("Line 2"), text("Line 3"),
        text("Line 4"), text("Line 5"), text("Line 6"),
        text("Line 7"), text("Line 8"),
    ]
    .spacing(4)
    .height(100)
    .padding(8)
    .background_color(Color::from_rgb(0.97, 0.97, 0.97))
    .corner_radius(6)
    .scrollable();

    // Chat-style (anchored to bottom)
    let chat_scrollable = column![
        text("Msg 1"), text("Msg 2"), text("Msg 3"), text("Msg 4"),
        text("Msg 5"), text("Msg 6"), text("Msg 7"),
        Text::new("Msg 8 (latest)").font_size(13),
    ]
    .spacing(4)
    .height(80)
    .padding(8)
    .background_color(Color::from_rgb(0.95, 0.95, 1.0))
    .corner_radius(6)
    .scrollable()
    .scroll_anchor_bottom();

    // ═══════════════════════════════════════════════════
    // 9. Reusable style + .modify() (backward compatible)
    // ═══════════════════════════════════════════════════

    fn card_base() -> Modifier {
        Modifier::new()
            .background_color(Color::WHITE)
            .corner_radius(8)
            .padding(16)
    }
    let elevated = card_base().then(Modifier::new().shadow(Shadow {
        color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
        offset: Vector::new(0.0, 4.0),
        blur_radius: 12.0,
    }));
    let composed = text("Composed (base + shadow)").size(14).modify(elevated);

    // Conditional modifier
    let is_error = true;
    let status = Text::new("Error status")
        .font_size(14)
        .padding(10)
        .corner_radius(4)
        .modify_if(is_error, |t| {
            t.background_color(Color::from_rgb(1.0, 0.9, 0.9))
                .text_color(Color::from_rgb(0.8, 0.0, 0.0))
        });

    // ═══════════════════════════════════════════════════
    // 10. on_scroll / on_move (direct chaining)
    // ═══════════════════════════════════════════════════

    let scroll_label = if state.last_scroll.is_empty() {
        "Scroll wheel here".to_string()
    } else {
        format!("Scrolled: {}", state.last_scroll)
    };
    let mouse_label = match state.last_mouse_pos {
        Some(p) => format!("Mouse: ({:.0}, {:.0})", p.x, p.y),
        None => "Move mouse here".to_string(),
    };
    let scroll_area = Text::new(scroll_label)
        .font_size(14)
        .padding(14)
        .background_color(Color::from_rgb(0.9, 1.0, 0.95))
        .corner_radius(8)
        .on_scroll(|delta| Message::Scrolled(format!("{:?}", delta)));

    let move_area = Text::new(mouse_label)
        .font_size(14)
        .padding(14)
        .fill_width()
        .background_color(Color::from_rgb(1.0, 0.97, 0.88))
        .corner_radius(8)
        .on_move(Message::MouseMoved);

    // ═══════════════════════════════════════════════════
    // 11. Alignment demo
    // ═══════════════════════════════════════════════════

    let alignment_demo = row![
        Text::new("Top").font_size(14).padding(8)
            .background_color(Color::from_rgb(0.9, 0.92, 1.0)).corner_radius(4)
            .align_top(Length::Fixed(80.0)),
        Text::new("Bottom").font_size(14).padding(8)
            .background_color(Color::from_rgb(1.0, 0.92, 0.9)).corner_radius(4)
            .align_bottom(Length::Fixed(80.0)),
        Text::new("Center").font_size(14).padding(8)
            .background_color(Color::from_rgb(0.92, 1.0, 0.9)).corner_radius(4)
            .center_y(Length::Fixed(80.0)),
    ]
    .spacing(8);

    // ═══════════════════════════════════════════════════
    // 12. Font size comparison
    // ═══════════════════════════════════════════════════

    let font_size_demo = row![
        Text::new("Small (12)").font_size(12).padding(8)
            .background_color(Color::from_rgb(0.93, 0.95, 1.0)).corner_radius(4),
        Text::new("Medium (18)").font_size(18).padding(8)
            .background_color(Color::from_rgb(0.95, 0.93, 1.0)).corner_radius(4),
        Text::new("Large (28)").font_size(28).padding(8)
            .background_color(Color::from_rgb(1.0, 0.93, 0.95)).corner_radius(4),
    ]
    .spacing(8);

    // ═══════════════════════════════════════════════════
    // Main Layout
    // ═══════════════════════════════════════════════════

    let content = column![
        Text::new("iced_modifier Demo").font_size(28),
        Text::new("SwiftUI/Compose-style direct chaining for iced").font_size(14)
            .color(Color::from_rgb(0.4, 0.4, 0.5)),

        section("Direct Chaining (NEW)"),
        direct_card, direct_shadow, direct_clickable,

        section("Column & Row"),
        styled_column, styled_row,

        section("Interactive Column"),
        interactive_column,

        section("Hover Detection"),
        hover_card,

        section("Tooltip"),
        with_tooltip,

        section("Visibility"),
        hidden_item, visible_item,

        section("Layering"),
        layered,

        section("Scrollable"),
        scrollable_content, chat_scrollable,

        section("Composition & Conditional (.modify)"),
        composed, status,

        section("Mouse Callbacks"),
        scroll_area, move_area,

        section("Alignment"),
        alignment_demo,

        section("Font Size"),
        font_size_demo,
    ]
    .spacing(6)
    .padding(20);

    container(iced::widget::scrollable(content))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

/// Section header helper
fn section<'a>(title: &'a str) -> Text<'a> {
    Text::new(title)
        .font_size(18)
        .padding(iced::Padding { top: 16.0, right: 0.0, bottom: 0.0, left: 0.0 })
}
