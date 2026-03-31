use iced::widget::{container, text_editor, tooltip};
use iced::{Color, Element, Length, Shadow, Theme, Vector, mouse};
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
    Scrolled(String),
    MouseMoved(iced::Point),
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
    last_mouse_pos: Option<iced::Point>,
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
            if entered {
                state.hover_count += 1;
            }
        }
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
    // 1. IntoColor — hex strings and arrays
    // ═══════════════════════════════════════════════════

    let hex_card = Text::new("Hex Color: #FF5733")
        .font_size(14)
        .padding(16)
        .background_color("#FF5733")
        .text_color("#FFF")
        .corner_radius(8);

    let array_card = Text::new("Array Color: [0.2, 0.6, 1.0]")
        .font_size(14)
        .padding(16)
        .background_color([0.2, 0.6, 1.0])
        .text_color([1.0, 1.0, 1.0])
        .corner_radius(8);

    let rgba_card = Text::new("RGBA: [0.0, 0.8, 0.4, 0.7]")
        .font_size(14)
        .padding(16)
        .background_color([0.0, 0.8, 0.4, 0.7])
        .text_color("#FFFFFF")
        .corner_radius(8);

    // ═══════════════════════════════════════════════════
    // 2. IntoBorder — tuple shortcuts
    // ═══════════════════════════════════════════════════

    let border_radius = Text::new("border(8.0) — radius only")
        .font_size(14)
        .padding(12)
        .border(8.0)
        .background_color("#F0F0F5");

    let border_color_width = Text::new("border((\"#3388FF\", 2.0))")
        .font_size(14)
        .padding(12)
        .border(("#3388FF", 2.0));

    let border_full = Text::new("border((\"#E74C3C\", 2.0, 12.0))")
        .font_size(14)
        .padding(12)
        .border(("#E74C3C", 2.0, 12.0));

    // ═══════════════════════════════════════════════════
    // 3. IntoShadow — tuple shortcuts
    // ═══════════════════════════════════════════════════

    let shadow_blur = Text::new("shadow(8.0) — blur only")
        .font_size(14)
        .padding(16)
        .background_color("#FFF")
        .shadow(8.0);

    let shadow_offset = Text::new("shadow((0.0, 4.0, 12.0))")
        .font_size(14)
        .padding(16)
        .background_color("#FFF")
        .corner_radius(8)
        .shadow((0.0, 4.0, 12.0));

    let shadow_full = Text::new("shadow with hex color")
        .font_size(14)
        .padding(16)
        .background_color("#FFF")
        .corner_radius(8)
        .shadow((0.0, 4.0, 12.0, "#00000040"));

    // ═══════════════════════════════════════════════════
    // 4. Text — direct chaining + font_size
    // ═══════════════════════════════════════════════════

    let font_sizes = row![
        Text::new("12px").font_size(12).padding(8).background_color("#E8EAF6").corner_radius(4),
        Text::new("18px").font_size(18).padding(8).background_color("#C5CAE9").corner_radius(4),
        Text::new("28px").font_size(28).padding(8).background_color("#9FA8DA").corner_radius(4),
    ]
    .spacing(8);

    let clickable_text = Text::new("Click me!")
        .font_size(16)
        .padding(16)
        .background_color("#E3F2FD")
        .corner_radius(8)
        .on_press(Message::CardClicked)
        .cursor(mouse::Interaction::Pointer);

    // ═══════════════════════════════════════════════════
    // 5. Column / Row
    // ═══════════════════════════════════════════════════

    let styled_column = column![
        Text::new("Item A").font_size(14).padding(8).background_color("#FFEBEE"),
        Text::new("Item B").font_size(14).padding(8).background_color("#E8F5E9"),
        Text::new("Item C").font_size(14).padding(8).background_color("#E3F2FD"),
    ]
    .spacing(4)
    .padding(12)
    .background_color("#FFF")
    .corner_radius(8)
    .border(("#DDD", 1.0, 8.0));

    let portions_row = row![
        Text::new("1/3").font_size(14).fill_portion(1).padding(8).background_color("#FFCDD2"),
        Text::new("2/3").font_size(14).fill_portion(2).padding(8).background_color("#BBDEFB"),
    ]
    .spacing(4);

    // Interactive column (whole area clickable)
    let interactive_column = column![
        Text::new("Clickable Column").font_size(16).color("#1A237E"),
        Text::new("The whole area is clickable").font_size(12).color("#5C6BC0"),
    ]
    .spacing(4)
    .padding(16)
    .background_color("#FFF")
    .corner_radius(8)
    .shadow((0.0, 2.0, 8.0, "#00000020"))
    .on_press(Message::CardClicked)
    .cursor(mouse::Interaction::Pointer);

    // ═══════════════════════════════════════════════════
    // 6. Button
    // ═══════════════════════════════════════════════════

    let styled_button = Button::new(Text::new("Styled Button").font_size(14).color("#FFF"))
        .on_press(Message::CardClicked)
        .button_padding(12)
        .padding(4)
        .background_color("#1976D2")
        .corner_radius(8)
        .on_enter(Message::Hovered(true))
        .on_exit(Message::Hovered(false))
        .cursor(mouse::Interaction::Pointer);

    let disabled_button = Button::new(Text::new("Disabled").font_size(14).color("#999"))
        .on_press_maybe(None::<Message>)
        .button_padding(12)
        .padding(4)
        .background_color("#EEEEEE")
        .corner_radius(8);

    // ═══════════════════════════════════════════════════
    // 7. TextInput / TextEditor
    // ═══════════════════════════════════════════════════

    let input = TextInput::new("Type something...", &state.input_value)
        .on_input(Message::TextChanged)
        .font_size(14)
        .input_padding(10)
        .padding(4)
        .corner_radius(6)
        .border(("#CCC", 1.0, 6.0));

    let editor = TextEditor::new(&state.editor_content)
        .on_action(Message::EditorAction)
        .font_size(13)
        .editor_height(60)
        .editor_padding(8)
        .padding(4)
        .corner_radius(6)
        .border(("#CCC", 1.0, 6.0));

    // ═══════════════════════════════════════════════════
    // 8. Checkbox / Toggler / Radio
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
            .text_size(14)
            .padding(4),
        Radio::new("Option B", 2, state.selected_radio, Message::RadioSelected)
            .text_size(14)
            .padding(4),
        Radio::new("Option C", 3, state.selected_radio, Message::RadioSelected)
            .text_size(14)
            .padding(4),
    ]
    .spacing(2);

    // ═══════════════════════════════════════════════════
    // 9. Slider
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
    // 10. PickList
    // ═══════════════════════════════════════════════════

    let options = vec!["Rust", "Kotlin", "Swift", "Dart"];
    let pick_label = match &state.picked {
        Some(v) => format!("Selected: {v}"),
        None => "No selection".to_string(),
    };
    let pick_demo = column![
        Text::new(pick_label).font_size(14),
        PickList::new(options, state.picked.as_deref(), |v: &str| {
            Message::PickSelected(v.to_string())
        })
        .placeholder("Choose language...")
        .text_size(14)
        .list_padding(8)
        .padding(4)
        .corner_radius(6),
    ]
    .spacing(4);

    // ═══════════════════════════════════════════════════
    // 11. Tooltip with config (gap, padding, snap)
    // ═══════════════════════════════════════════════════

    let tooltip_basic = Text::new("Basic tooltip")
        .font_size(14)
        .padding(12)
        .background_color("#E8F5E9")
        .corner_radius(6)
        .tooltip_text("Simple tooltip", tooltip::Position::Top);

    let tooltip_config = Text::new("Configured tooltip")
        .font_size(14)
        .padding(12)
        .background_color("#FFF3E0")
        .corner_radius(6)
        .tooltip_text("gap=12, padding=8, snap=true", tooltip::Position::Bottom)
        .tooltip_gap(12.0)
        .tooltip_padding(8)
        .tooltip_snap(true);

    // ═══════════════════════════════════════════════════
    // 12. Scrollable with anchor
    // ═══════════════════════════════════════════════════

    let scroll_basic = column![
        Text::new("Scroll me").font_size(14),
        text("Line 1"), text("Line 2"), text("Line 3"),
        text("Line 4"), text("Line 5"), text("Line 6"),
    ]
    .spacing(4)
    .height(80)
    .padding(8)
    .background_color("#FAFAFA")
    .corner_radius(6)
    .scrollable();

    // Chat-style: anchored to bottom
    let scroll_anchored = column![
        text("Msg 1"), text("Msg 2"), text("Msg 3"), text("Msg 4"),
        text("Msg 5"), text("Msg 6"), text("Msg 7"),
        Text::new("Msg 8 (latest)").font_size(13).color("#1976D2"),
    ]
    .spacing(4)
    .height(80)
    .padding(8)
    .background_color("#E8EAF6")
    .corner_radius(6)
    .scrollable()
    .scroll_anchor_bottom()
    .scroll_spacing(4);

    // ═══════════════════════════════════════════════════
    // 13. Hover + on_scroll + on_move
    // ═══════════════════════════════════════════════════

    let hover_text = format!("Hover me! (count: {})", state.hover_count);
    let hover_card = Text::new(hover_text)
        .font_size(14)
        .padding(16)
        .background_color("#FFF8E1")
        .corner_radius(8)
        .on_enter(Message::Hovered(true))
        .on_exit(Message::Hovered(false));

    let scroll_label = if state.last_scroll.is_empty() {
        "Scroll wheel here".to_string()
    } else {
        format!("Scrolled: {}", state.last_scroll)
    };
    let scroll_area = Text::new(scroll_label)
        .font_size(14)
        .padding(14)
        .background_color("#E8F5E9")
        .corner_radius(8)
        .on_scroll(|delta| Message::Scrolled(format!("{:?}", delta)));

    let mouse_label = match state.last_mouse_pos {
        Some(p) => format!("Mouse: ({:.0}, {:.0})", p.x, p.y),
        None => "Move mouse here".to_string(),
    };
    let move_area = Text::new(mouse_label)
        .font_size(14)
        .padding(14)
        .fill_width()
        .background_color("#FFF3E0")
        .corner_radius(8)
        .on_move(Message::MouseMoved);

    // ═══════════════════════════════════════════════════
    // 14. Alignment (align_top, align_bottom, center_y)
    // ═══════════════════════════════════════════════════

    let alignment_demo = row![
        Text::new("Top").font_size(14).padding(8)
            .background_color("#E3F2FD").corner_radius(4)
            .align_top(Length::Fixed(80.0)),
        Text::new("Bottom").font_size(14).padding(8)
            .background_color("#FCE4EC").corner_radius(4)
            .align_bottom(Length::Fixed(80.0)),
        Text::new("Center").font_size(14).padding(8)
            .background_color("#E8F5E9").corner_radius(4)
            .center_y(Length::Fixed(80.0)),
    ]
    .spacing(8);

    // ═══════════════════════════════════════════════════
    // 15. Hidden
    // ═══════════════════════════════════════════════════

    let hidden_item = Text::new("You can't see me").padding(10).hidden(true);
    let visible_item = Text::new("I'm visible (hidden item above)")
        .font_size(14)
        .padding(10)
        .background_color("#E8F5E9")
        .corner_radius(4);

    // ═══════════════════════════════════════════════════
    // 16. Layer (order-dependent wrapping)
    // ═══════════════════════════════════════════════════

    let layered = Text::new("Layered (padding then bg)")
        .font_size(14)
        .padding(20)
        .layer()
        .background_color("#FFE0B2")
        .corner_radius(8);

    // ═══════════════════════════════════════════════════
    // 17. Composition (.modify — backward compat)
    // ═══════════════════════════════════════════════════

    fn card_base() -> Modifier {
        Modifier::new()
            .background_color(Color::WHITE)
            .corner_radius(8)
            .padding(16)
    }
    let composed = text("Composed (base + shadow)").size(14).modify(
        card_base().then(Modifier::new().shadow(Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
            offset: Vector::new(0.0, 4.0),
            blur_radius: 12.0,
        })),
    );

    // Conditional modifier
    let is_error = true;
    let status = Text::new("Error status")
        .font_size(14)
        .padding(10)
        .corner_radius(4)
        .modify_if(is_error, |t| {
            t.background_color("#FFCDD2").text_color("#B71C1C")
        });

    // ═══════════════════════════════════════════════════
    // Main Layout
    // ═══════════════════════════════════════════════════

    let content = column![
        Text::new("iced_modifier Demo").font_size(28),
        Text::new("SwiftUI/Compose-style direct chaining for iced")
            .font_size(14)
            .color("#616161"),
        section("IntoColor — hex & array"),
        row![hex_card, array_card, rgba_card].spacing(8),
        section("IntoBorder — tuple shortcuts"),
        row![border_radius, border_color_width, border_full].spacing(8),
        section("IntoShadow — tuple shortcuts"),
        row![shadow_blur, shadow_offset, shadow_full].spacing(8),
        section("Text — font_size & interactions"),
        font_sizes,
        clickable_text,
        section("Column & Row"),
        styled_column,
        portions_row,
        interactive_column,
        section("Button"),
        row![styled_button, disabled_button].spacing(8),
        section("TextInput & TextEditor"),
        input,
        editor,
        section("Checkbox / Toggler / Radio"),
        check,
        toggle,
        radios,
        section("Slider"),
        slider_demo,
        section("PickList"),
        pick_demo,
        section("Tooltip (basic + configured)"),
        row![tooltip_basic, tooltip_config].spacing(8),
        section("Scrollable (basic + anchored)"),
        row![scroll_basic, scroll_anchored].spacing(8),
        section("Hover / on_scroll / on_move"),
        hover_card,
        scroll_area,
        move_area,
        section("Alignment (top / bottom / center)"),
        alignment_demo,
        section("Hidden"),
        hidden_item,
        visible_item,
        section("Layering"),
        layered,
        section("Composition & Conditional (.modify)"),
        composed,
        status,
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
        .padding(iced::Padding {
            top: 16.0,
            right: 0.0,
            bottom: 0.0,
            left: 0.0,
        })
}
