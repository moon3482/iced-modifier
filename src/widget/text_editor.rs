//! Modifier-aware TextEditor widget.

use iced::Pixels;
use iced::advanced::text as advanced_text;
use iced::widget::{text as iced_text, text_editor as iced_text_editor};

use crate::modifier::accumulator::Interactions;
use crate::modifier::bundle::{ModifierData, ModifyBase};

use super::{impl_area_interactions, impl_from_element};

/// Modifier-aware TextEditor widget with font_size support.
pub struct TextEditor<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Theme: iced_text_editor::Catalog,
    Renderer: advanced_text::Renderer,
{
    inner: iced::widget::TextEditor<
        'a,
        iced::advanced::text::highlighter::PlainText,
        Message,
        Theme,
        Renderer,
    >,
    data: ModifierData,
    interactions: Interactions<Message>,
}

/// Create a new modifier-aware TextEditor.
pub fn text_editor<'a, Message, Theme, Renderer>(
    content: &'a iced_text_editor::Content<Renderer>,
) -> TextEditor<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: iced_text_editor::Catalog + 'a,
    Renderer: advanced_text::Renderer + 'a,
{
    TextEditor::new(content)
}

impl<'a, Message, Theme, Renderer> ModifyBase for TextEditor<'a, Message, Theme, Renderer>
where
    Theme: iced_text_editor::Catalog,
    Renderer: advanced_text::Renderer,
{
    fn data_mut(&mut self) -> &mut ModifierData {
        &mut self.data
    }
}

impl<'a, Message: Clone, Theme, Renderer> TextEditor<'a, Message, Theme, Renderer>
where
    Theme: iced_text_editor::Catalog + 'a,
    Renderer: advanced_text::Renderer + 'a,
{
    /// Create a new TextEditor with the given content.
    pub fn new(content: &'a iced_text_editor::Content<Renderer>) -> Self {
        Self {
            inner: iced::widget::TextEditor::new(content),
            data: ModifierData::default(),
            interactions: Interactions::empty(),
        }
    }

    /// Set font size via modifier system.
    pub fn font_size(mut self, size: impl Into<Pixels>) -> Self {
        self.data.extras.font_size = Some(size.into());
        self
    }

    /// Set action handler.
    pub fn on_action(mut self, f: impl Fn(iced_text_editor::Action) -> Message + 'a) -> Self {
        self.inner = self.inner.on_action(f);
        self
    }

    /// Set iced TextEditor native font size.
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.inner = self.inner.size(size);
        self
    }

    /// Set font.
    pub fn font(mut self, font: impl Into<Renderer::Font>) -> Self {
        self.inner = self.inner.font(font);
        self
    }

    /// Set iced TextEditor native padding.
    pub fn editor_padding(mut self, padding: impl Into<iced::Padding>) -> Self {
        self.inner = self.inner.padding(padding);
        self
    }

    /// Set wrapping strategy.
    pub fn wrapping(mut self, wrapping: iced_text::Wrapping) -> Self {
        self.inner = self.inner.wrapping(wrapping);
        self
    }

    /// Set iced TextEditor native width.
    pub fn editor_width(mut self, width: impl Into<Pixels>) -> Self {
        self.inner = self.inner.width(width);
        self
    }

    /// Set iced TextEditor native height.
    pub fn editor_height(mut self, height: impl Into<iced::Length>) -> Self {
        self.inner = self.inner.height(height);
        self
    }
}

impl_area_interactions!(TextEditor<'a, Message>, where Theme: iced_text_editor::Catalog, Renderer: advanced_text::Renderer,);

impl_from_element!(TextEditor<'a, Message>, |t| {
    let mut inner = t.inner;
    if let Some(size) = t.data.extras.font_size {
        inner = inner.size(size);
    }
    (inner, t.data, t.interactions)
}, where iced_text_editor::Catalog);
