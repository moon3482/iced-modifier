# iced_modifier

[![Crates.io](https://img.shields.io/crates/v/iced_modifier.svg)](https://crates.io/crates/iced_modifier)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**SwiftUI/Jetpack Compose-style modifier API for [iced](https://github.com/iced-rs/iced)**

Chainable styling, layout, interactions, and widget-specific properties — all in one fluent API.

> [한국어 문서 (Korean)](docs/README-kr.md)

## Quick Start

```toml
[dependencies]
iced_modifier = "0.2"
```

```rust
use iced_modifier::prelude::*;

// Direct chaining — SwiftUI style
Text::new("Hello")
    .font_size(16)
    .padding(12)
    .background_color(Color::WHITE)
    .corner_radius(8)
    .on_press(Message::Clicked)
    .cursor(mouse::Interaction::Pointer)

// Column with spacing
column![
    Text::new("Item A").font_size(14).padding(8),
    Text::new("Item B").font_size(14).padding(8),
]
.spacing(8)
.padding(12)
.background_color(Color::WHITE)

// Button with styled content
Button::new(Text::new("Submit").font_size(14).color(Color::WHITE))
    .on_press(Message::Submit)
    .padding(12)
    .corner_radius(8)
```

## Why iced_modifier?

```rust
// Before: verbose Container wrapping + style closure
container(text("Hello"))
    .padding(16)
    .style(|_| container::Style {
        background: Some(Color::WHITE.into()),
        border: Border { radius: 12.into(), ..Border::default() },
        ..container::Style::default()
    })
    .into()

// After: direct chaining
Text::new("Hello")
    .padding(16)
    .background_color(Color::WHITE)
    .corner_radius(12)
```

## Widget Wrappers

All 12 major iced widgets have modifier-aware wrappers with direct chaining:

| Widget | Pattern | Widget-Specific Methods |
|--------|---------|----------------------|
| `Text` | A (→ InteractiveText) | `font_size`, `size`, `font`, `color`, `text_width`, `text_height` |
| `Image` | A (→ InteractiveImage) | `opacity`, `rotation`, `content_fit`, `scale` |
| `Column` | B | `spacing`, `push`, `extend`, `column![]` |
| `Row` | B | `spacing`, `push`, `extend`, `row![]` |
| `Button` | B | `on_press`, `on_press_maybe`, `button_padding` |
| `TextInput` | B | `font_size`, `on_input`, `on_submit`, `on_paste`, `secure` |
| `TextEditor` | B | `font_size`, `on_action`, `wrapping`, `editor_padding` |
| `Checkbox` | B | `on_toggle`, `label`, `check_size`, `check_spacing`, `text_size` |
| `Radio` | B | `radio_size`, `radio_spacing`, `text_size` |
| `Toggler` | B | `on_toggle`, `label`, `toggler_size`, `toggler_spacing`, `text_size` |
| `Slider` | B | `step`, `shift_step`, `on_release`, `slider_width`, `slider_height` |
| `PickList` | B | `placeholder`, `text_size`, `menu_height`, `on_open`, `on_close` |

**Pattern A**: No Message generic. Interaction methods (`.on_press()`) transition to interactive type.
**Pattern B**: Has Message generic from children/callbacks. Interactions available directly.

## Features

### Styling

| Method | Description | Compose | SwiftUI |
|--------|-------------|---------|---------|
| `.background_color()` | Background color | `.background()` | `.background()` |
| `.border()` | Full border | `.border()` | `.border()` |
| `.corner_radius()` | Corner rounding | `.clip(RoundedCornerShape())` | `.cornerRadius()` |
| `.shadow()` | Drop shadow | `.shadow()` | `.shadow()` |
| `.text_color()` | Text color | `color` param | `.foregroundColor()` |

### Layout

| Method | Description | Compose | SwiftUI |
|--------|-------------|---------|---------|
| `.padding()` | Inner padding | `.padding()` | `.padding()` |
| `.margin()` | Outer spacing | N/A | N/A |
| `.width()` / `.height()` | Dimensions | `.size()` | `.frame()` |
| `.fill_width()` | Fill available width | `.fillMaxWidth()` | `.frame(maxWidth: .infinity)` |
| `.fill_portion(n)` | Proportional fill | `.weight()` | N/A |
| `.center()` | Center both axes | `.align(Alignment.Center)` | `.center()` |
| `.align_top()` | Align top + set height | `Alignment.Top` | `.frame(alignment: .top)` |
| `.align_bottom()` | Align bottom + set height | `Alignment.Bottom` | `.frame(alignment: .bottom)` |

### Interactions

| Method | Description | Compose | SwiftUI |
|--------|-------------|---------|---------|
| `.on_press(msg)` | Click handler | `.clickable {}` | `.onTapGesture {}` |
| `.on_double_click(msg)` | Double-click | `.combinedClickable()` | `.onTapGesture(count: 2)` |
| `.on_enter(msg)` | Hover enter | `.hoverable()` | `.onHover {}` |
| `.on_exit(msg)` | Hover exit | `.hoverable()` | `.onHover {}` |
| `.on_scroll(fn)` | Scroll wheel event | `.pointerInput()` | `.onScrollGesture {}` |
| `.on_move(fn)` | Mouse move tracking | `.pointerInput()` | `.onContinuousHover {}` |
| `.cursor()` | Cursor style | `.pointerInput()` | `.cursor()` |

### Extras

| Method | Description | Compose | SwiftUI |
|--------|-------------|---------|---------|
| `.tooltip_text()` | Tooltip | `TooltipBox` | `.help()` |
| `.tooltip_gap()` | Tooltip-content gap | — | — |
| `.tooltip_padding()` | Tooltip inner padding | — | — |
| `.tooltip_snap()` | Snap tooltip in viewport | — | — |
| `.scrollable()` | Vertical scroll | `.verticalScroll()` | `ScrollView` |
| `.scrollable_x()` | Horizontal scroll | `.horizontalScroll()` | `ScrollView(.horizontal)` |
| `.scroll_anchor_bottom()` | Start scrolled to bottom | `reverseLayout` | `.defaultScrollAnchor(.bottom)` |
| `.scroll_anchor_right()` | Start scrolled to right | — | `.defaultScrollAnchor(.trailing)` |
| `.scroll_spacing()` | Scrollbar-content spacing | — | — |
| `.scrollable_id()` | Scrollable widget ID | — | — |
| `.hidden(bool)` | Visibility toggle | `AnimatedVisibility` | `.hidden()` |
| `.id()` | Widget ID | `.testTag()` | `.id()` |
| `.font_size()` | Font size (widget wrappers) | `fontSize` | `.font(.system(size:))` |
| `.spacing()` | Child spacing (widget wrappers) | `Arrangement.spacedBy()` | `VStack(spacing:)` |

### Composition

```rust
// Reusable styles (Modifier still available)
fn card_style() -> Modifier {
    Modifier::new()
        .background_color(Color::WHITE)
        .corner_radius(12)
        .padding(16)
}

// Apply with .modify() (backward compatible)
text("Hello").modify(card_style())

// Conditional modifiers
Text::new("Status")
    .padding(10)
    .modify_if(is_error, |t| t.background_color(Color::RED))

// Order-dependent layering
Text::new("Layered")
    .padding(20)
    .layer()
    .background_color(Color::RED)
```

## Feature Flags

```toml
iced_modifier = { version = "0.2", features = ["icons", "drag-drop", "animation"] }
```

| Feature | Crate | Description |
|---------|-------|-------------|
| `icons` | [iced_fonts](https://crates.io/crates/iced_fonts) | Bootstrap icon fonts, `icon_label()` helper |
| `drag-drop` | [iced_drop](https://crates.io/crates/iced_drop) | `.on_drag()` / `.on_drop()` / `.draggable()` on Element |
| `animation` | [iced_anim](https://crates.io/crates/iced_anim) | Re-exports `iced_anim` for animation support |
| `image` | iced (image feature) | `Image` / `InteractiveImage` widget wrapper |
| `all` | All above | Enable everything |

## Examples

```bash
cargo run --example basic
cargo run --example icons --features icons
cargo run --example drag_drop --features drag-drop
```

## Migration from v0.1

- `use iced::widget::text` → Remove, use `text` from prelude (our wrapper)
- `use iced::widget::{column, row}` → Use `use iced_modifier::{column, row}` macros
- `.modify(Modifier::new()...)` → Still works, but direct chaining preferred
- `Extras::tooltip_text` field → Renamed to `Extras::tooltip` (type changed to `TooltipConfig`)
- `Extras::scrollable` field → Type changed from `ScrollDirection` to `ScrollConfig`

## Compatibility

- **iced**: 0.14.x
- **Rust**: 2024 edition
- **Platforms**: All platforms supported by iced (macOS, Windows, Linux, Web)

## License

MIT License. See [LICENSE](LICENSE) for details.
