//! Text alignment enums for horizontal and vertical text positioning.
//!
//! ```ignore
//! use iced_modifier::prelude::*;
//!
//! Text::new("Hello World")
//!     .text_align(TextAlign::Center)        // horizontal
//!     .text_align_y(TextAlignY::Bottom)     // vertical
//! ```

use iced::alignment;
use iced::widget::text as iced_text;

/// Horizontal text alignment within text boundaries.
///
/// Controls how lines of text align within the text widget's width.
/// This is **not** the same as container alignment ([`ModifyBase::align_x`]),
/// which positions the widget within its parent.
///
/// **Prefer `Start`/`End` over `Left`/`Right`** for RTL language support.
///
/// Compose: `textAlign = TextAlign.Center`
/// SwiftUI: `.multilineTextAlignment(.center)`
/// Flutter: `textAlign: TextAlign.center`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextAlign {
    /// Leading edge (Left in LTR, Right in RTL). **Recommended.**
    /// Compose: `TextAlign.Start` / SwiftUI: `.leading` / Flutter: `TextAlign.start`
    Start,
    /// Center text horizontally.
    /// Compose: `TextAlign.Center` / SwiftUI: `.center` / Flutter: `TextAlign.center`
    Center,
    /// Trailing edge (Right in LTR, Left in RTL). **Recommended.**
    /// Compose: `TextAlign.End` / SwiftUI: `.trailing` / Flutter: `TextAlign.end`
    End,
    /// Justify text across the full width.
    /// Compose: `TextAlign.Justify` / Flutter: `TextAlign.justify`
    Justify,
    /// Always left regardless of text direction. Prefer [`Start`](TextAlign::Start).
    Left,
    /// Always right regardless of text direction. Prefer [`End`](TextAlign::End).
    Right,
}

impl From<TextAlign> for iced_text::Alignment {
    fn from(align: TextAlign) -> Self {
        match align {
            TextAlign::Start => iced_text::Alignment::Default,
            TextAlign::Center => iced_text::Alignment::Center,
            TextAlign::End => iced_text::Alignment::Right,
            TextAlign::Justify => iced_text::Alignment::Justified,
            TextAlign::Left => iced_text::Alignment::Left,
            TextAlign::Right => iced_text::Alignment::Right,
        }
    }
}

/// Vertical text alignment within text boundaries.
///
/// Only effective when [`text_height()`] is set larger than the text content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextAlignY {
    /// Align text to the top.
    Top,
    /// Center text vertically.
    Center,
    /// Align text to the bottom.
    Bottom,
}

impl From<TextAlignY> for alignment::Vertical {
    fn from(align: TextAlignY) -> Self {
        match align {
            TextAlignY::Top => alignment::Vertical::Top,
            TextAlignY::Center => alignment::Vertical::Center,
            TextAlignY::Bottom => alignment::Vertical::Bottom,
        }
    }
}
