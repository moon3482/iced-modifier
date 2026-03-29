//! # iced_modifier
//!
//! SwiftUI/Jetpack Compose-style modifier API for [iced](https://github.com/iced-rs/iced).
//!
//! iced_modifier는 iced 위젯에 체이닝 방식으로 스타일, 레이아웃, 인터랙션을 적용할 수 있는
//! modifier API를 제공합니다.
//!
//! ## Quick Start
//!
//! ```ignore
//! use iced::widget::text;
//! use iced::Color;
//! use iced_modifier::prelude::*;
//!
//! // Pure styling — returns Element directly, no .into() needed
//! let card = text("Hello").modify(
//!     Modifier::new()
//!         .padding(16)
//!         .background_color(Color::WHITE)
//!         .corner_radius(12)
//! );
//!
//! // With interactions
//! let clickable = text("Click me").modify(
//!     Modifier::new()
//!         .padding(12)
//!         .on_press(Message::Clicked)
//!         .cursor(mouse::Interaction::Pointer)
//! );
//! ```
//!
//! ## Core Types
//!
//! - [`Modifier`] — Composable styling and layout specification (no generics needed).
//! - [`Interactor`] — Modifier with interaction handlers. Created by calling
//!   `.on_press()`, `.on_enter()`, etc. on a `Modifier`.
//! - [`Modify`] — Extension trait adding `.modify()` to all iced widgets.
//! - [`ModifyBase`] — Trait providing all style/layout/extras methods.
//!   Implemented by both `Modifier` and `Interactor`.
//!
//! ## Feature Flags
//!
//! | Feature | Crate | Provides |
//! |---------|-------|----------|
//! | `icons` | `iced_fonts` | Bootstrap icon fonts, `icon_label()` helper |
//! | `drag-drop` | `iced_drop` | `DragExt` trait — `.on_drag()` / `.on_drop()` on Element |
//! | `animation` | `iced_anim` | Re-exports `iced_anim` |

pub mod modifier;
pub mod widget;

pub use modifier::{Interactor, IntoModified, Modifier, Modify, ModifyBase, modify};

/// Prelude module — import everything needed with `use iced_modifier::prelude::*`.
///
/// Includes [`Modifier`], [`Interactor`], [`Modify`], [`ModifyBase`], and [`modify`].
pub mod prelude {
    pub use crate::modifier::{Interactor, IntoModified, Modifier, Modify, ModifyBase, modify};
    pub use crate::widget::{Text, text};
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
