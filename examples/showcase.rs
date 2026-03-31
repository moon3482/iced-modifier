use iced::widget::{container, text_editor, tooltip};
use iced::{Color, Element, Length, Shadow, Theme, Vector, mouse};
use iced_modifier::prelude::*;
use iced_modifier::{column, row};

// ═══════════════════════════════════════════════════════════════
// Tab enum
// ═══════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    TextColor,
    Layout,
    ButtonsInputs,
    Controls,
    BordersShadows,
    Interactions,
    ScrollableExtras,
}

impl Tab {
    const ALL: &[Tab] = &[
        Tab::TextColor,
        Tab::Layout,
        Tab::ButtonsInputs,
        Tab::Controls,
        Tab::BordersShadows,
        Tab::Interactions,
        Tab::ScrollableExtras,
    ];

    fn label(self) -> &'static str {
        match self {
            Tab::TextColor => "Text & Color",
            Tab::Layout => "Layout",
            Tab::ButtonsInputs => "Buttons & Inputs",
            Tab::Controls => "Controls",
            Tab::BordersShadows => "Borders & Shadows",
            Tab::Interactions => "Interactions",
            Tab::ScrollableExtras => "Scrollable & Extras",
        }
    }
}

// ═══════════════════════════════════════════════════════════════
// Message enum
// ═══════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
enum Message {
    TabSelected(Tab),
    PrimaryClicked,
    ButtonHovered(bool),
    InputChanged(String),
    InputSubmitted,
    SecureInputChanged(String),
    EditorAction(text_editor::Action),
    CheckToggled(bool),
    ToggleChanged(bool),
    RadioSelected(u8),
    SliderChanged(f32),
    PickSelected(String),
    TextClicked,
    Hovered(bool),
    DoubleClicked,
    RightClicked,
    Scrolled(String),
    MouseMoved(iced::Point),
    ToggleHidden(bool),
    ToggleError(bool),
}

// ═══════════════════════════════════════════════════════════════
// App state
// ═══════════════════════════════════════════════════════════════

struct App {
    active_tab: Tab,
    // Tab 1: Text & Color — no extra state needed
    // Tab 2: Layout — no extra state needed
    // Tab 3: Buttons & Inputs
    button_hovered: bool,
    input_value: String,
    submit_count: u32,
    secure_value: String,
    editor_content: text_editor::Content,
    // Tab 4: Controls
    is_checked: bool,
    is_toggled: bool,
    selected_radio: Option<u8>,
    slider_value: f32,
    picked: Option<String>,
    // Tab 6: Interactions
    click_count: u32,
    interaction_hover_count: u32,
    double_click_count: u32,
    right_click_count: u32,
    last_scroll: String,
    last_mouse_pos: Option<iced::Point>,
    // Tab 7: Extras
    show_hidden: bool,
    is_error: bool,
}

impl App {
    fn new() -> Self {
        Self {
            active_tab: Tab::TextColor,
            button_hovered: false,
            input_value: String::new(),
            submit_count: 0,
            secure_value: String::new(),
            editor_content: text_editor::Content::with_text("Type here...\nMultiple lines supported."),
            is_checked: true,
            is_toggled: false,
            selected_radio: Some(1),
            slider_value: 50.0,
            picked: None,
            click_count: 0,
            interaction_hover_count: 0,
            double_click_count: 0,
            right_click_count: 0,
            last_scroll: String::new(),
            last_mouse_pos: None,
            show_hidden: false,
            is_error: false,
        }
    }
}

// ═══════════════════════════════════════════════════════════════
// main
// ═══════════════════════════════════════════════════════════════

fn main() -> iced::Result {
    iced::application(App::new, update, view)
        .title("iced_modifier showcase")
        .theme(Theme::Light)
        .run()
}

// ═══════════════════════════════════════════════════════════════
// update
// ═══════════════════════════════════════════════════════════════

