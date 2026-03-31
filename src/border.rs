//! Border conversion utilities.
//!
//! Provides [`IntoBorder`] trait that accepts `Border`, `f32` (radius only),
//! or tuples of (color, width) and (color, width, radius).
//!
//! ```ignore
//! use iced_modifier::prelude::*;
//!
//! Text::new("Hello")
//!     .border(8.0)                     // radius only
//!     .border(("#FF5733", 1.0))        // hex color + width
//!     .border((Color::BLACK, 1.0, 8.0)) // color + width + radius
//! ```

use iced::{Border, Color};

use crate::color::IntoColor;

/// Trait for types that can be converted into [`iced::Border`].
///
/// Implemented for:
/// - [`Border`] — identity conversion
/// - [`f32`] — uniform corner radius, no color/width
/// - `(Color, f32)` — color + width
/// - `(&str, f32)` — hex color + width
/// - `(Color, f32, f32)` — color + width + radius
/// - `(&str, f32, f32)` — hex color + width + radius
///
/// Used by the `border()` method on `ModifyBase`.
pub trait IntoBorder {
    fn into_border(self) -> Border;
}

impl IntoBorder for Border {
    fn into_border(self) -> Border {
        self
    }
}

// radius only (uniform corner radius, no color/width)
impl IntoBorder for f32 {
    fn into_border(self) -> Border {
        Border {
            radius: self.into(),
            ..Border::default()
        }
    }
}

// (color as Color, width)
impl IntoBorder for (Color, f32) {
    fn into_border(self) -> Border {
        Border {
            color: self.0,
            width: self.1,
            ..Border::default()
        }
    }
}

// (color as &str hex, width)
impl IntoBorder for (&str, f32) {
    fn into_border(self) -> Border {
        Border {
            color: self.0.into_color(),
            width: self.1,
            ..Border::default()
        }
    }
}

// (color as Color, width, radius)
impl IntoBorder for (Color, f32, f32) {
    fn into_border(self) -> Border {
        Border {
            color: self.0,
            width: self.1,
            radius: self.2.into(),
        }
    }
}

// (color as &str hex, width, radius)
impl IntoBorder for (&str, f32, f32) {
    fn into_border(self) -> Border {
        Border {
            color: self.0.into_color(),
            width: self.1,
            radius: self.2.into(),
        }
    }
}
