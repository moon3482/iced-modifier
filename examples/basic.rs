use iced::widget::{container, text_editor, tooltip};
use iced::{Border, Color, Element, Length, Point, Shadow, Theme, Vector, mouse};
use iced_modifier::prelude::*;
use iced_modifier::{column, row};

fn main() -> iced::Result {
    iced::application(App::new, update, view)
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
    TextChanged(String),
    EditorAction(text_editor::Action),
    CheckToggled(bool),
    ToggleChanged(bool),
    RadioSelected(u8),
    SliderChanged(f32),
    PickSelected(String),
}

struct App {
    hover_count: u32,
    last_scroll: String,
    last_mouse_pos: Option<Point>,
    input_value: String,
    editor_content: text_editor::Content,
    is_checked: bool,
    is_toggled: bool,
    selected_radio: Option<u8>,
    slider_value: f32,
    picked: Option<String>,
}

impl App {
    fn new() -> Self {
        Self {
            hover_count: 0,
            last_scroll: String::new(),
            last_mouse_pos: None,
            input_value: String::new(),
            editor_content: text_editor::Content::new(),
            is_checked: true,
            is_toggled: false,
            selected_radio: Some(1),
            slider_value: 50.0,
            picked: None,
        }
    }
}

fn update(state: &mut App, message: Message) -> iced::Task<Message> {
    match message {
        Message::CardClicked => println!("Card clicked!"),
        Message::Hovered(entered) => {
            if entered { state.hover_count += 1; }
        }
        Message::RightClicked => println!("Right clicked!"),
        Message::Scrolled(info) => state.last_scroll = info,
        Message::MouseMoved(pos) => state.last_mouse_pos = Some(pos),
        Message::TextChanged(val) => state.input_value = val,
        Message::EditorAction(action) => state.editor_content.perform(action),
        Message::CheckToggled(val) => state.is_checked = val,
        Message::ToggleChanged(val) => state.is_toggled = val,
        Message::RadioSelected(val) => state.selected_radio = Some(val),
        Message::SliderChanged(val) => state.slider_value = val,
        Message::PickSelected(val) => state.picked = Some(val),
    }
    iced::Task::none()
}

