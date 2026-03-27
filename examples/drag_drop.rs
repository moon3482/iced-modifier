use iced::widget::{column, container, row, text};
use iced::{Color, Element, Length, Point, Theme};
use iced_modifier::drag::DragExt;
use iced_modifier::prelude::*;

fn main() -> iced::Result {
    iced::application(App::default, update, view)
        .title("iced_modifier drag & drop demo")
        .theme(Theme::Light)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    Dragging(String, Point),
    Dropped(String, Point),
}

#[derive(Default)]
struct App {
    log: Vec<String>,
}

fn update(state: &mut App, message: Message) -> iced::Task<Message> {
    match message {
        Message::Dragging(name, point) => {
            let msg = format!("Dragging '{}' at ({:.0}, {:.0})", name, point.x, point.y);
            if state.log.last().map_or(true, |last| !last.starts_with("Dragging")) {
                state.log.push(msg);
            } else if let Some(last) = state.log.last_mut() {
                *last = msg;
            }
        }
        Message::Dropped(name, point) => {
            state.log.push(format!("Dropped '{}' at ({:.0}, {:.0})", name, point.x, point.y));
        }
    }
    if state.log.len() > 10 {
        state.log.remove(0);
    }
    iced::Task::none()
}

fn view(state: &App) -> Element<'_, Message> {
    // Draggable cards
    let card_a = text("Card A").size(16).modify(
        Modifier::new()
            .padding(20)
            .background_color(Color::from_rgb(0.9, 0.95, 1.0))
            .corner_radius(8)
            .cursor(iced::mouse::Interaction::Grab),
    )
    .draggable(
        {
            let name = "Card A".to_string();
            move |p, _r| Message::Dragging(name.clone(), p)
        },
        {
            let name = "Card A".to_string();
            move |p, _r| Message::Dropped(name.clone(), p)
        },
    );

    let card_b = text("Card B").size(16).modify(
        Modifier::new()
            .padding(20)
            .background_color(Color::from_rgb(1.0, 0.95, 0.9))
            .corner_radius(8)
            .cursor(iced::mouse::Interaction::Grab),
    )
    .draggable(
        {
            let name = "Card B".to_string();
            move |p, _r| Message::Dragging(name.clone(), p)
        },
        {
            let name = "Card B".to_string();
            move |p, _r| Message::Dropped(name.clone(), p)
        },
    );

    let card_c = text("Card C").size(16).modify(
        Modifier::new()
            .padding(20)
            .background_color(Color::from_rgb(0.95, 1.0, 0.9))
            .corner_radius(8)
            .cursor(iced::mouse::Interaction::Grab),
    )
    .on_drop({
        let name = "Card C".to_string();
        move |p, _r| Message::Dropped(name.clone(), p)
    });

    // Event log
    let log_entries: Vec<Element<Message>> = state
        .log
        .iter()
        .map(|entry| text(entry.as_str()).size(12).into())
        .collect();

    let log_col = if log_entries.is_empty() {
        column![text("Drag the cards above to see events here...").size(12)]
    } else {
        let mut c = column![].spacing(2);
        for entry in log_entries {
            c = c.push(entry);
        }
        c
    };

    let log_box = log_col.modify(
        Modifier::new()
            .padding(12)
            .background_color(Color::from_rgb(0.97, 0.97, 0.97))
            .corner_radius(6)
            .fill_width()
            .height(Length::Fixed(200.0))
            .scrollable(),
    );

    let content = column![
        text("iced_modifier Drag & Drop Demo").size(28),
        text("Using iced_drop with modifier API").size(14),
        text("").size(8),
        text("Draggable Cards").size(18),
        row![card_a, card_b, card_c].spacing(12),
        text("").size(8),
        text("Event Log").size(18),
        log_box,
    ]
    .spacing(6)
    .padding(20);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
