//! Modifier-aware Toggler widget.


use iced::advanced::text as advanced_text;
use iced::widget::{text as iced_text, toggler as iced_toggler};
use iced::Pixels;

use crate::modifier::accumulator::Interactions;
use crate::modifier::bundle::{ModifierData, ModifyBase};

use super::{impl_area_interactions, impl_from_element};

pub struct Toggler<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Theme: iced_toggler::Catalog + iced_text::Catalog,
    Renderer: advanced_text::Renderer,
{
    inner: iced::widget::Toggler<'a, Message, Theme, Renderer>,
    data: ModifierData,
    interactions: Interactions<Message>,
}

pub fn toggler<'a, Message, Theme, Renderer>(
    is_toggled: bool,
) -> Toggler<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: iced_toggler::Catalog + iced_text::Catalog + 'a,
    Renderer: advanced_text::Renderer + 'a,
{
    Toggler::new(is_toggled)
}

impl<'a, Message, Theme, Renderer> ModifyBase for Toggler<'a, Message, Theme, Renderer>
where
    Theme: iced_toggler::Catalog + iced_text::Catalog,
    Renderer: advanced_text::Renderer,
{
    fn data_mut(&mut self) -> &mut ModifierData { &mut self.data }
}

impl<'a, Message: Clone, Theme, Renderer> Toggler<'a, Message, Theme, Renderer>
where
    Theme: iced_toggler::Catalog + iced_text::Catalog + 'a,
    Renderer: advanced_text::Renderer + 'a,
{
    pub fn new(is_toggled: bool) -> Self {
        Self {
            inner: iced::widget::Toggler::new(is_toggled),
            data: ModifierData::default(),
            interactions: Interactions::empty(),
        }
    }

    /// Set label text.
    pub fn label(mut self, label: impl iced_text::IntoFragment<'a>) -> Self {
        self.inner = self.inner.label(label);
        self
    }

    /// Set toggle handler.
    pub fn on_toggle(mut self, f: impl Fn(bool) -> Message + 'a) -> Self {
        self.inner = self.inner.on_toggle(f);
        self
    }

    /// Set toggler size.
    pub fn toggler_size(mut self, size: impl Into<Pixels>) -> Self {
        self.inner = self.inner.size(size);
        self
    }

    /// Set spacing between toggler and label.
    pub fn toggler_spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.inner = self.inner.spacing(spacing);
        self
    }

    /// Set label text size.
    pub fn text_size(mut self, size: impl Into<Pixels>) -> Self {
        self.inner = self.inner.text_size(size);
        self
    }

    /// Set font.
    pub fn font(mut self, font: impl Into<Renderer::Font>) -> Self {
        self.inner = self.inner.font(font);
        self
    }

    /// Set toggler width.
    pub fn toggler_width(mut self, width: impl Into<iced::Length>) -> Self {
        self.inner = self.inner.width(width);
        self
    }
}

impl_area_interactions!(Toggler<'a, Message>, where Theme: iced::widget::toggler::Catalog + iced::widget::text::Catalog, Renderer: iced::advanced::text::Renderer,);

impl_from_element!(Toggler<'a, Message>, |t| {
    (t.inner, t.data, t.interactions)
}, where iced::widget::toggler::Catalog);