fn update(state: &mut App, message: Message) -> iced::Task<Message> {
    match message {
        Message::TabSelected(tab) => state.active_tab = tab,
        Message::PrimaryClicked => println!("Primary button clicked!"),
        Message::ButtonHovered(entered) => state.button_hovered = entered,
        Message::InputChanged(val) => state.input_value = val,
        Message::InputSubmitted => state.submit_count += 1,
        Message::SecureInputChanged(val) => state.secure_value = val,
        Message::EditorAction(action) => state.editor_content.perform(action),
        Message::CheckToggled(val) => state.is_checked = val,
        Message::ToggleChanged(val) => state.is_toggled = val,
        Message::RadioSelected(val) => state.selected_radio = Some(val),
        Message::SliderChanged(val) => state.slider_value = val,
        Message::PickSelected(val) => state.picked = Some(val),
        Message::TextClicked => state.click_count += 1,
        Message::Hovered(entered) => {
            if entered {
                state.interaction_hover_count += 1;
            }
        }
        Message::DoubleClicked => state.double_click_count += 1,
        Message::RightClicked => state.right_click_count += 1,
        Message::Scrolled(info) => state.last_scroll = info,
        Message::MouseMoved(pos) => state.last_mouse_pos = Some(pos),
        Message::ToggleHidden(val) => state.show_hidden = val,
        Message::ToggleError(val) => state.is_error = val,
    }
    iced::Task::none()
}

// ═══════════════════════════════════════════════════════════════
// view
// ═══════════════════════════════════════════════════════════════

fn view(state: &App) -> Element<'_, Message> {
    let tabs = tab_bar(state.active_tab);

    let content = match state.active_tab {
        Tab::TextColor => tab_text_color(state),
        Tab::Layout => tab_layout(state),
        Tab::ButtonsInputs => tab_buttons_inputs(state),
        Tab::Controls => tab_controls(state),
        Tab::BordersShadows => tab_borders_shadows(state),
        Tab::Interactions => tab_interactions(state),
        Tab::ScrollableExtras => tab_scrollable_extras(state),
    };

    let page = column![
        Text::new("iced_modifier showcase").font_size(28).color("#1A237E"),
        Text::new("Explore every feature across tabs").font_size(14).color("#616161"),
        tabs,
        content,
    ]
    .spacing(8)
    .padding(20);

    container(iced::widget::scrollable(page))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

// ═══════════════════════════════════════════════════════════════
// tab_bar — Row of buttons with active highlighting via modify_if
// ═══════════════════════════════════════════════════════════════

fn tab_bar(active: Tab) -> Row<'static, Message> {
    let mut bar = Row::new().spacing(4).padding_y(8);
    for &tab in Tab::ALL {
        let is_active = tab == active;
        let btn = Button::new(
            Text::new(tab.label())
                .font_size(13)
                .modify_if(is_active, |t| t.color("#FFF"))
                .modify_if(!is_active, |t| t.color("#424242")),
        )
        .on_press(Message::TabSelected(tab))
        .button_padding(iced::Padding {
            top: 10.0,
            right: 14.0,
            bottom: 10.0,
            left: 14.0,
        })
        .corner_radius(6)
        .modify_if(is_active, |b| b.background_color("#1976D2"))
        .modify_if(!is_active, |b| b.background_color("#E0E0E0"))
        .cursor(mouse::Interaction::Pointer);
        bar = bar.push(btn);
    }
    bar
}

// ═══════════════════════════════════════════════════════════════
// Helpers: section() and code_label()
// ═══════════════════════════════════════════════════════════════

fn section<'a>(title: &'a str) -> Text<'a> {
    Text::new(title)
        .font_size(18)
        .color("#1A237E")
        .padding(iced::Padding {
            top: 16.0,
            right: 0.0,
            bottom: 4.0,
            left: 0.0,
        })
}

fn code_label<'a>(code: &'a str) -> Text<'a> {
    Text::new(code)
        .font_size(12)
        .color("#6A1B9A")
        .padding_y(2)
}

// ═══════════════════════════════════════════════════════════════
// Tab 1: Text & Color
// ═══════════════════════════════════════════════════════════════

