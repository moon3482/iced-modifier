# iced_modifier

[![Crates.io](https://img.shields.io/crates/v/iced_modifier.svg)](https://crates.io/crates/iced_modifier)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](../LICENSE)

**API modifier style SwiftUI/Jetpack Compose pour [iced](https://github.com/iced-rs/iced)**

Style, mise en page, interactions et proprietes specifiques aux widgets -- le tout dans une API fluide et chainable.

> [English](../README.md)

## Demarrage rapide

```toml
[dependencies]
iced_modifier = "0.4"
```

```rust
use iced_modifier::prelude::*;

// Chainage direct -- style SwiftUI
Text::new("Hello")
    .font_size(16)
    .text_align(TextAlign::Center)
    .padding(12)
    .background_color(Color::WHITE)
    .corner_radius(8)
    .on_press(Message::Clicked)
    .cursor(mouse::Interaction::Pointer)

// Column + spacing en chainage direct
column![
    Text::new("Element A").font_size(14).padding(8),
    Text::new("Element B").font_size(14).padding(8),
]
.spacing(8)
.padding(12)
.background_color(Color::WHITE)

// Couleurs hex -- pas besoin de Color::from_rgb()
Text::new("Alerte")
    .background_color("#FF5733")
    .text_color("#FFF")
    .border_color([1.0, 0.0, 0.0])

// Button avec contenu style
Button::new(Text::new("Envoyer").font_size(14).color("#FFFFFF"))
    .on_press(Message::Submit)
    .padding(12)
    .corner_radius(8)
```

## Pourquoi iced_modifier ?

```rust
// Avant : enrobage Container verbeux + closure de style
container(text("Hello"))
    .padding(16)
    .style(|_| container::Style {
        background: Some(Color::WHITE.into()),
        border: Border { radius: 12.into(), ..Border::default() },
        ..container::Style::default()
    })
    .into()

// Apres : chainage direct
Text::new("Hello")
    .padding(16)
    .background_color(Color::WHITE)
    .corner_radius(12)
```

## Wrappers de widgets

Les 12 principaux widgets iced sont supportes avec chainage direct de modifiers :

| Widget | Modele | Methodes specifiques au widget |
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

**Modele A** : Pas de generique Message. Les methodes d'interaction (`.on_press()`) effectuent une transition vers le type interactif.
**Modele B** : Generique Message via enfants/callbacks. Interactions disponibles directement.

## Types principaux

| Type | Description |
|------|-------------|
| `Text`, `Column`, `Row`, ... | Wrappers de widgets avec modifiers integres. Chainage direct pour style/mise en page/interactions |
| `Modifier` | Specification de style/mise en page reutilisable. Stockable/composable sans generique |
| `Interactor<M>` | Modifier + gestionnaires d'interaction |
| `ModifyBase` trait | Fournit 40+ methodes chainables (wrappers, Modifier, Interactor l'implementent tous) |

## Fonctionnalites

### Style

| Methode | Description | Compose | SwiftUI |
|---------|-------------|---------|---------|
| `.background_color()` | Couleur de fond (Color, hex, tableau) | `.background()` | `.background()` |
| `.border()` | Bordure (Border, tuple, rayon) | `.border()` | `.border()` |
| `.corner_radius()` | Arrondi des coins | `.clip(RoundedCornerShape())` | `.cornerRadius()` |
| `.shadow()` | Ombre portee (Shadow, tuple, flou) | `.shadow()` | `.shadow()` |
| `.text_color()` | Couleur du texte (Color, hex, tableau) | `color` param | `.foregroundColor()` |
| `.text_align()` | Alignement du texte (Start, Center, End, Justify) | `textAlign` | `.multilineTextAlignment()` |

### Formats de couleur

Toutes les methodes de couleur (`.background_color()`, `.text_color()`, `.border_color()`, `.color()`) acceptent :

| Format | Exemple |
|--------|---------|
| `Color` | `Color::WHITE`, `Color::from_rgb(1.0, 0.5, 0.0)` |
| Hex `&str` | `"#FF5733"`, `"#FFF"`, `"3388FF"`, `"#FF573380"` (RGBA) |
| `[f32; 3]` | `[1.0, 0.5, 0.0]` (RGB, 0.0-1.0) |
| `[f32; 4]` | `[1.0, 0.5, 0.0, 0.8]` (RGBA, 0.0-1.0) |

### Formats de bordure

`.border()` accepte :

| Format | Exemple |
|--------|---------|
| `Border` | `Border { color: Color::RED, width: 1.0, radius: 8.0.into() }` |
| `f32` | `8.0` (rayon des coins uniquement) |
| `(Color, f32)` | `(Color::BLACK, 1.0)` (couleur + largeur) |
| `(&str, f32)` | `("#000", 1.0)` (couleur hex + largeur) |
| `(Color, f32, f32)` | `(Color::BLACK, 1.0, 8.0)` (couleur + largeur + rayon) |
| `(&str, f32, f32)` | `("#CCC", 1.0, 8.0)` (hex + largeur + rayon) |

### Formats d'ombre

`.shadow()` accepte :

| Format | Exemple |
|--------|---------|
| `Shadow` | `Shadow { color: Color::BLACK, offset: Vector::new(0.0, 4.0), blur_radius: 8.0 }` |
| `f32` | `8.0` (rayon de flou uniquement, noir, sans decalage) |
| `(f32, f32, f32)` | `(0.0, 4.0, 8.0)` (x, y, flou) |
| `(f32, f32, f32, Color)` | `(0.0, 4.0, 8.0, Color::BLACK)` |
| `(f32, f32, f32, &str)` | `(0.0, 4.0, 8.0, "#00000040")` (avec couleur hex) |

### Mise en page

| Methode | Description | Compose | SwiftUI |
|---------|-------------|---------|---------|
| `.padding()` | Marge interieure | `.padding()` | `.padding()` |
| `.margin()` | Marge exterieure | N/A | N/A |
| `.width()` / `.height()` | Dimensions | `.size()` | `.frame()` |
| `.fill_width()` | Remplir la largeur disponible | `.fillMaxWidth()` | `.frame(maxWidth: .infinity)` |
| `.fill_portion(n)` | Remplissage proportionnel | `.weight()` | N/A |
| `.center()` | Centrer sur les deux axes | `.align(Alignment.Center)` | `.center()` |
| `.align_top()` | Aligner en haut + definir la hauteur | `Alignment.Top` | `.frame(alignment: .top)` |
| `.align_bottom()` | Aligner en bas + definir la hauteur | `Alignment.Bottom` | `.frame(alignment: .bottom)` |
| `.max_width()` / `.max_height()` | Taille maximale | `.requiredSize()` | `.frame(maxWidth:)` |
| `.clip(bool)` | Rognage du debordement | `.clip()` | `.clipped()` |

### Interactions

| Methode | Description | Compose | SwiftUI |
|---------|-------------|---------|---------|
| `.on_press(msg)` | Gestionnaire de clic | `.clickable {}` | `.onTapGesture {}` |
| `.on_release(msg)` | Gestionnaire de relachement | — | — |
| `.on_double_click(msg)` | Double-clic | `.combinedClickable()` | `.onTapGesture(count: 2)` |
| `.on_right_press(msg)` | Clic droit | — | `.contextMenu {}` |
| `.on_enter(msg)` | Entree du survol | `.hoverable()` | `.onHover {}` |
| `.on_exit(msg)` | Sortie du survol | `.hoverable()` | `.onHover {}` |
| `.on_scroll(fn)` | Evenement de molette | `.pointerInput()` | `.onScrollGesture {}` |
| `.on_move(fn)` | Suivi du mouvement souris | `.pointerInput()` | `.onContinuousHover {}` |
| `.cursor()` | Style du curseur | `.pointerInput()` | `.cursor()` |

### Extras

| Methode | Description | Compose | SwiftUI |
|---------|-------------|---------|---------|
| `.tooltip_text()` | Info-bulle | `TooltipBox` | `.help()` |
| `.tooltip_gap()` | Espacement info-bulle/contenu | — | — |
| `.tooltip_padding()` | Marge interieure de l'info-bulle | — | — |
| `.tooltip_snap()` | Accrochage info-bulle dans le viewport | — | — |
| `.scrollable()` | Defilement vertical | `.verticalScroll()` | `ScrollView` |
| `.scrollable_x()` | Defilement horizontal | `.horizontalScroll()` | `ScrollView(.horizontal)` |
| `.scrollable_xy()` | Defilement bidirectionnel | — | — |
| `.scroll_anchor_bottom()` | Defilement depuis le bas | `reverseLayout` | `.defaultScrollAnchor(.bottom)` |
| `.scroll_anchor_right()` | Defilement depuis la droite | — | `.defaultScrollAnchor(.trailing)` |
| `.scroll_spacing()` | Espacement barre de defilement/contenu | — | — |
| `.scrollable_id()` | ID du widget de defilement | — | — |
| `.hidden(bool)` | Bascule de visibilite | `AnimatedVisibility` | `.hidden()` |
| `.id()` | ID du widget | `.testTag()` | `.id()` |
| `.font_size()` | Taille de police (wrappers de widgets) | `fontSize` | `.font(.system(size:))` |
| `.spacing()` | Espacement enfants (wrappers de widgets) | `Arrangement.spacedBy()` | `VStack(spacing:)` |

### Composition et reutilisation

```rust
// Fonctions de style reutilisables (avec Modifier)
fn card_style() -> Modifier {
    Modifier::new()
        .background_color(Color::WHITE)
        .corner_radius(12)
        .padding(16)
}

// Appliquer avec .modify() (retrocompatible)
text("Hello").modify(card_style())

// Modifiers conditionnels
Text::new("Statut")
    .padding(10)
    .modify_if(is_error, |t| t.background_color(Color::RED))

// Superposition dependante de l'ordre
Text::new("Couche")
    .padding(20)
    .layer()  // Flush de la couche actuelle
    .background_color(Color::RED)  // Applique a l'exterieur du padding
```

## Feature Flags

```toml
iced_modifier = { version = "0.2", features = ["icons", "drag-drop", "animation"] }
```

| Feature | Crate | Description |
|---------|-------|-------------|
| `icons` | [iced_fonts](https://crates.io/crates/iced_fonts) | Polices d'icones Bootstrap, helper `icon_label()` |
| `drag-drop` | [iced_drop](https://crates.io/crates/iced_drop) | Extensions `.on_drag()` / `.on_drop()` / `.draggable()` |
| `animation` | [iced_anim](https://crates.io/crates/iced_anim) | Re-export de `iced_anim` |
| `image` | iced (image feature) | Wrappers `Image` / `InteractiveImage` |
| `all` | Tous les precedents | Activer toutes les fonctionnalites |

## Exemples

```bash
cargo run --example basic
cargo run --example showcase
cargo run --example icons --features icons
cargo run --example drag_drop --features drag-drop
```

## Migration depuis v0.1

- `use iced::widget::text` -> Supprimer, utiliser `text` du prelude (wrapper)
- `use iced::widget::{column, row}` -> Utiliser les macros `use iced_modifier::{column, row}`
- `.modify(Modifier::new()...)` -> Fonctionne toujours, mais chainage direct prefere
- Champ `Extras::tooltip_text` -> Renomme en `Extras::tooltip` (type : `TooltipConfig`)
- Champ `Extras::scrollable` -> Type change de `ScrollDirection` a `ScrollConfig`

## Compatibilite

- **iced** : 0.14.x
- **Rust** : 2024 edition
- **Plateformes** : Toutes les plateformes supportees par iced (macOS, Windows, Linux, Web)

## Licence

Licence MIT. Voir [LICENSE](../LICENSE) pour les details.
