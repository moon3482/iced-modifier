# iced_modifier

[![Crates.io](https://img.shields.io/crates/v/iced_modifier.svg)](https://crates.io/crates/iced_modifier)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](../LICENSE)

**API modifier estilo SwiftUI/Jetpack Compose para [iced](https://github.com/iced-rs/iced)**

Estilos, disposicion, interacciones y propiedades especificas de widgets -- todo en una API fluida y encadenable.

> [English](../README.md)

## Inicio rapido

```toml
[dependencies]
iced_modifier = "0.4"
```

```rust
use iced_modifier::prelude::*;

// Encadenamiento directo -- estilo SwiftUI
Text::new("Hello")
    .font_size(16)
    .text_align(TextAlign::Center)
    .padding(12)
    .background_color(Color::WHITE)
    .corner_radius(8)
    .on_press(Message::Clicked)
    .cursor(mouse::Interaction::Pointer)

// Column + spacing encadenamiento directo
column![
    Text::new("Elemento A").font_size(14).padding(8),
    Text::new("Elemento B").font_size(14).padding(8),
]
.spacing(8)
.padding(12)
.background_color(Color::WHITE)

// Colores hex -- sin necesidad de Color::from_rgb()
Text::new("Alerta")
    .background_color("#FF5733")
    .text_color("#FFF")
    .border_color([1.0, 0.0, 0.0])

// Button con contenido estilizado
Button::new(Text::new("Enviar").font_size(14).color("#FFFFFF"))
    .on_press(Message::Submit)
    .padding(12)
    .corner_radius(8)
```

## Por que iced_modifier?

```rust
// Antes: envoltorio Container verboso + closure de estilo
container(text("Hello"))
    .padding(16)
    .style(|_| container::Style {
        background: Some(Color::WHITE.into()),
        border: Border { radius: 12.into(), ..Border::default() },
        ..container::Style::default()
    })
    .into()

// Despues: encadenamiento directo
Text::new("Hello")
    .padding(16)
    .background_color(Color::WHITE)
    .corner_radius(12)
```

## Wrappers de widgets

Los 12 principales widgets de iced tienen soporte de encadenamiento directo de modifiers:

| Widget | Patron | Metodos especificos del widget |
|--------|--------|-------------------------------|
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

**Patron A**: Sin generico Message. Los metodos de interaccion (`.on_press()`) transicionan al tipo interactivo.
**Patron B**: Generico Message por hijos/callbacks. Interacciones disponibles directamente.

## Tipos principales

| Tipo | Descripcion |
|------|-------------|
| `Text`, `Column`, `Row`, ... | Wrappers de widgets con modifiers integrados. Encadenamiento directo para estilo/disposicion/interacciones |
| `Modifier` | Especificacion de estilo/disposicion reutilizable. Almacenable/componible sin genericos |
| `Interactor<M>` | Modifier + manejadores de interaccion |
| `ModifyBase` trait | Proporciona 40+ metodos encadenables (wrappers, Modifier, Interactor lo implementan) |

## Funcionalidades

### Estilos

| Metodo | Descripcion | Compose | SwiftUI |
|--------|-------------|---------|---------|
| `.background_color()` | Color de fondo (Color, hex, array) | `.background()` | `.background()` |
| `.border()` | Borde (Border, tupla, radio) | `.border()` | `.border()` |
| `.corner_radius()` | Redondeo de esquinas | `.clip(RoundedCornerShape())` | `.cornerRadius()` |
| `.shadow()` | Sombra (Shadow, tupla, desenfoque) | `.shadow()` | `.shadow()` |
| `.text_color()` | Color del texto (Color, hex, array) | `color` param | `.foregroundColor()` |
| `.text_align()` | Alineacion de texto (Start, Center, End, Justify) | `textAlign` | `.multilineTextAlignment()` |

### Formatos de color

Todos los metodos de color (`.background_color()`, `.text_color()`, `.border_color()`, `.color()`) aceptan:

| Formato | Ejemplo |
|---------|---------|
| `Color` | `Color::WHITE`, `Color::from_rgb(1.0, 0.5, 0.0)` |
| Hex `&str` | `"#FF5733"`, `"#FFF"`, `"3388FF"`, `"#FF573380"` (RGBA) |
| `[f32; 3]` | `[1.0, 0.5, 0.0]` (RGB, 0.0-1.0) |
| `[f32; 4]` | `[1.0, 0.5, 0.0, 0.8]` (RGBA, 0.0-1.0) |

### Formatos de borde

`.border()` acepta:

| Formato | Ejemplo |
|---------|---------|
| `Border` | `Border { color: Color::RED, width: 1.0, radius: 8.0.into() }` |
| `f32` | `8.0` (solo radio de esquina) |
| `(Color, f32)` | `(Color::BLACK, 1.0)` (color + ancho) |
| `(&str, f32)` | `("#000", 1.0)` (color hex + ancho) |
| `(Color, f32, f32)` | `(Color::BLACK, 1.0, 8.0)` (color + ancho + radio) |
| `(&str, f32, f32)` | `("#CCC", 1.0, 8.0)` (hex + ancho + radio) |

### Formatos de sombra

`.shadow()` acepta:

| Formato | Ejemplo |
|---------|---------|
| `Shadow` | `Shadow { color: Color::BLACK, offset: Vector::new(0.0, 4.0), blur_radius: 8.0 }` |
| `f32` | `8.0` (solo radio de desenfoque, negro, sin desplazamiento) |
| `(f32, f32, f32)` | `(0.0, 4.0, 8.0)` (x, y, desenfoque) |
| `(f32, f32, f32, Color)` | `(0.0, 4.0, 8.0, Color::BLACK)` |
| `(f32, f32, f32, &str)` | `(0.0, 4.0, 8.0, "#00000040")` (con color hex) |

### Disposicion

| Metodo | Descripcion | Compose | SwiftUI |
|--------|-------------|---------|---------|
| `.padding()` | Relleno interior | `.padding()` | `.padding()` |
| `.margin()` | Margen exterior | N/A | N/A |
| `.width()` / `.height()` | Dimensiones | `.size()` | `.frame()` |
| `.fill_width()` | Llenar ancho disponible | `.fillMaxWidth()` | `.frame(maxWidth: .infinity)` |
| `.fill_portion(n)` | Llenado proporcional | `.weight()` | N/A |
| `.center()` | Centrar en ambos ejes | `.align(Alignment.Center)` | `.center()` |
| `.align_top()` | Alinear arriba + establecer altura | `Alignment.Top` | `.frame(alignment: .top)` |
| `.align_bottom()` | Alinear abajo + establecer altura | `Alignment.Bottom` | `.frame(alignment: .bottom)` |
| `.max_width()` / `.max_height()` | Tamano maximo | `.requiredSize()` | `.frame(maxWidth:)` |
| `.clip(bool)` | Recorte de desbordamiento | `.clip()` | `.clipped()` |

### Interacciones

| Metodo | Descripcion | Compose | SwiftUI |
|--------|-------------|---------|---------|
| `.on_press(msg)` | Manejador de clic | `.clickable {}` | `.onTapGesture {}` |
| `.on_release(msg)` | Manejador de liberacion | — | — |
| `.on_double_click(msg)` | Doble clic | `.combinedClickable()` | `.onTapGesture(count: 2)` |
| `.on_right_press(msg)` | Clic derecho | — | `.contextMenu {}` |
| `.on_enter(msg)` | Entrada de hover | `.hoverable()` | `.onHover {}` |
| `.on_exit(msg)` | Salida de hover | `.hoverable()` | `.onHover {}` |
| `.on_scroll(fn)` | Evento de rueda de scroll | `.pointerInput()` | `.onScrollGesture {}` |
| `.on_move(fn)` | Seguimiento de movimiento del raton | `.pointerInput()` | `.onContinuousHover {}` |
| `.cursor()` | Estilo del cursor | `.pointerInput()` | `.cursor()` |

### Extras

| Metodo | Descripcion | Compose | SwiftUI |
|--------|-------------|---------|---------|
| `.tooltip_text()` | Tooltip | `TooltipBox` | `.help()` |
| `.tooltip_gap()` | Espacio tooltip/contenido | — | — |
| `.tooltip_padding()` | Relleno interior del tooltip | — | — |
| `.tooltip_snap()` | Ajuste del tooltip en el viewport | — | — |
| `.scrollable()` | Desplazamiento vertical | `.verticalScroll()` | `ScrollView` |
| `.scrollable_x()` | Desplazamiento horizontal | `.horizontalScroll()` | `ScrollView(.horizontal)` |
| `.scrollable_xy()` | Desplazamiento bidireccional | — | — |
| `.scroll_anchor_bottom()` | Iniciar desplazamiento desde abajo | `reverseLayout` | `.defaultScrollAnchor(.bottom)` |
| `.scroll_anchor_right()` | Iniciar desplazamiento desde la derecha | — | `.defaultScrollAnchor(.trailing)` |
| `.scroll_spacing()` | Espacio barra de desplazamiento/contenido | — | — |
| `.scrollable_id()` | ID del widget de desplazamiento | — | — |
| `.hidden(bool)` | Alternar visibilidad | `AnimatedVisibility` | `.hidden()` |
| `.id()` | ID del widget | `.testTag()` | `.id()` |
| `.font_size()` | Tamano de fuente (wrappers de widgets) | `fontSize` | `.font(.system(size:))` |
| `.spacing()` | Espaciado entre hijos (wrappers de widgets) | `Arrangement.spacedBy()` | `VStack(spacing:)` |

### Composicion y reutilizacion

```rust
// Funciones de estilo reutilizables (con Modifier)
fn card_style() -> Modifier {
    Modifier::new()
        .background_color(Color::WHITE)
        .corner_radius(12)
        .padding(16)
}

// Aplicar con .modify() (retrocompatible)
text("Hello").modify(card_style())

// Modifiers condicionales
Text::new("Estado")
    .padding(10)
    .modify_if(is_error, |t| t.background_color(Color::RED))

// Capas dependientes del orden
Text::new("Capa")
    .padding(20)
    .layer()  // Flush de la capa actual
    .background_color(Color::RED)  // Se aplica fuera del padding
```

## Feature Flags

```toml
iced_modifier = { version = "0.2", features = ["icons", "drag-drop", "animation"] }
```

| Feature | Crate | Descripcion |
|---------|-------|-------------|
| `icons` | [iced_fonts](https://crates.io/crates/iced_fonts) | Fuentes de iconos Bootstrap, helper `icon_label()` |
| `drag-drop` | [iced_drop](https://crates.io/crates/iced_drop) | Extensiones `.on_drag()` / `.on_drop()` / `.draggable()` |
| `animation` | [iced_anim](https://crates.io/crates/iced_anim) | Re-export de `iced_anim` |
| `image` | iced (image feature) | Wrappers `Image` / `InteractiveImage` |
| `all` | Todos los anteriores | Activar todas las funcionalidades |

## Ejemplos

```bash
cargo run --example basic
cargo run --example showcase
cargo run --example icons --features icons
cargo run --example drag_drop --features drag-drop
```

## Migracion desde v0.1

- `use iced::widget::text` -> Eliminar, usar `text` del prelude (wrapper)
- `use iced::widget::{column, row}` -> Usar macros `use iced_modifier::{column, row}`
- `.modify(Modifier::new()...)` -> Sigue funcionando, pero se prefiere encadenamiento directo
- Campo `Extras::tooltip_text` -> Renombrado a `Extras::tooltip` (tipo: `TooltipConfig`)
- Campo `Extras::scrollable` -> Tipo cambiado de `ScrollDirection` a `ScrollConfig`

## Compatibilidad

- **iced**: 0.14.x
- **Rust**: 2024 edition
- **Plataformas**: Todas las plataformas soportadas por iced (macOS, Windows, Linux, Web)

## Licencia

Licencia MIT. Ver [LICENSE](../LICENSE) para mas detalles.
