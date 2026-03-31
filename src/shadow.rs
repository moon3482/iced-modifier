//! Shadow conversion utilities.
//!
//! Provides [`IntoShadow`] trait that accepts `Shadow`, `f32`, or tuples.
//!
//! ```ignore
//! use iced_modifier::prelude::*;
//!
//! Text::new("Hello")
//!     .shadow(8.0)                           // blur only (black, no offset)
//!     .shadow((0.0, 4.0, 8.0))              // offset + blur (25% black)
//!     .shadow((0.0, 4.0, 8.0, Color::RED))  // offset + blur + color
//!     .shadow((0.0, 4.0, 8.0, "#FF000040")) // offset + blur + hex color
//! ```

use iced::{Color, Shadow, Vector};

use crate::color::IntoColor;

/// Trait for types that can be converted into [`iced::Shadow`].
///
/// Implemented for:
/// - [`Shadow`] — identity conversion
/// - [`f32`] — blur radius only (black shadow, no offset)
/// - `(f32, f32, f32)` — (offset_x, offset_y, blur_radius) with 25% black
/// - `(f32, f32, f32, Color)` — full specification with color
/// - `(f32, f32, f32, &str)` — full specification with hex color string
pub trait IntoShadow {
    fn into_shadow(self) -> Shadow;
}

impl IntoShadow for Shadow {
    fn into_shadow(self) -> Shadow {
        self
    }
}

/// Blur radius only (black shadow, no offset).
impl IntoShadow for f32 {
    fn into_shadow(self) -> Shadow {
        Shadow {
            color: Color::BLACK,
            offset: Vector::ZERO,
            blur_radius: self,
        }
    }
}

/// (offset_x, offset_y, blur_radius) with 25% black default color.
impl IntoShadow for (f32, f32, f32) {
    fn into_shadow(self) -> Shadow {
        Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.25),
            offset: Vector::new(self.0, self.1),
            blur_radius: self.2,
        }
    }
}

/// (offset_x, offset_y, blur_radius, color).
impl IntoShadow for (f32, f32, f32, Color) {
    fn into_shadow(self) -> Shadow {
        Shadow {
            color: self.3,
            offset: Vector::new(self.0, self.1),
            blur_radius: self.2,
        }
    }
}

/// (offset_x, offset_y, blur_radius, hex_color_str).
impl IntoShadow for (f32, f32, f32, &str) {
    fn into_shadow(self) -> Shadow {
        Shadow {
            color: self.3.into_color(),
            offset: Vector::new(self.0, self.1),
            blur_radius: self.2,
        }
    }
}