fn view(state: &App) -> Element<'_, Message> {
    // ═══════════════════════════════════════════════════
    // 1. Direct Chaining — Text
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
    // 2. Column / Row
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
        Text::new("Left (1/3)").font_size(14).fill_portion(1).padding(8).background_color(Color::from_rgb(1.0, 0.9, 0.9)),
        Text::new("Right (2/3)").font_size(14).fill_portion(2).padding(8).background_color(Color::from_rgb(0.9, 0.9, 1.0)),
    ]
    .spacing(4);

    // ═══════════════════════════════════════════════════
    // 3. Button
    // ═══════════════════════════════════════════════════

    let styled_button = Button::new(
        Text::new("Styled Button").font_size(14).color(Color::WHITE)
    )
    .on_press(Message::CardClicked)
    .button_padding(12)
    .padding(4)
    .background_color(Color::from_rgb(0.2, 0.5, 0.9))
    .corner_radius(8)
    .on_enter(Message::Hovered(true))
    .on_exit(Message::Hovered(false))
    .cursor(mouse::Interaction::Pointer);

    let disabled_button = Button::new(
        Text::new("Disabled").font_size(14).color(Color::from_rgb(0.6, 0.6, 0.6))
    )
    .on_press_maybe(None::<Message>)
    .button_padding(12)
    .padding(4);

    // ═══════════════════════════════════════════════════
    // 4. Form Controls — TextInput / TextEditor
    // ═══════════════════════════════════════════════════

    let input = TextInput::new("Type something...", &state.input_value)
        .on_input(Message::TextChanged)
        .font_size(14)
        .input_padding(10)
        .padding(4)
        .corner_radius(6);

    let editor = TextEditor::new(&state.editor_content)
        .on_action(Message::EditorAction)
        .font_size(13)
        .editor_height(60)
        .editor_padding(8)
        .padding(4)
        .corner_radius(6);

    // ═══════════════════════════════════════════════════
    // 5. Toggle Controls — Checkbox / Toggler / Radio
    // ═══════════════════════════════════════════════════

    let check = Checkbox::new(state.is_checked)
        .label("I agree to terms")
        .on_toggle(Message::CheckToggled)
        .text_size(14)
        .padding(8);

    let toggle = Toggler::new(state.is_toggled)
        .label("Dark mode")
        .on_toggle(Message::ToggleChanged)
        .text_size(14)
        .padding(8);

    let radios = column![
        Radio::new("Option A", 1, state.selected_radio, Message::RadioSelected)
            .text_size(14).padding(4),
        Radio::new("Option B", 2, state.selected_radio, Message::RadioSelected)
            .text_size(14).padding(4),
        Radio::new("Option C", 3, state.selected_radio, Message::RadioSelected)
            .text_size(14).padding(4),
    ]
    .spacing(2);

    // ═══════════════════════════════════════════════════
    // 6. Slider
    // ═══════════════════════════════════════════════════

    let slider_label = format!("Value: {:.0}", state.slider_value);
    let slider_demo = column![
        Text::new(slider_label).font_size(14),
        Slider::new(0.0..=100.0, state.slider_value, Message::SliderChanged)
            .step(1.0)
            .slider_width(Length::Fill)
            .padding(4),
    ]
    .spacing(4);

    // ═══════════════════════════════════════════════════
    // 7. PickList
    // ═══════════════════════════════════════════════════

    let options = vec!["Rust", "Kotlin", "Swift", "Dart"];
    let pick_label = match &state.picked {
        Some(v) => format!("Selected: {v}"),
        None => "No selection".to_string(),
    };
    let pick_demo = column![
        Text::new(pick_label).font_size(14),
        PickList::new(options, state.picked.as_deref(), |v: &str| Message::PickSelected(v.to_string()))
            .placeholder("Choose language...")
            .text_size(14)
            .list_padding(8)
            .padding(4)
            .corner_radius(6),
    ]
    .spacing(4);

    // ═══════════════════════════════════════════════════
    // 8. Hover / Tooltip / Visibility / Layering
    // ═══════════════════════════════════════════════════

    let hover_text = format!("Hover me! (count: {})", state.hover_count);
    let hover_card = Text::new(hover_text)
        .font_size(14)
        .padding(16)
        .background_color(Color::from_rgb(1.0, 0.95, 0.9))
        .corner_radius(8)
        .on_enter(Message::Hovered(true))
        .on_exit(Message::Hovered(false));

    let with_tooltip = Text::new("Hover for tooltip")
        .font_size(14)
        .padding(12)
        .background_color(Color::from_rgb(0.85, 0.92, 0.85))
        .corner_radius(6)
        .tooltip_text("gap=12, snap=true", tooltip::Position::Top)
        .tooltip_gap(12.0)
        .tooltip_snap(true);

    let layered = Text::new("Layered (padding then bg)")
        .font_size(14)
        .padding(20)
        .layer()
        .background_color(Color::from_rgb(1.0, 0.9, 0.8))
        .corner_radius(8);

    // ═══════════════════════════════════════════════════
    // 9. Scrollable
    // ═══════════════════════════════════════════════════

    let scrollable_content = column![
        Text::new("Scroll me").font_size(14),
        text("Line 1"), text("Line 2"), text("Line 3"),
        text("Line 4"), text("Line 5"), text("Line 6"),
    ]
    .spacing(4)
    .height(80)
    .padding(8)
    .background_color(Color::from_rgb(0.97, 0.97, 0.97))
    .corner_radius(6)
    .scrollable();

    // ═══════════════════════════════════════════════════
    // 10. Mouse Callbacks
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
        .font_size(14).padding(14)
        .background_color(Color::from_rgb(0.9, 1.0, 0.95)).corner_radius(8)
        .on_scroll(|delta| Message::Scrolled(format!("{:?}", delta)));

    let move_area = Text::new(mouse_label)
        .font_size(14).padding(14).fill_width()
        .background_color(Color::from_rgb(1.0, 0.97, 0.88)).corner_radius(8)
        .on_move(Message::MouseMoved);

    // ═══════════════════════════════════════════════════
    // 11. Composition (.modify — backward compat)
    // ═══════════════════════════════════════════════════

    fn card_base() -> Modifier {
        Modifier::new().background_color(Color::WHITE).corner_radius(8).padding(16)
    }
    let composed = text("Composed (base + shadow)").size(14).modify(
        card_base().then(Modifier::new().shadow(Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
            offset: Vector::new(0.0, 4.0),
            blur_radius: 12.0,
        })),
    );

    let is_error = true;
    let status = Text::new("Error status")
        .font_size(14).padding(10).corner_radius(4)
        .modify_if(is_error, |t| {
            t.background_color(Color::from_rgb(1.0, 0.9, 0.9))
                .text_color(Color::from_rgb(0.8, 0.0, 0.0))
        });

    // ═══════════════════════════════════════════════════
    // Main Layout
    // ═══════════════════════════════════════════════════

    let content = column![
        Text::new("iced_modifier Demo").font_size(28),
        Text::new("SwiftUI/Compose-style direct chaining for iced").font_size(14)
            .color(Color::from_rgb(0.4, 0.4, 0.5)),

        section("Text — Direct Chaining"),
        direct_card, direct_shadow, direct_clickable,

        section("Column & Row"),
        styled_column, styled_row,

        section("Button"),
        row![styled_button, disabled_button].spacing(8),

        section("TextInput & TextEditor"),
        input, editor,

        section("Checkbox / Toggler / Radio"),
        check, toggle, radios,

        section("Slider"),
        slider_demo,

        section("PickList"),
        pick_demo,

        section("Hover & Tooltip"),
        hover_card, with_tooltip,

        section("Layering"),
        layered,

        section("Scrollable"),
        scrollable_content,

        section("Mouse Callbacks"),
        scroll_area, move_area,

        section("Composition (.modify)"),
        composed, status,
    ]
    .spacing(6)
    .padding(20);

    container(iced::widget::scrollable(content))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn section<'a>(title: &'a str) -> Text<'a> {
    Text::new(title)
        .font_size(18)
        .padding(iced::Padding { top: 16.0, right: 0.0, bottom: 0.0, left: 0.0 })
}
