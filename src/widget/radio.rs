//! Modifier-aware Radio widget.

use iced::Pixels;
use iced::advanced::text as advanced_text;
use iced::widget::{radio as iced_radio, text as iced_text};

use crate::modifier::accumulator::Interactions;
use crate::modifier::bundle::{ModifierData, ModifyBase};

use super::{impl_area_interactions, impl_from_element};

pub struct Radio<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Theme: iced_radio::Catalog + iced_text::Catalog,
    Renderer: advanced_text::Renderer,
{
    inner: iced::widget::Radio<'a, Message, Theme, Renderer>,
    data: ModifierData,
    interactions: Interactions<Message>,
}

pub fn radio<'a, Message, Theme, Renderer, V>(
    label: impl Into<String>,
    value: V,
    selected: Option<V>,
    on_click: impl FnOnce(V) -> Message,
) -> Radio<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: iced_radio::Catalog + iced_text::Catalog + 'a,
    Renderer: advanced_text::Renderer + 'a,
    V: Copy + Eq,
{
    Radio::new(label, value, selected, on_click)
}

impl<'a, Message, Theme, Renderer> ModifyBase for Radio<'a, Message, Theme, Renderer>
where
    Theme: iced_radio::Catalog + iced_text::Catalog,
    Renderer: advanced_text::Renderer,
{
    fn data_mut(&mut self) -> &mut ModifierData {
        &mut self.data
    }
}

impl<'a, Message: Clone, Theme, Renderer> Radio<'a, Message, Theme, Renderer>
where
    Theme: iced_radio::Catalog + iced_text::Catalog + 'a,
    Renderer: advanced_text::Renderer + 'a,
{
    pub fn new<V: Copy + Eq>(
        label: impl Into<String>,
        value: V,
        selected: Option<V>,
        on_click: impl FnOnce(V) -> Message,
    ) -> Self {
        Self {
            inner: iced::widget::Radio::new(label, value, selected, on_click),
            data: ModifierData::default(),
            interactions: Interactions::empty(),
        }
    }

    /// Set radio button size.
    pub fn radio_size(mut self, size: impl Into<Pixels>) -> Self {
        self.inner = self.inner.size(size);
        self
    }

    /// Set spacing between radio and label.
    pub fn radio_spacing(mut self, spacing: impl Into<Pixels>) -> Self {
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
}

impl_area_interactions!(Radio<'a, Message>, where Theme: iced_radio::Catalog + iced_text::Catalog, Renderer: advanced_text::Renderer,);

impl_from_element!(Radio<'a, Message>, |r| {
    (r.inner, r.data, r.interactions)
}, where iced_radio::Catalog);
