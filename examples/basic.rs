use iced::widget::{column, container, row, text, tooltip};
use iced::{Border, Color, Element, Length, Shadow, Theme, Vector, mouse};
use iced_modifier::prelude::*;

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
}

#[derive(Default)]
struct App {
    hover_count: u32,
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
    }
    iced::Task::none()
}

fn view(state: &App) -> Element<'_, Message> {
    // 1. Basic styling
    let card1 = text("Basic Card").size(16).modify(
        Modifier::new()
            .padding(20)
            .background_color(Color::from_rgb(0.93, 0.94, 0.98))
            .corner_radius(12),
    );

    // 2. Shadow + border
    let card2 = text("Shadow Card").size(16).modify(
        Modifier::new()
            .padding(20)
            .background_color(Color::WHITE)
            .corner_radius(8)
            .border(Border { color: Color::from_rgb(0.8, 0.8, 0.85), width: 1.0, ..Border::default() })
            .corner_radius(8)
            .shadow(Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 8.0,
            }),
    );

    // 3. Clickable card (Compose: .clickable {}, SwiftUI: .onTapGesture {})
    let clickable_card = text("Click me! (on_press)").size(16).modify(
        Modifier::new()
            .padding(20)
            .background_color(Color::from_rgb(0.9, 0.95, 1.0))
            .corner_radius(8)
            .on_press(Message::CardClicked)
            .cursor(mouse::Interaction::Pointer),
    );

    // 4. Hover detection (Compose: .hoverable(), SwiftUI: .onHover {})
    let hover_text = format!("Hover me! (count: {})", state.hover_count);
    let hover_card = text(hover_text).size(14).modify(
        Modifier::new()
            .padding(16)
            .background_color(Color::from_rgb(1.0, 0.95, 0.9))
            .corner_radius(8)
            .on_enter(Message::Hovered(true))
            .on_exit(Message::Hovered(false)),
    );

    // 5. Multiple interactions (click + right-click + hover + cursor)
    let multi_interact = text("Click / Right-click / Hover").size(14).modify(
        Modifier::new()
            .padding(14)
            .background_color(Color::from_rgb(0.95, 0.9, 1.0))
            .corner_radius(8)
            .on_press(Message::CardClicked)
            .on_right_press(Message::RightClicked)
            .on_enter(Message::Hovered(true))
            .on_exit(Message::Hovered(false))
            .cursor(mouse::Interaction::Pointer),
    );

    // 6. Tooltip (Compose: TooltipBox, SwiftUI: .help())
    let with_tooltip = text("Hover for tooltip").size(14).modify(
        Modifier::new()
            .padding(12)
            .background_color(Color::from_rgb(0.85, 0.92, 0.85))
            .corner_radius(6)
            .tooltip_text("This is a helpful tooltip!", tooltip::Position::Top),
    );

    // 7. Hidden / conditional visibility (SwiftUI: .hidden())
    let hidden_item = text("You can't see me").modify(Modifier::new().hidden(true).padding(10));
    let visible_item = text("I'm visible").size(14).modify(
        Modifier::new()
            .padding(10)
            .background_color(Color::from_rgb(0.9, 1.0, 0.9))
            .corner_radius(4),
    );

    // 8. fill_portion (Compose: .fillMaxWidth(fraction) / .weight())
    let portions = row![
        text("1/3").size(14).modify(
            Modifier::new()
                .fill_portion(1)
                .padding(8)
                .background_color(Color::from_rgb(1.0, 0.9, 0.9))
        ),
        text("2/3").size(14).modify(
            Modifier::new()
                .fill_portion(2)
                .padding(8)
                .background_color(Color::from_rgb(0.9, 0.9, 1.0))
        ),
    ]
    .spacing(4);

    // 9. Order-dependent layering
    let layered = text("Layered (padding then bg)").size(14).modify(
        Modifier::new()
            .padding(20)
            .layer()
            .background_color(Color::from_rgb(1.0, 0.9, 0.8))
            .corner_radius(8),
    );

    // 10. Scrollable content (Compose: .verticalScroll(), SwiftUI: ScrollView)
    let scrollable_content = column![
        text("Scroll me ↓").size(14),
        text("Line 1"),
        text("Line 2"),
        text("Line 3"),
        text("Line 4"),
        text("Line 5"),
        text("Line 6"),
        text("Line 7"),
        text("Line 8"),
    ]
    .spacing(4)
    .modify(
        Modifier::new()
            .height(100)
            .padding(8)
            .background_color(Color::from_rgb(0.97, 0.97, 0.97))
            .corner_radius(6)
            .scrollable(),
    );

    // 11. Reusable style + then() composition
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

    // 12. Conditional modifier
    let is_error = true;
    let status = text("Error status").size(14).modify(
        Modifier::new()
            .padding(10)
            .corner_radius(4)
            .modify_if(is_error, |m| {
                m.background_color(Color::from_rgb(1.0, 0.9, 0.9))
                    .text_color(Color::from_rgb(0.8, 0.0, 0.0))
            }),
    );

    // Main layout
    let content = column![
        text("iced_modifier Demo").size(28),
        text("Compose/SwiftUI-style modifier API for iced").size(14),
        text("").size(8),
        text("Basic Styling").size(18),
        card1, card2,
        text("").size(8),
        text("Interactions (MouseArea)").size(18),
        clickable_card,
        hover_card,
        multi_interact,
        text("").size(8),
        text("Tooltip").size(18),
        with_tooltip,
        text("").size(8),
        text("Visibility").size(18),
        hidden_item, visible_item,
        text("").size(8),
        text("Fill Portion (weight)").size(18),
        portions,
        text("").size(8),
        text("Layering").size(18),
        layered,
        text("").size(8),
        text("Scrollable").size(18),
        scrollable_content,
        text("").size(8),
        text("Composition & Conditional").size(18),
        composed, status,
    ]
    .spacing(6)
    .padding(20);

    container(iced::widget::scrollable(content))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
