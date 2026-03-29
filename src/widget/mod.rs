//! Wrapped iced widgets with modifier-aware capabilities.
//!
//! These types wrap iced's native widgets to support widget-specific properties
//! (like `font_size`, `spacing`) through the modifier chain.

mod text;
mod column;
mod row;

pub use text::{Text, InteractiveText, text};
pub use column::Column;
pub use row::Row;