fn tab_text_color(_state: &App) -> Element<'_, Message> {
    // --- Hex colors ---
    let hex_3 = Text::new("#F00 (3-digit hex)")
        .font_size(14)
        .padding(12)
        .background_color("#F00")
        .text_color("#FFF")
        .corner_radius(6);

    let hex_6 = Text::new("#FF5733 (6-digit hex)")
        .font_size(14)
        .padding(12)
        .background_color("#FF5733")
        .text_color("#FFF")
        .corner_radius(6);

    let hex_8 = Text::new("#FF573380 (8-digit RGBA)")
        .font_size(14)
        .padding(12)
        .background_color("#FF573380")
        .text_color("#333")
        .corner_radius(6);

    // --- Array colors ---
    let arr_rgb = Text::new("[0.2, 0.6, 1.0] RGB")
        .font_size(14)
        .padding(12)
        .background_color([0.2, 0.6, 1.0])
        .text_color("#FFF")
        .corner_radius(6);

    let arr_rgba = Text::new("[0.0, 0.8, 0.4, 0.7] RGBA")
        .font_size(14)
        .padding(12)
        .background_color([0.0, 0.8, 0.4, 0.7])
        .text_color("#FFF")
        .corner_radius(6);

    // --- Native Color ---
    let native_color = Text::new("Color::WHITE (iced native)")
        .font_size(14)
        .padding(12)
        .background_color(Color::WHITE)
        .border(("#CCC", 1.0, 6.0));

    // --- font_size comparison ---
    let font_sizes = row![
        Text::new("12").font_size(12).padding(8).background_color("#E8EAF6").corner_radius(4),
        Text::new("16").font_size(16).padding(8).background_color("#C5CAE9").corner_radius(4),
        Text::new("24").font_size(24).padding(8).background_color("#9FA8DA").corner_radius(4),
        Text::new("32").font_size(32).padding(8).background_color("#7986CB").text_color("#FFF").corner_radius(4),
    ]
    .spacing(8);

    // --- .color() on Text vs .text_color() on modifier ---
    let color_vs_text_color = row![
        Text::new(".color(\"#E65100\")").font_size(14).color("#E65100").padding(10).background_color("#FFF3E0").corner_radius(4),
        Text::new(".text_color(\"#1B5E20\")").font_size(14).padding(10).background_color("#E8F5E9").text_color("#1B5E20").corner_radius(4),
    ]
    .spacing(8);

    // --- .layer() demo ---
    let layer_padding_then_bg = Text::new("padding(16) then layer() then background")
        .font_size(13)
        .padding(16)
        .layer()
        .background_color("#FFE0B2")
        .corner_radius(8);

    let layer_bg_then_padding = Text::new("background then layer() then padding(16)")
        .font_size(13)
        .background_color("#BBDEFB")
        .corner_radius(8)
        .layer()
        .padding(16);

    // --- .text_center() ---
    let centered_text = Text::new("text_center()")
        .font_size(14)
        .text_center()
        .text_width(Length::Fill)
        .text_height(Length::Fixed(60.0))
        .padding(8)
        .background_color("#F3E5F5")
        .corner_radius(6)
        .fill_width();

    column![
        section("Hex Colors (IntoColor)"),
        code_label(".background_color(\"#F00\") | \"#FF5733\" | \"#FF573380\""),
        row![hex_3, hex_6, hex_8].spacing(8),

        section("Array Colors (IntoColor)"),
        code_label(".background_color([0.2, 0.6, 1.0]) | [0.0, 0.8, 0.4, 0.7]"),
        row![arr_rgb, arr_rgba, native_color].spacing(8),

        section("font_size Comparison"),
        code_label(".font_size(12) | .font_size(16) | .font_size(24) | .font_size(32)"),
        font_sizes,

        section(".color() vs .text_color()"),
        code_label("Text::color() sets text directly | .text_color() wraps via container"),
        color_vs_text_color,

        section(".layer() — Order-Dependent Wrapping"),
        code_label("padding then bg: visible gap | bg then padding: bg hugs text"),
        row![layer_padding_then_bg, layer_bg_then_padding].spacing(12),

        section(".text_center()"),
        code_label(".text_center() centers text horizontally and vertically"),
        centered_text,
    ]
    .spacing(6)
    .into()
}

// ═══════════════════════════════════════════════════════════════
// Tab 2: Layout
// ═══════════════════════════════════════════════════════════════

