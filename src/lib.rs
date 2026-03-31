//! # iced_modifier
//!
//! SwiftUI/Jetpack Compose-style modifier API for [iced](https://github.com/iced-rs/iced).
//!
//! Provides modifier-aware widget wrappers with direct chaining for styling,
//! layout, and interaction — no generics, no `.modify()` needed.
//!
//! ## Quick Start
//!
//! ```ignore
//! use iced_modifier::prelude::*;
//!
//! // Direct chaining — SwiftUI/Compose style
//! Text::new("Hello")
//!     .font_size(16)
//!     .padding(12)
//!     .background_color("#FF5733")
//!     .corner_radius(8)
//! ```
//!
//! ## Core Types
//!
//! - [`widget::Text`], [`widget::Column`], [`widget::Row`], [`widget::Button`],
//!   [`widget::Checkbox`], [`widget::Radio`], [`widget::Slider`],
//!   [`widget::TextInput`], [`widget::TextEditor`], [`widget::Toggler`],
//!   [`widget::PickList`] — Modifier-aware widget wrappers with direct chaining.
//! - [`Modifier`] — Reusable style/layout specification (no generics).
//! - [`Interactor`] — Modifier with interaction handlers. Created by calling
//!   `.on_press()`, `.on_enter()`, etc. on a `Modifier`.
//! - [`ModifyBase`] — Trait providing 40+ chainable methods.
//!   Implemented by `Modifier`, `Interactor`, and all widget wrappers.
//! - [`IntoColor`] — Trait for hex string and array color values.
//! - [`IntoBorder`] — Trait for tuple shortcuts in border styling.
//! - [`IntoShadow`] — Trait for tuple shortcuts in shadow styling.
//!
//! ## Feature Flags
//!
//! | Feature | Crate | Provides |
//! |---------|-------|----------|
//! | `icons` | `iced_fonts` | Bootstrap icon fonts, `icon_label()` helper |
//! | `image` | `iced` (image) | `Image` widget wrapper |
//! | `drag-drop` | `iced_drop` | `DragExt` trait — `.on_drag()` / `.on_drop()` on Element |
//! | `animation` | `iced_anim` | Re-exports `iced_anim` |

pub mod border;
pub mod color;
pub mod modifier;
pub mod shadow;
pub mod text_align;
pub mod widget;

pub use border::IntoBorder;
pub use color::IntoColor;
pub use modifier::{Interactor, IntoModified, Modifier, Modify, ModifyBase, modify};
pub use shadow::IntoShadow;
pub use text_align::{TextAlign, TextAlignY};

/// Prelude module — import everything needed with `use iced_modifier::prelude::*`.
///
/// Re-exports:
/// - **Widget wrappers**: `Text`, `Button`, `Column`, `Row`, `Checkbox`,
///   `Radio`, `Slider`, `TextInput`, `TextEditor`, `Toggler`, `PickList`
///   (and `Image` with the `image` feature).
/// - **Convenience constructors**: `text()`, `button()`, `checkbox()`, `radio()`,
///   `slider()`, `text_input()`, `text_editor()`, `toggler()`
///   (and `image()` with the `image` feature).
/// - **Modifier types**: `Modifier`, `Interactor`, `Modify`, `ModifyBase`,
///   `IntoModified`, `modify()`.
/// - **Color**: `IntoColor`.
/// - **Border**: `IntoBorder`.
/// - **Shadow**: `IntoShadow`.
/// - **Macros**: `column!`, `row!`.
pub mod prelude {
    pub use crate::border::IntoBorder;
    pub use crate::color::IntoColor;
    pub use crate::modifier::{Interactor, IntoModified, Modifier, Modify, ModifyBase, modify};
    pub use crate::shadow::IntoShadow;
    pub use crate::text_align::{TextAlign, TextAlignY};
    pub use crate::widget::{
        Button, Checkbox, Column, PickList, Radio, Row, Slider, Text, TextEditor, TextInput,
        Toggler, button, checkbox, radio, slider, text, text_editor, text_input, toggler,
    };
    #[cfg(feature = "image")]
    pub use crate::widget::{Image, image};
    pub use crate::{column, row};
}

/// Icon font utilities (requires `icons` feature).
///
/// Re-exports `iced_fonts::bootstrap` and provides [`icons::icon_label`] and
/// [`icons::load_icon_fonts`] helpers.
#[cfg(feature = "icons")]
pub mod icons;

/// Re-exports `iced_anim` for animation support (requires `animation` feature).
#[cfg(feature = "animation")]
pub use iced_anim;

/// Drag-and-drop extension trait for Element (requires `drag-drop` feature).
///
/// Provides [`drag::DragExt`] with `.on_drag()`, `.on_drop()`, `.draggable()`.
#[cfg(feature = "drag-drop")]
pub mod drag;
