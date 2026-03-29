//! Modifier-aware Checkbox widget.


use iced::advanced::text as advanced_text;
use iced::widget::{text as iced_text, checkbox as iced_checkbox};
use iced::Pixels;

use crate::modifier::accumulator::Interactions;
use crate::modifier::bundle::{ModifierData, ModifyBase};

use super::{impl_area_interactions, impl_from_element};

pub struct Checkbox<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Theme: iced_checkbox::Catalog + iced_text::Catalog,
    Renderer: advanced_text::Renderer,
{
    inner: iced::widget::Checkbox<'a, Message, Theme, Renderer>,
    data: ModifierData,
    interactions: Interactions<Message>,
}

pub fn checkbox<'a, Message, Theme, Renderer>(
    is_checked: bool,
) -> Checkbox<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: iced_checkbox::Catalog + iced_text::Catalog + 'a,
    Renderer: advanced_text::Renderer + 'a,
{
    Checkbox::new(is_checked)
}

impl<'a, Message, Theme, Renderer> ModifyBase for Checkbox<'a, Message, Theme, Renderer>
where
    Theme: iced_checkbox::Catalog + iced_text::Catalog,
    Renderer: advanced_text::Renderer,
{
    fn data_mut(&mut self) -> &mut ModifierData { &mut self.data }
}

impl<'a, Message: Clone, Theme, Renderer> Checkbox<'a, Message, Theme, Renderer>
where
    Theme: iced_checkbox::Catalog + iced_text::Catalog + 'a,
    Renderer: advanced_text::Renderer + 'a,
{
    pub fn new(is_checked: bool) -> Self {
        Self {
            inner: iced::widget::Checkbox::new(is_checked),
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

    /// Set checkbox icon size.
    pub fn check_size(mut self, size: impl Into<Pixels>) -> Self {
        self.inner = self.inner.size(size);
        self
    }

    /// Set spacing between checkbox and label.
    pub fn check_spacing(mut self, spacing: impl Into<Pixels>) -> Self {
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

impl_area_interactions!(Checkbox<'a, Message>, where Theme: iced::widget::checkbox::Catalog + iced::widget::text::Catalog, Renderer: iced::advanced::text::Renderer,);

impl_from_element!(Checkbox<'a, Message>, |c| {
    (c.inner, c.data, c.interactions)
}, where iced::widget::checkbox::Catalog);