fn tab_layout(_state: &App) -> Element<'_, Message> {
    // --- Spacing comparison ---
    let spacing_4 = column![
        Text::new("spacing(4)").font_size(12).color("#666"),
        Text::new("A").font_size(13).padding(6).background_color("#FFCDD2").corner_radius(4),
        Text::new("B").font_size(13).padding(6).background_color("#FFCDD2").corner_radius(4),
        Text::new("C").font_size(13).padding(6).background_color("#FFCDD2").corner_radius(4),
    ]
    .spacing(4)
    .padding(8)
    .background_color("#FFF")
    .border(("#DDD", 1.0, 6.0));

    let spacing_8 = column![
        Text::new("spacing(8)").font_size(12).color("#666"),
        Text::new("A").font_size(13).padding(6).background_color("#C8E6C9").corner_radius(4),
        Text::new("B").font_size(13).padding(6).background_color("#C8E6C9").corner_radius(4),
        Text::new("C").font_size(13).padding(6).background_color("#C8E6C9").corner_radius(4),
    ]
    .spacing(8)
    .padding(8)
    .background_color("#FFF")
    .border(("#DDD", 1.0, 6.0));

    let spacing_16 = column![
        Text::new("spacing(16)").font_size(12).color("#666"),
        Text::new("A").font_size(13).padding(6).background_color("#BBDEFB").corner_radius(4),
        Text::new("B").font_size(13).padding(6).background_color("#BBDEFB").corner_radius(4),
        Text::new("C").font_size(13).padding(6).background_color("#BBDEFB").corner_radius(4),
    ]
    .spacing(16)
    .padding(8)
    .background_color("#FFF")
    .border(("#DDD", 1.0, 6.0));

    let spacing_row = row![spacing_4, spacing_8, spacing_16].spacing(12);

    // --- Row spacing ---
    let row_spacing = row![
        Text::new("Row").font_size(13).padding(6).background_color("#E1BEE7").corner_radius(4),
        Text::new("spacing").font_size(13).padding(6).background_color("#CE93D8").corner_radius(4),
        Text::new("12").font_size(13).padding(6).background_color("#BA68C8").text_color("#FFF").corner_radius(4),
    ]
    .spacing(12)
    .padding(8)
    .background_color("#FFF")
    .border(("#DDD", 1.0, 6.0));

    // --- fill_portion 1:2:3 ---
    let portions = row![
        Text::new("1").font_size(14).fill_portion(1).padding(10).background_color("#FFCDD2").corner_radius(4),
        Text::new("2").font_size(14).fill_portion(2).padding(10).background_color("#BBDEFB").corner_radius(4),
        Text::new("3").font_size(14).fill_portion(3).padding(10).background_color("#C8E6C9").corner_radius(4),
    ]
    .spacing(4);

    // --- fill_width ---
    let fill_w = Text::new("fill_width()")
        .font_size(14)
        .padding(10)
        .fill_width()
        .background_color("#E8EAF6")
        .corner_radius(4);

    // --- padding vs padding_x vs padding_y ---
    let pad_all = Text::new("padding(16)")
        .font_size(13)
        .padding(16)
        .background_color("#FFF9C4")
        .border(("#FBC02D", 1.0, 4.0));

    let pad_x = Text::new("padding_x(24)")
        .font_size(13)
        .padding_x(24)
        .background_color("#DCEDC8")
        .border(("#8BC34A", 1.0, 4.0));

    let pad_y = Text::new("padding_y(20)")
        .font_size(13)
        .padding_y(20)
        .background_color("#B3E5FC")
        .border(("#03A9F4", 1.0, 4.0));

    let pad_row = row![pad_all, pad_x, pad_y].spacing(8);

    // --- margin ---
    let margin_demo = Text::new("margin(12) around this box")
        .font_size(13)
        .padding(10)
        .background_color("#FFE0B2")
        .corner_radius(4)
        .margin(12)
        .background_color("#FFF3E0")
        .corner_radius(4);

    // --- max_width ---
    let max_w = Text::new("max_width(200) — this text will not exceed 200px wide even with fill_width")
        .font_size(13)
        .padding(10)
        .fill_width()
        .max_width(200)
        .background_color("#E1BEE7")
        .corner_radius(4);

    // --- Vertical alignment in a Row ---
    let align_row = row![
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

    // --- center_x ---
    let center_x_demo = Text::new("center_x(200px)")
        .font_size(13)
        .padding(8)
        .background_color("#F3E5F5")
        .corner_radius(4)
        .center_x(Length::Fixed(200.0));

    column![
        section("Column Spacing (4, 8, 16)"),
        code_label(".spacing(4) | .spacing(8) | .spacing(16)"),
        spacing_row,

        section("Row Spacing"),
        code_label("row![...].spacing(12)"),
        row_spacing,

        section("fill_portion (1:2:3 Ratio)"),
        code_label(".fill_portion(1) | .fill_portion(2) | .fill_portion(3)"),
        portions,

        section("fill_width"),
        code_label(".fill_width() — expands to all available width"),
        fill_w,

        section("padding vs padding_x vs padding_y"),
        code_label(".padding(16) | .padding_x(24) | .padding_y(20)"),
        pad_row,

        section("margin"),
        code_label(".margin(12) wraps content with outer spacing"),
        margin_demo,

        section("max_width Constraint"),
        code_label(".max_width(200) — clamps width at 200px"),
        max_w,

        section("Vertical Alignment in a Row"),
        code_label(".align_top(80) | .align_bottom(80) | .center_y(80)"),
        align_row,

        section("center_x with Fixed Width"),
        code_label(".center_x(Length::Fixed(200.0))"),
        center_x_demo,
    ]
    .spacing(6)
    .into()
}

