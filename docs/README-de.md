# iced_modifier

[![Crates.io](https://img.shields.io/crates/v/iced_modifier.svg)](https://crates.io/crates/iced_modifier)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](../LICENSE)

**SwiftUI/Jetpack Compose-Stil Modifier-API fuer [iced](https://github.com/iced-rs/iced)**

Styling, Layout, Interaktionen und Widget-spezifische Eigenschaften -- alles in einer fluessigen, verkettbaren API.

> [English](../README.md)

## Schnellstart

```toml
[dependencies]
iced_modifier = "0.2"
```

```rust
use iced_modifier::prelude::*;

// Direkte Verkettung -- SwiftUI-Stil
Text::new("Hello")
    .font_size(16)
    .text_align(TextAlign::Center)
    .padding(12)
    .background_color(Color::WHITE)
    .corner_radius(8)
    .on_press(Message::Clicked)
    .cursor(mouse::Interaction::Pointer)

// Column + spacing direkt verkettet
column![
    Text::new("Element A").font_size(14).padding(8),
    Text::new("Element B").font_size(14).padding(8),
]
.spacing(8)
.padding(12)
.background_color(Color::WHITE)

// Hex-Farben -- kein Color::from_rgb() noetig
Text::new("Warnung")
    .background_color("#FF5733")
    .text_color("#FFF")
    .border_color([1.0, 0.0, 0.0])

// Button mit gestyltem Inhalt
Button::new(Text::new("Absenden").font_size(14).color("#FFFFFF"))
    .on_press(Message::Submit)
    .padding(12)
    .corner_radius(8)
```

## Warum iced_modifier?

```rust
// Vorher: umstaendliches Container-Wrapping + Style-Closure
container(text("Hello"))
    .padding(16)
    .style(|_| container::Style {
        background: Some(Color::WHITE.into()),
        border: Border { radius: 12.into(), ..Border::default() },
        ..container::Style::default()
    })
    .into()

// Nachher: direkte Verkettung
Text::new("Hello")
    .padding(16)
    .background_color(Color::WHITE)
    .corner_radius(12)
```

## Widget-Wrapper

Alle 12 wichtigen iced-Widgets werden mit direkter Modifier-Verkettung unterstuetzt:

| Widget | Muster | Widget-spezifische Methoden |
|--------|--------|----------------------------|
| `Text` | A (-> InteractiveText) | `font_size`, `size`, `font`, `color`, `text_width`, `text_height` |
| `Image` | A (-> InteractiveImage) | `opacity`, `rotation`, `content_fit`, `scale` |
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

**Muster A**: Kein Message-Generic. Interaktionsmethoden (`.on_press()`) fuehren zum interaktiven Typ.
**Muster B**: Message-Generic durch Kinder/Callbacks. Interaktionen direkt verfuegbar.

## Kerntypen

| Typ | Beschreibung |
|-----|-------------|
| `Text`, `Column`, `Row`, ... | Widget-Wrapper mit integrierten Modifiern. Direkte Verkettung fuer Styling/Layout/Interaktionen |
| `Modifier` | Wiederverwendbare Styling-/Layout-Spezifikation. Ohne Generics speicher-/kombinierbar |
| `Interactor<M>` | Modifier + Interaktions-Handler |
| `ModifyBase` Trait | Bietet 40+ verkettbare Methoden (Wrapper, Modifier, Interactor implementieren alle) |

## Funktionen

### Styling

| Methode | Beschreibung | Compose | SwiftUI |
|---------|-------------|---------|---------|
| `.background_color()` | Hintergrundfarbe (Color, Hex, Array) | `.background()` | `.background()` |
| `.border()` | Rahmen (Border, Tupel, Radius) | `.border()` | `.border()` |
| `.corner_radius()` | Eckenrundung | `.clip(RoundedCornerShape())` | `.cornerRadius()` |
| `.shadow()` | Schatten (Shadow, Tupel, Unschaerfe) | `.shadow()` | `.shadow()` |
| `.text_color()` | Textfarbe (Color, Hex, Array) | `color`-Parameter | `.foregroundColor()` |
| `.text_align()` | Textausrichtung (Start, Center, End, Justify) | `textAlign` | `.multilineTextAlignment()` |

### Farbformate

Alle Farbmethoden (`.background_color()`, `.text_color()`, `.border_color()`, `.color()`) akzeptieren:

| Format | Beispiel |
|--------|----------|
| `Color` | `Color::WHITE`, `Color::from_rgb(1.0, 0.5, 0.0)` |
| Hex `&str` | `"#FF5733"`, `"#FFF"`, `"3388FF"`, `"#FF573380"` (RGBA) |
| `[f32; 3]` | `[1.0, 0.5, 0.0]` (RGB, 0.0-1.0) |
| `[f32; 4]` | `[1.0, 0.5, 0.0, 0.8]` (RGBA, 0.0-1.0) |

### Rahmenformate

`.border()` akzeptiert:

| Format | Beispiel |
|--------|----------|
| `Border` | `Border { color: Color::RED, width: 1.0, radius: 8.0.into() }` |
| `f32` | `8.0` (nur Eckenradius) |
| `(Color, f32)` | `(Color::BLACK, 1.0)` (Farbe + Breite) |
| `(&str, f32)` | `("#000", 1.0)` (Hex-Farbe + Breite) |
| `(Color, f32, f32)` | `(Color::BLACK, 1.0, 8.0)` (Farbe + Breite + Radius) |
| `(&str, f32, f32)` | `("#CCC", 1.0, 8.0)` (Hex + Breite + Radius) |

### Schattenformate

`.shadow()` akzeptiert:

| Format | Beispiel |
|--------|----------|
| `Shadow` | `Shadow { color: Color::BLACK, offset: Vector::new(0.0, 4.0), blur_radius: 8.0 }` |
| `f32` | `8.0` (nur Unschaerferadius, Schwarz, kein Versatz) |
| `(f32, f32, f32)` | `(0.0, 4.0, 8.0)` (x, y, Unschaerfe) |
| `(f32, f32, f32, Color)` | `(0.0, 4.0, 8.0, Color::BLACK)` |
| `(f32, f32, f32, &str)` | `(0.0, 4.0, 8.0, "#00000040")` (mit Hex-Farbe) |

### Layout

| Methode | Beschreibung | Compose | SwiftUI |
|---------|-------------|---------|---------|
| `.padding()` | Innenabstand | `.padding()` | `.padding()` |
| `.margin()` | Aussenabstand | N/A | N/A |
| `.width()` / `.height()` | Abmessungen | `.size()` | `.frame()` |
| `.fill_width()` | Verfuegbare Breite fuellen | `.fillMaxWidth()` | `.frame(maxWidth: .infinity)` |
| `.fill_portion(n)` | Proportionales Fuellen | `.weight()` | N/A |
| `.center()` | Auf beiden Achsen zentrieren | `.align(Alignment.Center)` | `.center()` |
| `.align_top()` | Oben ausrichten + Hoehe setzen | `Alignment.Top` | `.frame(alignment: .top)` |
| `.align_bottom()` | Unten ausrichten + Hoehe setzen | `Alignment.Bottom` | `.frame(alignment: .bottom)` |
| `.max_width()` / `.max_height()` | Maximale Groesse | `.requiredSize()` | `.frame(maxWidth:)` |
| `.clip(bool)` | Ueberlauf-Clipping | `.clip()` | `.clipped()` |

### Interaktionen

| Methode | Beschreibung | Compose | SwiftUI |
|---------|-------------|---------|---------|
| `.on_press(msg)` | Klick-Handler | `.clickable {}` | `.onTapGesture {}` |
| `.on_release(msg)` | Release-Handler | — | — |
| `.on_double_click(msg)` | Doppelklick | `.combinedClickable()` | `.onTapGesture(count: 2)` |
| `.on_right_press(msg)` | Rechtsklick | — | `.contextMenu {}` |
| `.on_enter(msg)` | Hover-Eintritt | `.hoverable()` | `.onHover {}` |
| `.on_exit(msg)` | Hover-Austritt | `.hoverable()` | `.onHover {}` |
| `.on_scroll(fn)` | Scrollrad-Ereignis | `.pointerInput()` | `.onScrollGesture {}` |
| `.on_move(fn)` | Mausbewegungsverfolgung | `.pointerInput()` | `.onContinuousHover {}` |
| `.cursor()` | Cursorstil | `.pointerInput()` | `.cursor()` |

### Extras

| Methode | Beschreibung | Compose | SwiftUI |
|---------|-------------|---------|---------|
| `.tooltip_text()` | Tooltip | `TooltipBox` | `.help()` |
| `.tooltip_gap()` | Abstand Tooltip/Inhalt | — | — |
| `.tooltip_padding()` | Tooltip-Innenabstand | — | — |
| `.tooltip_snap()` | Tooltip im Viewport einrasten | — | — |
| `.scrollable()` | Vertikales Scrollen | `.verticalScroll()` | `ScrollView` |
| `.scrollable_x()` | Horizontales Scrollen | `.horizontalScroll()` | `ScrollView(.horizontal)` |
| `.scrollable_xy()` | Bidirektionales Scrollen | — | — |
| `.scroll_anchor_bottom()` | Scrollen von unten starten | `reverseLayout` | `.defaultScrollAnchor(.bottom)` |
| `.scroll_anchor_right()` | Scrollen von rechts starten | — | `.defaultScrollAnchor(.trailing)` |
| `.scroll_spacing()` | Abstand Scrollleiste/Inhalt | — | — |
| `.scrollable_id()` | Scroll-Widget-ID | — | — |
| `.hidden(bool)` | Sichtbarkeitsumschaltung | `AnimatedVisibility` | `.hidden()` |
| `.id()` | Widget-ID | `.testTag()` | `.id()` |
| `.font_size()` | Schriftgroesse (Widget-Wrapper) | `fontSize` | `.font(.system(size:))` |
| `.spacing()` | Kindabstand (Widget-Wrapper) | `Arrangement.spacedBy()` | `VStack(spacing:)` |

### Komposition und Wiederverwendung

```rust
// Wiederverwendbare Stil-Funktionen (mit Modifier)
fn card_style() -> Modifier {
    Modifier::new()
        .background_color(Color::WHITE)
        .corner_radius(12)
        .padding(16)
}

// Mit .modify() anwenden (abwaertskompatibel)
text("Hello").modify(card_style())

// Bedingte Modifier
Text::new("Status")
    .padding(10)
    .modify_if(is_error, |t| t.background_color(Color::RED))

// Reihenfolgeabhaengiges Layering
Text::new("Ebene")
    .padding(20)
    .layer()  // Aktuelle Ebene flushen
    .background_color(Color::RED)  // Ausserhalb des Paddings angewendet
```

## Feature Flags

```toml
iced_modifier = { version = "0.2", features = ["icons", "drag-drop", "animation"] }
```

| Feature | Crate | Beschreibung |
|---------|-------|-------------|
| `icons` | [iced_fonts](https://crates.io/crates/iced_fonts) | Bootstrap-Icon-Fonts, `icon_label()`-Helper |
| `drag-drop` | [iced_drop](https://crates.io/crates/iced_drop) | `.on_drag()` / `.on_drop()` / `.draggable()`-Erweiterungen |
| `animation` | [iced_anim](https://crates.io/crates/iced_anim) | `iced_anim` Re-Export |
| `image` | iced (image feature) | `Image` / `InteractiveImage` Widget-Wrapper |
| `all` | Alle obigen | Alle Funktionen aktivieren |

## Beispiele

```bash
cargo run --example basic
cargo run --example icons --features icons
cargo run --example drag_drop --features drag-drop
```

## Migration von v0.1

- `use iced::widget::text` -> Entfernen, `text` aus dem Prelude verwenden (Wrapper)
- `use iced::widget::{column, row}` -> `use iced_modifier::{column, row}` Makros verwenden
- `.modify(Modifier::new()...)` -> Funktioniert weiterhin, direkte Verkettung bevorzugt
- Feld `Extras::tooltip_text` -> Umbenannt zu `Extras::tooltip` (Typ: `TooltipConfig`)
- Feld `Extras::scrollable` -> Typ geaendert von `ScrollDirection` zu `ScrollConfig`

## Kompatibilitaet

- **iced**: 0.14.x
- **Rust**: 2024 Edition
- **Plattformen**: Alle von iced unterstuetzten Plattformen (macOS, Windows, Linux, Web)

## Lizenz

MIT-Lizenz. Siehe [LICENSE](../LICENSE) fuer Details.
