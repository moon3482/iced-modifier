use iced::widget::{column, container, row, text};
use iced::{Color, Element, Length, Theme};
use iced_modifier::icons::{bootstrap, icon_label, load_icon_fonts};
use iced_modifier::prelude::*;

fn main() -> iced::Result {
    iced::application(boot, update, view)
        .title("iced_modifier icons demo")
        .theme(Theme::Light)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    FontLoaded,
}

#[derive(Default)]
struct App;

fn boot() -> (App, iced::Task<Message>) {
    (App, load_icon_fonts().map(|_result| Message::FontLoaded))
}

fn update(_state: &mut App, _message: Message) -> iced::Task<Message> {
    iced::Task::none()
}

fn view(_state: &App) -> Element<'_, Message> {
    // Individual icons with modifiers
    let check_icon = bootstrap::check().size(24).modify(
        Modifier::new()
            .padding(8)
            .background_color(Color::from_rgb(0.85, 0.95, 0.85))
            .corner_radius(4),
    );

    let gear_icon = bootstrap::gear().size(24).modify(
        Modifier::new()
            .padding(8)
            .background_color(Color::from_rgb(0.9, 0.9, 0.95))
            .corner_radius(4),
    );

    let heart_icon = bootstrap::heart_fill().size(24).modify(
        Modifier::new()
            .padding(8)
            .text_color(Color::from_rgb(0.9, 0.2, 0.2))
            .background_color(Color::from_rgb(1.0, 0.95, 0.95))
            .corner_radius(4),
    );

    let star_icon = bootstrap::star_fill().size(24).modify(
        Modifier::new()
            .padding(8)
            .text_color(Color::from_rgb(0.9, 0.7, 0.0))
            .background_color(Color::from_rgb(1.0, 0.98, 0.9))
            .corner_radius(4),
    );

    // icon_label helper: icon + text in a row
    let label1 = icon_label(bootstrap::check().size(16), "Complete").modify(
        Modifier::new()
            .padding(10)
            .background_color(Color::from_rgb(0.85, 0.95, 0.85))
            .corner_radius(6),
    );

    let label2 = icon_label(bootstrap::exclamation_triangle().size(16), "Warning").modify(
        Modifier::new()
            .padding(10)
            .background_color(Color::from_rgb(1.0, 0.95, 0.85))
            .corner_radius(6),
    );

    let label3 = icon_label(bootstrap::x_circle().size(16), "Error").modify(
        Modifier::new()
            .padding(10)
            .background_color(Color::from_rgb(1.0, 0.9, 0.9))
            .corner_radius(6),
    );

    let label4 = icon_label(bootstrap::info_circle().size(16), "Information").modify(
        Modifier::new()
            .padding(10)
            .background_color(Color::from_rgb(0.9, 0.93, 1.0))
            .corner_radius(6),
    );

    // Icon buttons (icons with interaction)
    let icon_btn = icon_label(bootstrap::gear().size(16), "Settings").modify(
        Modifier::new()
            .padding(10)
            .background_color(Color::WHITE)
            .corner_radius(8)
            .border_color(Color::from_rgb(0.8, 0.8, 0.85))
            .border_width(1.0)
            .on_press(Message::FontLoaded)
            .cursor(iced::mouse::Interaction::Pointer),
    );

    let content = column![
        text("iced_modifier Icons Demo").size(28),
        text("Using iced_fonts with modifier API").size(14),
        text("").size(8),
        text("Individual Icons").size(18),
        row![check_icon, gear_icon, heart_icon, star_icon].spacing(8),
        text("").size(8),
        text("Icon + Label (icon_label helper)").size(18),
        column![label1, label2, label3, label4].spacing(6),
        text("").size(8),
        text("Interactive Icon Button").size(18),
        icon_btn,
    ]
    .spacing(6)
    .padding(20);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