// ═══════════════════════════════════════════════════════════════
// Tab 3: Buttons & Inputs
// ═══════════════════════════════════════════════════════════════

fn tab_buttons_inputs(state: &App) -> Element<'_, Message> {
    // --- Styled button with hover ---
    let hover_bg = if state.button_hovered { "#1565C0" } else { "#1976D2" };
    let styled_btn = Button::new(
        Text::new("Styled Button").font_size(14).color("#FFF"),
    )
    .on_press(Message::PrimaryClicked)
    .button_padding(iced::Padding {
        top: 10.0,
        right: 20.0,
        bottom: 10.0,
        left: 20.0,
    })
    .background_color(hover_bg)
    .corner_radius(8)
    .shadow((0.0, 2.0, 6.0, "#00000030"))
    .on_enter(Message::ButtonHovered(true))
    .on_exit(Message::ButtonHovered(false))
    .cursor(mouse::Interaction::Pointer);

    // --- Disabled button ---
    let disabled_btn = Button::new(
        Text::new("Disabled").font_size(14).color("#999"),
    )
    .on_press_maybe(None::<Message>)
    .button_padding(iced::Padding {
        top: 10.0,
        right: 20.0,
        bottom: 10.0,
        left: 20.0,
    })
    .background_color("#EEEEEE")
    .corner_radius(8);

    // --- Outline / ghost button ---
    let outline_btn = Button::new(
        Text::new("Outline Button").font_size(14).color("#1976D2"),
    )
    .on_press(Message::PrimaryClicked)
    .button_padding(iced::Padding {
        top: 10.0,
        right: 20.0,
        bottom: 10.0,
        left: 20.0,
    })
    .background_color(Color::TRANSPARENT)
    .border(("#1976D2", 2.0, 8.0));

    // --- TextInput ---
    let submit_label = format!("Type and press Enter (submitted {} times)", state.submit_count);
    let input = TextInput::new("Type something...", &state.input_value)
        .on_input(Message::InputChanged)
        .on_submit(Message::InputSubmitted)
        .font_size(14)
        .input_padding(iced::Padding {
            top: 10.0,
            right: 12.0,
            bottom: 10.0,
            left: 12.0,
        })
        .padding(4)
        .corner_radius(6)
        .border(("#CCC", 1.0, 6.0));

    // --- Secure TextInput (password) ---
    let secure_input = TextInput::new("Password...", &state.secure_value)
        .on_input(Message::SecureInputChanged)
        .secure(true)
        .font_size(14)
        .input_padding(iced::Padding {
            top: 10.0,
            right: 12.0,
            bottom: 10.0,
            left: 12.0,
        })
        .padding(4)
        .corner_radius(6)
        .border(("#CCC", 1.0, 6.0));

    // --- TextEditor ---
    let editor = TextEditor::new(&state.editor_content)
        .on_action(Message::EditorAction)
        .font_size(13)
        .editor_height(100)
        .editor_padding(iced::Padding {
            top: 8.0,
            right: 10.0,
            bottom: 8.0,
            left: 10.0,
        })
        .padding(4)
        .corner_radius(6)
        .border(("#CCC", 1.0, 6.0));

    column![
        section("Styled Button (hover + shadow)"),
        code_label(".background_color() .corner_radius(8) .shadow() .on_enter() .cursor(Pointer)"),
        row![styled_btn, disabled_btn, outline_btn].spacing(8),

        section("TextInput"),
        code_label(".on_input() .on_submit() .input_padding() .font_size(14) .border((\"#CCC\", 1.0, 6.0))"),
        Text::new(submit_label).font_size(13).color("#666"),
        input,

        section("Secure TextInput (Password)"),
        code_label(".secure(true) — input is masked"),
        secure_input,

        section("TextEditor"),
        code_label(".on_action() .editor_height(100) .editor_padding() .font_size(13)"),
        editor,
    ]
    .spacing(6)
    .into()
}

// ═══════════════════════════════════════════════════════════════
// Tab 4: Controls (stub)
// ═══════════════════════════════════════════════════════════════

