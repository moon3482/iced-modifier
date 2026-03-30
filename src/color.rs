//! Color conversion utilities.
//!
//! Provides [`IntoColor`] trait that accepts `Color`, hex `&str`, or float arrays.
//!
//! ```ignore
//! use iced_modifier::prelude::*;
//!
//! Text::new("Hello")
//!     .background_color("#FF5733")     // hex string
//!     .text_color(Color::WHITE)        // iced Color
//!     .border_color([1.0, 0.0, 0.0])  // RGB array
//! ```

use iced::Color;

/// Trait for types that can be converted into [`iced::Color`].
///
/// Implemented for:
/// - [`Color`] — identity conversion
/// - [`&str`] — hex color parsing (`"#RGB"`, `"#RRGGBB"`, `"#RRGGBBAA"`, with or without `#`)
/// - `[f32; 3]` — RGB values (0.0–1.0)
/// - `[f32; 4]` — RGBA values (0.0–1.0)
pub trait IntoColor {
    fn into_color(self) -> Color;
}

impl IntoColor for Color {
    fn into_color(self) -> Color {
        self
    }
}

impl IntoColor for &str {
    fn into_color(self) -> Color {
        self.parse()
            .unwrap_or_else(|e| panic!("invalid hex color \"{self}\": {e}"))
    }
}

impl IntoColor for [f32; 3] {
    fn into_color(self) -> Color {
        Color::from_rgb(self[0], self[1], self[2])
    }
}

impl IntoColor for [f32; 4] {
    fn into_color(self) -> Color {
        Color::from_rgba(self[0], self[1], self[2], self[3])
    }
}
