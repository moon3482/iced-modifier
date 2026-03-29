//! Modifier-aware TextInput widget.


use iced::advanced::text as advanced_text;
use iced::widget::text_input as iced_text_input;
use iced::Pixels;

use crate::modifier::accumulator::Interactions;
use crate::modifier::bundle::{ModifierData, ModifyBase};

use super::{impl_area_interactions, impl_from_element};

pub struct TextInput<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Theme: iced_text_input::Catalog,
    Renderer: advanced_text::Renderer,
{
    inner: iced::widget::TextInput<'a, Message, Theme, Renderer>,
    data: ModifierData,
    interactions: Interactions<Message>,
}

pub fn text_input<'a, Message, Theme, Renderer>(
    placeholder: &str,
    value: &str,
) -> TextInput<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: iced_text_input::Catalog + 'a,
    Renderer: advanced_text::Renderer + 'a,
{
    TextInput::new(placeholder, value)
}

impl<'a, Message, Theme, Renderer> ModifyBase for TextInput<'a, Message, Theme, Renderer>
where
    Theme: iced_text_input::Catalog,
    Renderer: advanced_text::Renderer,
{
    fn data_mut(&mut self) -> &mut ModifierData { &mut self.data }
}

impl<'a, Message: Clone, Theme, Renderer> TextInput<'a, Message, Theme, Renderer>
where
    Theme: iced_text_input::Catalog + 'a,
    Renderer: advanced_text::Renderer + 'a,
{
    pub fn new(placeholder: &str, value: &str) -> Self {
        Self {
            inner: iced::widget::TextInput::new(placeholder, value),
            data: ModifierData::default(),
            interactions: Interactions::empty(),
        }
    }

    // ── Widget-specific ──

    /// Set font size via modifier system.
    pub fn font_size(mut self, size: impl Into<Pixels>) -> Self {
        self.data.extras.font_size = Some(size.into());
        self
    }

    /// Set input handler.
    pub fn on_input(mut self, f: impl Fn(String) -> Message + 'a) -> Self {
        self.inner = self.inner.on_input(f);
        self
    }

    /// Set submit handler (Enter key).
    pub fn on_submit(mut self, msg: Message) -> Self {
        self.inner = self.inner.on_submit(msg);
        self
    }

    /// Set paste handler.
    pub fn on_paste(mut self, f: impl Fn(String) -> Message + 'a) -> Self {
        self.inner = self.inner.on_paste(f);
        self
    }

    /// Enable secure/password mode.
    pub fn secure(mut self, is_secure: bool) -> Self {
        self.inner = self.inner.secure(is_secure);
        self
    }

    /// Set iced TextInput native font size directly.
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.inner = self.inner.size(size);
        self
    }

    /// Set font.
    pub fn font(mut self, font: Renderer::Font) -> Self {
        self.inner = self.inner.font(font);
        self
    }

    /// Set iced TextInput native padding (inside the input field).
    pub fn input_padding(mut self, padding: impl Into<iced::Padding>) -> Self {
        self.inner = self.inner.padding(padding);
        self
    }

    /// Set widget ID.
    pub fn input_id(mut self, id: impl Into<iced::widget::Id>) -> Self {
        self.inner = self.inner.id(id);
        self
    }
}

impl_area_interactions!(TextInput<'a, Message>, where Theme: iced::widget::text_input::Catalog, Renderer: iced::advanced::text::Renderer,);

impl_from_element!(TextInput<'a, Message>, |t| {
    let mut inner = t.inner;
    if let Some(size) = t.data.extras.font_size {
        inner = inner.size(size);
    }
    (inner, t.data, t.interactions)
}, where iced::widget::text_input::Catalog);