fn tab_controls(state: &App) -> Element<'_, Message> {
    // ── Checkbox ──
    let check_basic = Checkbox::new(state.is_checked)
        .label("Accept terms")
        .on_toggle(Message::CheckToggled);
    let check_styled = Checkbox::new(state.is_checked)
        .label("Accept terms (styled)")
        .on_toggle(Message::CheckToggled)
        .check_size(24)
        .check_spacing(12)
        .text_size(16)
        .padding(8);

    // ── Toggler ──
    let toggle_basic = Toggler::new(state.is_toggled)
        .label("Dark mode")
        .on_toggle(Message::ToggleChanged);
    let toggle_styled = Toggler::new(state.is_toggled)
        .label("Dark mode (styled)")
        .on_toggle(Message::ToggleChanged)
        .toggler_size(28)
        .text_size(16)
        .padding(8);

    // ── Radio ──
    let radio_label = match state.selected_radio {
        Some(1) => "Selected: Small",
        Some(2) => "Selected: Medium",
        Some(3) => "Selected: Large",
        _ => "No selection",
    };
    let radios = column![
        Radio::new("Small", 1, state.selected_radio, Message::RadioSelected)
            .text_size(14).radio_size(18).padding(4),
        Radio::new("Medium", 2, state.selected_radio, Message::RadioSelected)
            .text_size(14).radio_size(18).padding(4),
        Radio::new("Large", 3, state.selected_radio, Message::RadioSelected)
            .text_size(14).radio_size(18).padding(4),
    ]
    .spacing(2);

    // ── Slider ──
    let slider_label = format!("Value: {:.0}", state.slider_value);
    let slider_demo = column![
        Text::new(slider_label).font_size(14),
        Slider::new(0.0..=100.0, state.slider_value, Message::SliderChanged)
            .step(1.0).slider_width(Length::Fill).padding(4),
    ]
    .spacing(4);

    // ── PickList ──
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
        .placeholder("Choose...")
        .text_size(14)
        .list_padding(8)
        .corner_radius(6),
    ]
    .spacing(4);

    column![
        section("Checkbox"),
        check_basic,
        code_label(".label(\"Accept terms\").on_toggle(Msg)"),
        check_styled,
        code_label(".check_size(24).check_spacing(12).text_size(16).padding(8)"),
        section("Toggler"),
        toggle_basic,
        code_label(".label(\"Dark mode\").on_toggle(Msg)"),
        toggle_styled,
        code_label(".toggler_size(28).text_size(16).padding(8)"),
        section("Radio"),
        Text::new(radio_label).font_size(14).color("#1976D2"),
        radios,
        code_label(".text_size(14).radio_size(18).padding(4)"),
        section("Slider"),
        slider_demo,
        code_label("Slider::new(0.0..=100.0, val, Msg).step(1.0)"),
        section("PickList"),
        pick_demo,
        code_label(".placeholder(\"Choose...\").text_size(14).list_padding(8)"),
    ]
    .spacing(8)
    .padding(16)
    .into()
}

// ═══════════════════════════════════════════════════════════════
// Tab 5: Borders & Shadows
// ═══════════════════════════════════════════════════════════════

