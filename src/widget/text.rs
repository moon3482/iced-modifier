//! Modifier-aware Text widget wrapper.
//!
//! Wraps [`iced::widget::Text`] to support `font_size` in the modifier chain.
//!
//! # Example
//! ```ignore
//! use iced_modifier::prelude::*;
//!
//! text("Hello").modify(
//!     Modifier::new()
//!         .font_size(16)
//!         .padding(12)
//!         .background_color(Color::WHITE)
//! )
//! ```

use iced::advanced::text as advanced_text;
use iced::widget::text as iced_text;
use iced::{alignment, Color, Element, Length, Pixels};

use crate::modifier::build::build_element;
use crate::IntoModified;

/// Modifier-aware Text widget.
///
/// Wraps [`iced::widget::Text`] and provides an inherent `.modify()` method
/// that extracts `font_size` from the modifier and applies it before Element conversion.
///
/// All standard [`iced::widget::Text`] methods are available through forwarding.
pub struct Text<'a, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Theme: iced_text::Catalog,
    Renderer: advanced_text::Renderer,
{
    inner: iced::widget::Text<'a, Theme, Renderer>,
}

/// Create a new modifier-aware [`Text`] widget.
///
/// This shadows [`iced::widget::text`] when imported via `iced_modifier::prelude::*`.
pub fn text<'a, Theme, Renderer>(
    content: impl iced::widget::text::IntoFragment<'a>,
) -> Text<'a, Theme, Renderer>
where
    Theme: iced_text::Catalog + 'a,
    Renderer: advanced_text::Renderer,
{
    Text::new(content)
}

impl<'a, Theme, Renderer> Text<'a, Theme, Renderer>
where
    Theme: iced_text::Catalog + 'a,
    Renderer: advanced_text::Renderer,
{
    /// Create a new modifier-aware Text widget.
    pub fn new(content: impl iced::widget::text::IntoFragment<'a>) -> Self {
        Self {
            inner: iced::widget::Text::new(content),
        }
    }

    // ── Forwarded iced::widget::Text methods ──

    /// Set the font size of the text.
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.inner = self.inner.size(size);
        self
    }

    /// Set the line height of the text.
    pub fn line_height(mut self, line_height: impl Into<iced::widget::text::LineHeight>) -> Self {
        self.inner = self.inner.line_height(line_height);
        self
    }

    /// Set the font of the text.
    pub fn font(mut self, font: impl Into<Renderer::Font>) -> Self {
        self.inner = self.inner.font(font);
        self
    }

    /// Set the width of the text boundaries.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.inner = self.inner.width(width);
        self
    }

    /// Set the height of the text boundaries.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.inner = self.inner.height(height);
        self
    }

    /// Center the text horizontally and vertically.
    pub fn center(self) -> Self {
        self.align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
    }

    /// Set horizontal alignment.
    pub fn align_x(mut self, alignment: impl Into<iced::widget::text::Alignment>) -> Self {
        self.inner = self.inner.align_x(alignment);
        self
    }

    /// Set vertical alignment.
    pub fn align_y(mut self, alignment: impl Into<alignment::Vertical>) -> Self {
        self.inner = self.inner.align_y(alignment);
        self
    }

    /// Set the shaping strategy.
    pub fn shaping(mut self, shaping: iced::widget::text::Shaping) -> Self {
        self.inner = self.inner.shaping(shaping);
        self
    }

    /// Set the wrapping strategy.
    pub fn wrapping(mut self, wrapping: iced::widget::text::Wrapping) -> Self {
        self.inner = self.inner.wrapping(wrapping);
        self
    }

    /// Set the text color.
    pub fn color(self, color: impl Into<Color>) -> Self
    where
        <Theme as iced_text::Catalog>::Class<'a>:
            From<iced::widget::text::StyleFn<'a, Theme>>,
    {
        Self {
            inner: self.inner.color(color),
        }
    }

    // ── Inherent modify — shadows Modify trait's blanket impl ──

    /// Apply a modifier to this Text widget.
    ///
    /// If the modifier contains `font_size`, it is applied to the Text widget
    /// before wrapping with Container layers.
    ///
    /// This inherent method takes priority over the blanket `Modify` trait implementation,
    /// enabling widget-specific property extraction.
    pub fn modify<Message>(
        self,
        modifier: impl IntoModified<Message>,
    ) -> Element<'a, Message, Theme, Renderer>
    where
        Message: Clone + 'a,
        Theme: iced::widget::container::Catalog
            + iced::widget::scrollable::Catalog
            + iced_text::Catalog
            + 'a,
        Renderer: iced::advanced::Renderer + advanced_text::Renderer + 'a,
        <Theme as iced::widget::container::Catalog>::Class<'a>:
            From<iced::widget::container::StyleFn<'a, Theme>>,
    {
        let (layers, extras, interactions) = modifier.into_parts();

        // Apply font_size to the Text widget before Element conversion
        let mut inner = self.inner;
        if let Some(size) = extras.font_size {
            inner = inner.size(size);
        }

        let element: Element<'a, Message, Theme, Renderer> = inner.into();

        // Early return if no modifications
        if layers.is_empty()
            && !extras.hidden
            && !interactions.has_content()
            && extras.tooltip.is_none()
            && extras.scrollable.is_none()
            && extras.widget_id.is_none()
        {
            return element;
        }

        build_element(element, &layers, extras, interactions)
    }
}

/// Convert to Element for use without `.modify()`.
impl<'a, Message, Theme, Renderer> From<Text<'a, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: iced_text::Catalog + 'a,
    Renderer: advanced_text::Renderer + 'a,
{
    fn from(text: Text<'a, Theme, Renderer>) -> Self {
        text.inner.into()
    }
}
