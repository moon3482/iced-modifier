//! Modifier-aware Slider widget.

use std::ops::RangeInclusive;

use iced::widget::slider as iced_slider;
use iced::{Length, Pixels, mouse};
use num_traits::FromPrimitive;

use crate::modifier::accumulator::Interactions;
use crate::modifier::build::build_element;
use crate::modifier::bundle::{ModifierData, ModifyBase};

/// Modifier-aware Slider widget.
pub struct Slider<'a, T, Message, Theme = iced::Theme>
where
    Theme: iced_slider::Catalog,
{
    inner: iced::widget::Slider<'a, T, Message, Theme>,
    data: ModifierData,
    interactions: Interactions<Message>,
}

/// Create a new modifier-aware Slider.
pub fn slider<'a, T, Message, Theme, F>(
    range: RangeInclusive<T>,
    value: T,
    on_change: F,
) -> Slider<'a, T, Message, Theme>
where
    T: Copy + From<u8> + PartialOrd,
    Message: Clone + 'a,
    Theme: iced_slider::Catalog + 'a,
    F: 'a + Fn(T) -> Message,
{
    Slider::new(range, value, on_change)
}

impl<'a, T, Message, Theme> ModifyBase for Slider<'a, T, Message, Theme>
where
    Theme: iced_slider::Catalog,
{
    fn data_mut(&mut self) -> &mut ModifierData {
        &mut self.data
    }
}

impl<'a, T, Message: Clone, Theme> Slider<'a, T, Message, Theme>
where
    T: Copy + From<u8> + PartialOrd,
    Theme: iced_slider::Catalog + 'a,
{
    /// Create a new Slider with range, value, and on_change callback.
    pub fn new<F: 'a + Fn(T) -> Message>(range: RangeInclusive<T>, value: T, on_change: F) -> Self {
        Self {
            inner: iced::widget::Slider::new(range, value, on_change),
            data: ModifierData::default(),
            interactions: Interactions::empty(),
        }
    }

    /// Set the step increment.
    pub fn step(mut self, step: impl Into<T>) -> Self {
        self.inner = self.inner.step(step);
        self
    }
    /// Set the step increment when Shift is held.
    pub fn shift_step(mut self, step: impl Into<T>) -> Self {
        self.inner = self.inner.shift_step(step);
        self
    }
    /// Set the message emitted when the slider is released.
    pub fn on_release(mut self, msg: Message) -> Self {
        self.inner = self.inner.on_release(msg);
        self
    }
    /// Set the default (double-click reset) value.
    pub fn default(mut self, default: impl Into<T>) -> Self {
        self.inner = self.inner.default(default);
        self
    }
    /// Set iced Slider native width.
    pub fn slider_width(mut self, width: impl Into<Length>) -> Self {
        self.inner = self.inner.width(width);
        self
    }
    /// Set iced Slider native height.
    pub fn slider_height(mut self, height: impl Into<Pixels>) -> Self {
        self.inner = self.inner.height(height);
        self
    }

    /// Mouse enter handler (MouseArea).
    pub fn on_enter(mut self, msg: Message) -> Self {
        self.interactions.on_enter = Some(msg);
        self
    }
    /// Mouse exit handler (MouseArea).
    pub fn on_exit(mut self, msg: Message) -> Self {
        self.interactions.on_exit = Some(msg);
        self
    }
    /// Cursor style (MouseArea).
    pub fn cursor(mut self, c: mouse::Interaction) -> Self {
        self.interactions.cursor = Some(c);
        self
    }
}

impl<'a, T, Message, Theme, Renderer> From<Slider<'a, T, Message, Theme>>
    for iced::Element<'a, Message, Theme, Renderer>
where
    T: Copy + Into<f64> + FromPrimitive + 'a,
    Message: Clone + 'a,
    Theme: iced::widget::container::Catalog
        + iced::widget::scrollable::Catalog
        + iced::widget::text::Catalog
        + iced_slider::Catalog
        + 'a,
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer + 'a,
    <Theme as iced::widget::container::Catalog>::Class<'a>:
        From<iced::widget::container::StyleFn<'a, Theme>>,
{
    fn from(s: Slider<'a, T, Message, Theme>) -> Self {
        if s.data.extras.hidden {
            return iced::widget::Space::new().into();
        }
        let element: iced::Element<'a, Message, Theme, Renderer> = s.inner.into();
        let mut layers = s.data.layers;
        if !s.data.current.is_empty() {
            layers.push(s.data.current);
        }
        if layers.is_empty()
            && !s.interactions.has_content()
            && s.data.extras.tooltip.is_none()
            && s.data.extras.scrollable.is_none()
            && s.data.extras.widget_id.is_none()
        {
            return element;
        }
        build_element(element, &layers, s.data.extras, s.interactions)
    }
}
