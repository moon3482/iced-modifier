//! Modifier-aware Button widget.

use iced::Element;

use crate::modifier::accumulator::Interactions;
use crate::modifier::bundle::{ModifierData, ModifyBase};

use super::{impl_area_interactions, impl_from_element};

pub struct Button<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Theme: iced::widget::button::Catalog,
    Renderer: iced::advanced::Renderer,
{
    inner: iced::widget::Button<'a, Message, Theme, Renderer>,
    data: ModifierData,
    interactions: Interactions<Message>,
}

pub fn button<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Button<'a, Message, Theme, Renderer>
where
    Theme: iced::widget::button::Catalog + 'a,
    Renderer: iced::advanced::Renderer + 'a,
{
    Button::new(content)
}

impl<'a, Message, Theme, Renderer> ModifyBase for Button<'a, Message, Theme, Renderer>
where
    Theme: iced::widget::button::Catalog,
    Renderer: iced::advanced::Renderer,
{
    fn data_mut(&mut self) -> &mut ModifierData {
        &mut self.data
    }
}

impl<'a, Message, Theme, Renderer> Button<'a, Message, Theme, Renderer>
where
    Theme: iced::widget::button::Catalog + 'a,
    Renderer: iced::advanced::Renderer + 'a,
{
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        Self {
            inner: iced::widget::Button::new(content),
            data: ModifierData::default(),
            interactions: Interactions::empty(),
        }
    }

    /// Set click handler (iced Button native — includes press style).
    pub fn on_press(mut self, msg: Message) -> Self {
        self.inner = self.inner.on_press(msg);
        self
    }

    /// Conditionally set click handler.
    pub fn on_press_maybe(mut self, msg: Option<Message>) -> Self {
        self.inner = self.inner.on_press_maybe(msg);
        self
    }

    /// Set iced Button native padding (inside the button).
    pub fn button_padding(mut self, padding: impl Into<iced::Padding>) -> Self {
        self.inner = self.inner.padding(padding);
        self
    }

    /// Set iced Button native width.
    pub fn button_width(mut self, width: impl Into<iced::Length>) -> Self {
        self.inner = self.inner.width(width);
        self
    }

    /// Set iced Button native height.
    pub fn button_height(mut self, height: impl Into<iced::Length>) -> Self {
        self.inner = self.inner.height(height);
        self
    }

    /// Set iced Button clip.
    pub fn button_clip(mut self, clip: bool) -> Self {
        self.inner = self.inner.clip(clip);
        self
    }
}

impl_area_interactions!(Button<'a, Message>, where Theme: iced::widget::button::Catalog,);

impl_from_element!(Button<'a, Message>, |b| {
    (b.inner, b.data, b.interactions)
}, where iced::widget::button::Catalog);
