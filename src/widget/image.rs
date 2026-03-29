//! Modifier-aware Image widget (Pattern A — no Message, transitions on interaction).

use std::sync::Arc;

use iced::advanced::text as advanced_text;
use iced::widget::{container, image as iced_image, scrollable, text as iced_text};
use iced::{mouse, Element, Point};

use crate::modifier::accumulator::Interactions;
use crate::modifier::build::build_element;
use crate::modifier::bundle::{ModifierData, ModifyBase};

// ═══════════════════════════════════════════════════════════════
// Image: pure styling (no Message)
// ═══════════════════════════════════════════════════════════════

pub struct Image<'a, Handle = iced_image::Handle> {
    inner: iced::widget::Image<'a, Handle>,
    data: ModifierData,
}

pub fn image<'a, Handle>(handle: impl Into<Handle>) -> Image<'a, Handle>
where
    Handle: Clone + 'a,
{
    Image::new(handle)
}

impl<'a, Handle: Clone + 'a> ModifyBase for Image<'a, Handle> {
    fn data_mut(&mut self) -> &mut ModifierData { &mut self.data }
}

impl<'a, Handle: Clone + 'a> Image<'a, Handle> {
    pub fn new(handle: impl Into<Handle>) -> Self {
        Self {
            inner: iced::widget::Image::new(handle),
            data: ModifierData::default(),
        }
    }

    // ── Widget-specific ──

    /// Set image native width.
    pub fn image_width(mut self, width: impl Into<iced::Length>) -> Self {
        self.inner = self.inner.width(width);
        self
    }

    /// Set image native height.
    pub fn image_height(mut self, height: impl Into<iced::Length>) -> Self {
        self.inner = self.inner.height(height);
        self
    }

    /// Set opacity.
    pub fn opacity(mut self, opacity: impl Into<f32>) -> Self {
        self.inner = self.inner.opacity(opacity);
        self
    }

    /// Set rotation.
    pub fn rotation(mut self, rotation: impl Into<iced_image::Rotation>) -> Self {
        self.inner = self.inner.rotation(rotation);
        self
    }

    /// Set content fit mode.
    pub fn content_fit(mut self, content_fit: iced::ContentFit) -> Self {
        self.inner = self.inner.content_fit(content_fit);
        self
    }

    /// Set filter method.
    pub fn filter_method(mut self, filter: iced_image::FilterMethod) -> Self {
        self.inner = self.inner.filter_method(filter);
        self
    }

    /// Set scale.
    pub fn scale(mut self, scale: impl Into<f32>) -> Self {
        self.inner = self.inner.scale(scale);
        self
    }

    // ── Interaction transitions → InteractiveImage ──

    pub fn on_press<M: Clone>(self, msg: M) -> InteractiveImage<'a, M, Handle> {
        let mut interactions = Interactions::empty();
        interactions.on_press = Some(msg);
        InteractiveImage { inner: self.inner, data: self.data, interactions }
    }

    pub fn on_enter<M: Clone>(self, msg: M) -> InteractiveImage<'a, M, Handle> {
        let mut interactions = Interactions::empty();
        interactions.on_enter = Some(msg);
        InteractiveImage { inner: self.inner, data: self.data, interactions }
    }

    pub fn cursor<M: Clone>(self, cursor: mouse::Interaction) -> InteractiveImage<'a, M, Handle> {
        let mut interactions = Interactions::empty();
        interactions.cursor = Some(cursor);
        InteractiveImage { inner: self.inner, data: self.data, interactions }
    }
}

/// Image → Element (any Message)
impl<'a, Message, Handle, Theme, Renderer> From<Image<'a, Handle>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Handle: Clone + 'a,
    Theme: container::Catalog + scrollable::Catalog + iced_text::Catalog + 'a,
    Renderer: iced::advanced::Renderer + advanced_text::Renderer + iced_image::Renderer<Handle = Handle> + 'a,
    <Theme as container::Catalog>::Class<'a>: From<container::StyleFn<'a, Theme>>,
{
    fn from(img: Image<'a, Handle>) -> Self {
        if img.data.extras.hidden {
            return iced::widget::Space::new().into();
        }
        let element: Element<'a, Message, Theme, Renderer> = img.inner.into();
        let mut layers = img.data.layers;
        if !img.data.current.is_empty() { layers.push(img.data.current); }
        if layers.is_empty()
            && img.data.extras.tooltip.is_none()
            && img.data.extras.scrollable.is_none()
            && img.data.extras.widget_id.is_none()
        {
            return element;
        }
        build_element(element, &layers, img.data.extras, Interactions::empty())
    }
}

// ═══════════════════════════════════════════════════════════════
// InteractiveImage (has Message)
// ═══════════════════════════════════════════════════════════════

pub struct InteractiveImage<'a, Message, Handle = iced_image::Handle> {
    inner: iced::widget::Image<'a, Handle>,
    data: ModifierData,
    interactions: Interactions<Message>,
}

impl<'a, M, Handle: Clone + 'a> ModifyBase for InteractiveImage<'a, M, Handle> {
    fn data_mut(&mut self) -> &mut ModifierData { &mut self.data }
}

impl<'a, M: Clone, Handle: Clone + 'a> InteractiveImage<'a, M, Handle> {
    pub fn on_press(mut self, msg: M) -> Self { self.interactions.on_press = Some(msg); self }
    pub fn on_enter(mut self, msg: M) -> Self { self.interactions.on_enter = Some(msg); self }
    pub fn on_exit(mut self, msg: M) -> Self { self.interactions.on_exit = Some(msg); self }
    pub fn cursor(mut self, cursor: mouse::Interaction) -> Self { self.interactions.cursor = Some(cursor); self }

    pub fn image_width(mut self, width: impl Into<iced::Length>) -> Self { self.inner = self.inner.width(width); self }
    pub fn image_height(mut self, height: impl Into<iced::Length>) -> Self { self.inner = self.inner.height(height); self }
    pub fn opacity(mut self, opacity: impl Into<f32>) -> Self { self.inner = self.inner.opacity(opacity); self }
    pub fn content_fit(mut self, fit: iced::ContentFit) -> Self { self.inner = self.inner.content_fit(fit); self }
}

/// InteractiveImage → Element
impl<'a, Message, Handle, Theme, Renderer> From<InteractiveImage<'a, Message, Handle>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Handle: Clone + 'a,
    Theme: container::Catalog + scrollable::Catalog + iced_text::Catalog + 'a,
    Renderer: iced::advanced::Renderer + advanced_text::Renderer + iced_image::Renderer<Handle = Handle> + 'a,
    <Theme as container::Catalog>::Class<'a>: From<container::StyleFn<'a, Theme>>,
{
    fn from(img: InteractiveImage<'a, Message, Handle>) -> Self {
        if img.data.extras.hidden {
            return iced::widget::Space::new().into();
        }
        let element: Element<'a, Message, Theme, Renderer> = img.inner.into();
        let mut layers = img.data.layers;
        if !img.data.current.is_empty() { layers.push(img.data.current); }
        build_element(element, &layers, img.data.extras, img.interactions)
    }
}