fn tab_borders_shadows(_state: &App) -> Element<'_, Message> {
    let b1 = column![
        Text::new("radius only").font_size(14).padding(16).background_color("#FFF").border(8.0),
        code_label("border(8.0)"),
    ].spacing(4);

    let b2 = column![
        Text::new("hex + width").font_size(14).padding(16).background_color("#FFF").border(("#3388FF", 2.0)),
        code_label("border((\"#3388FF\", 2.0))"),
    ].spacing(4);

    let b3 = column![
        Text::new("hex + width + radius").font_size(14).padding(16).background_color("#FFF").border(("#E74C3C", 2.0, 12.0)),
        code_label("border((\"#E74C3C\", 2.0, 12.0))"),
    ].spacing(4);

    let b4 = column![
        Text::new("Color + width").font_size(14).padding(16).background_color("#FFF").border((Color::BLACK, 1.0)),
        code_label("border((Color::BLACK, 1.0))"),
    ].spacing(4);

    let b5 = column![
        Text::new("Color + w + r").font_size(14).padding(16).background_color("#FFF")
            .border((Color::from_rgb(0.0, 0.5, 1.0), 2.0, 8.0)),
        code_label("border((Color, 2.0, 8.0))"),
    ].spacing(4);

    let b6 = column![
        Text::new("Border struct").font_size(14).padding(16).background_color("#FFF")
            .border(iced::Border { color: Color::from_rgb(0.0, 0.6, 0.3), width: 2.0, radius: 10.0.into() }),
        code_label("border(Border { ... })"),
    ].spacing(4);

    let b_setters = row![
        column![
            Text::new("corner_radius(16)").font_size(14).padding(16).background_color("#FFF").corner_radius(16),
            code_label(".corner_radius(16)"),
        ].spacing(4),
        column![
            Text::new("border_color + width").font_size(14).padding(16).background_color("#FFF")
                .border_color("#F00").border_width(2.0),
            code_label(".border_color(\"#F00\").border_width(2.0)"),
        ].spacing(4),
    ].spacing(8);

    let s1 = column![
        Text::new("blur only").font_size(14).padding(16).background_color("#FFF").shadow(8.0),
        code_label("shadow(8.0)"),
    ].spacing(4);

    let s2 = column![
        Text::new("offset + blur").font_size(14).padding(16).background_color("#FFF")
            .corner_radius(8).shadow((0.0, 4.0, 12.0)),
        code_label("shadow((0.0, 4.0, 12.0))"),
    ].spacing(4);

    let s3 = column![
        Text::new("full hex").font_size(14).padding(16).background_color("#FFF")
            .corner_radius(8).shadow((2.0, 4.0, 8.0, "#00000040")),
        code_label("shadow((2.0, 4.0, 8.0, \"#00000040\"))"),
    ].spacing(4);

    let s4 = column![
        Text::new("full Color").font_size(14).padding(16).background_color("#FFF")
            .corner_radius(8).shadow((0.0, 4.0, 12.0, Color::from_rgba(0.0, 0.0, 0.5, 0.3))),
        code_label("shadow((..., Color))"),
    ].spacing(4);

    let s5 = column![
        Text::new("Shadow struct").font_size(14).padding(16).background_color("#FFF")
            .corner_radius(8).shadow(Shadow { color: Color::from_rgba(0.0, 0.0, 0.0, 0.3), offset: Vector::new(2.0, 4.0), blur_radius: 10.0 }),
        code_label("shadow(Shadow { ... })"),
    ].spacing(4);

    let combined = column![
        Text::new("border + shadow").font_size(14).padding(16).background_color("#FFF")
            .border(("#1976D2", 2.0, 12.0)).shadow((0.0, 4.0, 12.0, "#00000030")),
        code_label(".border((...)).shadow((...))")
    ].spacing(4);

    column![
        section("IntoBorder — tuple shortcuts"),
        row![b1, b2, b3].spacing(8),
        row![b4, b5, b6].spacing(8),
        section("Individual border setters"),
        b_setters,
        section("IntoShadow — tuple shortcuts"),
        row![s1, s2, s3].spacing(8),
        row![s4, s5].spacing(8),
        section("Combined: border + shadow"),
        combined,
    ]
    .spacing(8)
    .padding(16)
    .into()
}

// ═══════════════════════════════════════════════════════════════
// Tab 6: Interactions
// ═══════════════════════════════════════════════════════════════

