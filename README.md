# iced_modifier

[![Crates.io](https://img.shields.io/crates/v/iced_modifier.svg)](https://crates.io/crates/iced_modifier)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**SwiftUI/Jetpack Compose-style modifier API for [iced](https://github.com/iced-rs/iced)**

Chainable styling, layout, interactions, tooltips, and scrolling — all in one fluent API.

> [한국어 문서 (Korean)](docs/README-kr.md)

## Quick Start

```toml
[dependencies]
iced_modifier = "0.1"
```

```rust
use iced::widget::text;
use iced::Color;
use iced_modifier::prelude::*;

// Pure styling — returns Element directly, no .into() needed
let card = text("Hello").modify(
    Modifier::new()
        .padding(16)
        .background_color(Color::WHITE)
        .corner_radius(12)
);

// With interactions (click, hover, cursor)
let button = text("Click me").modify(
    Modifier::new()
        .padding(12)
        .background_color(Color::WHITE)
        .corner_radius(8)
        .on_press(Message::Clicked)
        .on_enter(Message::Hovered(true))
        .on_exit(Message::Hovered(false))
        .cursor(mouse::Interaction::Pointer)
);
```

## Why iced_modifier?

iced's current API requires verbose Container wrapping for common styling:

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

// After: one-liner with iced_modifier
text("Hello").modify(
    Modifier::new().padding(16).background_color(Color::WHITE).corner_radius(12)
)
```

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

### Interactions

| Method | Description | Compose | SwiftUI |
|--------|-------------|---------|---------|
| `.on_press(msg)` | Click handler | `.clickable {}` | `.onTapGesture {}` |
| `.on_double_click(msg)` | Double-click | `.combinedClickable()` | `.onTapGesture(count: 2)` |
| `.on_enter(msg)` | Hover enter | `.hoverable()` | `.onHover {}` |
| `.on_exit(msg)` | Hover exit | `.hoverable()` | `.onHover {}` |
| `.cursor()` | Cursor style | `.pointerInput()` | `.cursor()` |

### Extras

| Method | Description | Compose | SwiftUI |
|--------|-------------|---------|---------|
| `.tooltip_text()` | Tooltip | `TooltipBox` | `.help()` |
| `.scrollable()` | Vertical scroll | `.verticalScroll()` | `ScrollView` |
| `.scrollable_x()` | Horizontal scroll | `.horizontalScroll()` | `ScrollView(.horizontal)` |
| `.hidden(bool)` | Visibility toggle | `AnimatedVisibility` | `.hidden()` |
| `.id()` | Widget ID | `.testTag()` | `.id()` |

### Composition

```rust
// Reusable styles
fn card_style() -> Modifier {
    Modifier::new()
        .background_color(Color::WHITE)
        .corner_radius(12)
        .padding(16)
}

// Compose with then()
let elevated = card_style().then(Modifier::new().shadow(shadow));

// Conditional modifiers
Modifier::new()
    .padding(10)
    .modify_if(is_error, |m| m.background_color(Color::RED))

// Order-dependent layering
Modifier::new()
    .padding(20)
    .layer()  // flush current layer
    .background_color(Color::RED)  // applied outside padding
```

## Feature Flags

```toml
# Optional features
iced_modifier = { version = "0.1", features = ["icons", "drag-drop", "animation"] }
```

| Feature | Crate | Description |
|---------|-------|-------------|
| `icons` | [iced_fonts](https://crates.io/crates/iced_fonts) | Bootstrap icon fonts, `icon_label()` helper |
| `drag-drop` | [iced_drop](https://crates.io/crates/iced_drop) | `.on_drag()` / `.on_drop()` / `.draggable()` on Element |
| `animation` | [iced_anim](https://crates.io/crates/iced_anim) | Re-exports `iced_anim` for animation support |
| `all` | All above | Enable everything |

## Examples

```bash
cargo run --example basic
cargo run --example icons --features icons
cargo run --example drag_drop --features drag-drop
```

## Compatibility

- **iced**: 0.14.x
- **Rust**: 2024 edition
- **Platforms**: All platforms supported by iced (macOS, Windows, Linux, Web)

## License

MIT License. See [LICENSE](LICENSE) for details.
