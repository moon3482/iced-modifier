//! Modifier-aware PickList widget.

use std::borrow::Borrow;

use iced::advanced::text as advanced_text;
use iced::widget::{pick_list as iced_pick_list, text as iced_text};
use iced::{Pixels, mouse};

use crate::modifier::accumulator::Interactions;
use crate::modifier::build::build_element;
use crate::modifier::bundle::{ModifierData, ModifyBase};

pub struct PickList<'a, T, L, V, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    T: ToString + PartialEq + Clone,
    L: Borrow<[T]>,
    V: Borrow<T>,
    Theme: iced_pick_list::Catalog + iced::overlay::menu::Catalog,
    Renderer: advanced_text::Renderer,
{
    inner: iced::widget::PickList<'a, T, L, V, Message, Theme, Renderer>,
    data: ModifierData,
    interactions: Interactions<Message>,
}

impl<'a, T, L, V, Message, Theme, Renderer> ModifyBase
    for PickList<'a, T, L, V, Message, Theme, Renderer>
where
    T: ToString + PartialEq + Clone,
    L: Borrow<[T]>,
    V: Borrow<T>,
    Theme: iced_pick_list::Catalog + iced::overlay::menu::Catalog,
    Renderer: advanced_text::Renderer,
{
    fn data_mut(&mut self) -> &mut ModifierData {
        &mut self.data
    }
}

impl<'a, T, L, V, Message: Clone, Theme, Renderer> PickList<'a, T, L, V, Message, Theme, Renderer>
where
    T: ToString + PartialEq + Clone + 'a,
    L: Borrow<[T]> + 'a,
    V: Borrow<T> + 'a,
    Message: Clone + 'a,
    Theme: iced_pick_list::Catalog + iced::overlay::menu::Catalog + iced_text::Catalog + 'a,
    Renderer: advanced_text::Renderer + 'a,
{
    pub fn new(options: L, selected: Option<V>, on_select: impl Fn(T) -> Message + 'a) -> Self {
        Self {
            inner: iced::widget::PickList::new(options, selected, on_select),
            data: ModifierData::default(),
            interactions: Interactions::empty(),
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.inner = self.inner.placeholder(placeholder);
        self
    }
    pub fn text_size(mut self, size: impl Into<Pixels>) -> Self {
        self.inner = self.inner.text_size(size);
        self
    }
    pub fn menu_height(mut self, height: impl Into<iced::Length>) -> Self {
        self.inner = self.inner.menu_height(height);
        self
    }
    pub fn font(mut self, font: impl Into<Renderer::Font>) -> Self {
        self.inner = self.inner.font(font);
        self
    }
    pub fn list_padding(mut self, padding: impl Into<iced::Padding>) -> Self {
        self.inner = self.inner.padding(padding);
        self
    }
    pub fn on_open(mut self, msg: Message) -> Self {
        self.inner = self.inner.on_open(msg);
        self
    }
    pub fn on_close(mut self, msg: Message) -> Self {
        self.inner = self.inner.on_close(msg);
        self
    }

    // Interactions
    pub fn on_enter(mut self, msg: Message) -> Self {
        self.interactions.on_enter = Some(msg);
        self
    }
    pub fn on_exit(mut self, msg: Message) -> Self {
        self.interactions.on_exit = Some(msg);
        self
    }
    pub fn cursor(mut self, c: mouse::Interaction) -> Self {
        self.interactions.cursor = Some(c);
        self
    }
}

impl<'a, T, L, V, Message, Theme, Renderer> From<PickList<'a, T, L, V, Message, Theme, Renderer>>
    for iced::Element<'a, Message, Theme, Renderer>
where
    T: ToString + PartialEq + Clone + 'a,
    L: Borrow<[T]> + 'a,
    V: Borrow<T> + 'a,
    Message: Clone + 'a,
    Theme: iced::widget::container::Catalog
        + iced::widget::scrollable::Catalog
        + iced_text::Catalog
        + iced_pick_list::Catalog
        + iced::overlay::menu::Catalog
        + 'a,
    Renderer: iced::advanced::Renderer + advanced_text::Renderer + 'a,
    <Theme as iced::widget::container::Catalog>::Class<'a>:
        From<iced::widget::container::StyleFn<'a, Theme>>,
{
    fn from(p: PickList<'a, T, L, V, Message, Theme, Renderer>) -> Self {
        if p.data.extras.hidden {
            return iced::widget::Space::new().into();
        }
        let element: iced::Element<'a, Message, Theme, Renderer> = p.inner.into();
        let mut layers = p.data.layers;
        if !p.data.current.is_empty() {
            layers.push(p.data.current);
        }
        if layers.is_empty()
            && !p.interactions.has_content()
            && p.data.extras.tooltip.is_none()
            && p.data.extras.scrollable.is_none()
            && p.data.extras.widget_id.is_none()
        {
            return element;
        }
        build_element(element, &layers, p.data.extras, p.interactions)
    }
}