fn tab_interactions(state: &App) -> Element<'_, Message> {
    column![
        section("Click Events"),
        code_label(".on_press(Msg)"),
        Text::new(format!("Click me! (count: {})", state.click_count))
            .padding(10).background_color("#E3F2FD").corner_radius(4)
            .on_press(Message::TextClicked),
        code_label(".on_double_click(Msg)"),
        Text::new(format!("Double-click me! (count: {})", state.double_click_count))
            .padding(10).background_color("#F3E5F5").corner_radius(4)
            .on_double_click(Message::DoubleClicked),
        code_label(".on_right_press(Msg)"),
        Text::new(format!("Right-click me! (count: {})", state.right_click_count))
            .padding(10).background_color("#FFF3E0").corner_radius(4)
            .on_right_press(Message::RightClicked),

        section("Hover"),
        code_label(".on_enter(Msg).on_exit(Msg).cursor(Pointer)"),
        Text::new(format!("Hover over me! (count: {})", state.interaction_hover_count))
            .padding(10).background_color("#E8F5E9").corner_radius(4)
            .on_enter(Message::Hovered(true)).on_exit(Message::Hovered(false))
            .cursor(mouse::Interaction::Pointer),

        section("Mouse Tracking"),
        code_label(".on_scroll(|delta| Msg)"),
        Text::new(format!("Scroll here: {}", state.last_scroll))
            .padding(10).background_color("#ECEFF1").corner_radius(4)
            .on_scroll(|delta| Message::Scrolled(format!("{:?}", delta))),
        code_label(".on_move(|point| Msg)"),
        Text::new(format!("Move mouse here: {}", match state.last_mouse_pos {
            Some(p) => format!("({:.0}, {:.0})", p.x, p.y),
            None => "---".to_string(),
        }))
        .padding(10).background_color("#ECEFF1").corner_radius(4)
        .on_move(Message::MouseMoved),

        section("Tooltip"),
        code_label(".tooltip_text(\"...\", Position::Top)"),
        Text::new("Hover for tooltip (top)")
            .padding(10).background_color("#E0F7FA").corner_radius(4)
            .tooltip_text("Simple tooltip", tooltip::Position::Top),
        code_label(".tooltip_gap(12).tooltip_padding(8).tooltip_snap(true)"),
        Text::new("Hover for tooltip (bottom, configured)")
            .padding(10).background_color("#FCE4EC").corner_radius(4)
            .tooltip_text("Configured", tooltip::Position::Bottom)
            .tooltip_gap(12.0).tooltip_padding(8).tooltip_snap(true),

        section("Clickable Column"),
        code_label("column![...].on_press(Msg)"),
        column![
            Text::new("Row 1 — click anywhere in this column"),
            Text::new("Row 2 — the whole column is clickable"),
        ]
        .spacing(4).padding(10).background_color("#F1F8E9").corner_radius(4)
        .on_press(Message::TextClicked),
    ]
    .spacing(8)
    .padding(16)
    .into()
}

// ═══════════════════════════════════════════════════════════════
// Tab 7: Scrollable & Extras
// ═══════════════════════════════════════════════════════════════

fn tab_scrollable_extras(state: &App) -> Element<'_, Message> {
    fn card_base() -> Modifier {
        Modifier::new().padding(12).background_color("#FFFFFF").corner_radius(8)
    }

    column![
        section("Scrollable"),
        code_label(".height(80).scrollable()"),
        column![
            text("Line 1"), text("Line 2"), text("Line 3"), text("Line 4"),
            text("Line 5"), text("Line 6"), text("Line 7"), text("Line 8"),
        ]
        .spacing(4).padding(8).background_color("#F5F5F5").corner_radius(4)
        .height(80).scrollable(),

        code_label(".scrollable().scroll_anchor_bottom().scroll_spacing(4)"),
        column![
            text("Msg 1"), text("Msg 2"), text("Msg 3"), text("Msg 4"),
            text("Msg 5"), text("Msg 6"), text("Msg 7"), text("Msg 8"),
        ]
        .spacing(4).padding(8).background_color("#EEEEEE").corner_radius(4)
        .height(80).scrollable().scroll_anchor_bottom().scroll_spacing(4),

        section("Hidden Toggle"),
        code_label(".hidden(bool)"),
        Checkbox::new(state.show_hidden).label("Show hidden item").on_toggle(Message::ToggleHidden),
        Text::new("I can be hidden!").padding(10).background_color("#E8F5E9").hidden(!state.show_hidden),

        section("Conditional Styling"),
        code_label(".modify_if(condition, |t| ...)"),
        Checkbox::new(state.is_error).label("Error state").on_toggle(Message::ToggleError),
        Text::new("modify_if: styled when error is on")
            .padding(10).background_color("#E0E0E0").corner_radius(4)
            .modify_if(state.is_error, |t| t.background_color("#FFCDD2").text_color("#B71C1C")),
        code_label(".modify_if_else(cond, green, gray)"),
        Text::new("modify_if_else: green when on, gray when off")
            .padding(10).corner_radius(4)
            .modify_if_else(state.is_error, |t| t.background_color("#C8E6C9"), |t| t.background_color("#E0E0E0")),

        section("Layer"),
        code_label(".padding(20).layer().background_color(\"#FFE0B2\").corner_radius(8)"),
        Text::new("Padded text, then layered with background")
            .padding(20).layer().background_color("#FFE0B2").corner_radius(8),

        section("Composition (.modify)"),
        code_label("text(...).modify(card_base())"),
        Text::new("Reusable card_base() via .modify()").modify(card_base()),
        code_label("card_base().then(Modifier::new().shadow(...))"),
        Text::new("card_base() + shadow via .then()").modify(
            card_base().then(Modifier::new().shadow(Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                offset: Vector::new(2.0, 2.0),
                blur_radius: 6.0,
            }))
        ),
    ]
    .spacing(8)
    .padding(16)
    .into()
}
