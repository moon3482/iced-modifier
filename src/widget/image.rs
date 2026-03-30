//! Modifier-aware Image widget (Pattern A — no Message, transitions on interaction).

use iced::advanced::image as iced_image_renderer;
use iced::advanced::text as advanced_text;
use iced::widget::{container, image as iced_image, scrollable, text as iced_text};
use iced::{ContentFit, Element, Rotation, mouse};

use crate::modifier::accumulator::Interactions;
use crate::modifier::build::build_element;
use crate::modifier::bundle::{ModifierData, ModifyBase};

// ═══════════════════════════════════════════════════════════════
// Image: pure styling (no Message)
// ═══════════════════════════════════════════════════════════════

/// Modifier-aware Image widget (Pattern A -- no Message generic).
pub struct Image<Handle = iced_image::Handle> {
    inner: iced::widget::Image<Handle>,
    data: ModifierData,
}

/// Create a new modifier-aware Image.
pub fn image<Handle>(handle: impl Into<Handle>) -> Image<Handle>
where
    Handle: Clone,
{
    Image::new(handle)
}

impl<Handle: Clone> ModifyBase for Image<Handle> {
    fn data_mut(&mut self) -> &mut ModifierData {
        &mut self.data
    }
}

impl<Handle: Clone> Image<Handle> {
    /// Create a new Image with the given handle.
    pub fn new(handle: impl Into<Handle>) -> Self {
        Self {
            inner: iced::widget::Image::new(handle),
            data: ModifierData::default(),
        }
    }

    /// Set iced Image native width.
    pub fn image_width(mut self, width: impl Into<iced::Length>) -> Self {
        self.inner = self.inner.width(width);
        self
    }

    /// Set iced Image native height.
    pub fn image_height(mut self, height: impl Into<iced::Length>) -> Self {
        self.inner = self.inner.height(height);
        self
    }

    /// Set image opacity (0.0 transparent, 1.0 opaque).
    pub fn opacity(mut self, opacity: impl Into<f32>) -> Self {
        self.inner = self.inner.opacity(opacity);
        self
    }

    /// Set image rotation.
    pub fn rotation(mut self, rotation: impl Into<Rotation>) -> Self {
        self.inner = self.inner.rotation(rotation);
        self
    }

    /// Set how the image content is fitted within its bounds.
    pub fn content_fit(mut self, content_fit: ContentFit) -> Self {
        self.inner = self.inner.content_fit(content_fit);
        self
    }

    /// Set the image filter method (nearest or linear).
    pub fn filter_method(mut self, filter: iced_image::FilterMethod) -> Self {
        self.inner = self.inner.filter_method(filter);
        self
    }

    /// Set image scale factor.
    pub fn scale(mut self, scale: impl Into<f32>) -> Self {
        self.inner = self.inner.scale(scale);
        self
    }

    // ── Interaction transitions → InteractiveImage ──

    /// Transition to InteractiveImage with a press handler.
    pub fn on_press<M: Clone>(self, msg: M) -> InteractiveImage<M, Handle> {
        let mut interactions = Interactions::empty();
        interactions.on_press = Some(msg);
        InteractiveImage {
            inner: self.inner,
            data: self.data,
            interactions,
        }
    }

    /// Transition to InteractiveImage with an enter handler.
    pub fn on_enter<M: Clone>(self, msg: M) -> InteractiveImage<M, Handle> {
        let mut interactions = Interactions::empty();
        interactions.on_enter = Some(msg);
        InteractiveImage {
            inner: self.inner,
            data: self.data,
            interactions,
        }
    }

    /// Transition to InteractiveImage with a custom cursor style.
    pub fn cursor<M: Clone>(self, cursor: mouse::Interaction) -> InteractiveImage<M, Handle> {
        let mut interactions = Interactions::empty();
        interactions.cursor = Some(cursor);
        InteractiveImage {
            inner: self.inner,
            data: self.data,
            interactions,
        }
    }
}

/// Image → Element (any Message)
impl<Message, Handle, Theme, Renderer> From<Image<Handle>>
    for Element<'static, Message, Theme, Renderer>
where
    Message: Clone + 'static,
    Handle: Clone + 'static,
    Theme: container::Catalog + scrollable::Catalog + iced_text::Catalog + 'static,
    Renderer: iced::advanced::Renderer
        + advanced_text::Renderer
        + iced_image_renderer::Renderer<Handle = Handle>
        + 'static,
    <Theme as container::Catalog>::Class<'static>: From<container::StyleFn<'static, Theme>>,
{
    fn from(img: Image<Handle>) -> Self {
        if img.data.extras.hidden {
            return iced::widget::Space::new().into();
        }
        let element: Element<'static, Message, Theme, Renderer> = img.inner.into();
        let mut layers = img.data.layers;
        if !img.data.current.is_empty() {
            layers.push(img.data.current);
        }
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

/// Image widget with interaction handlers.
pub struct InteractiveImage<Message, Handle = iced_image::Handle> {
    inner: iced::widget::Image<Handle>,
    data: ModifierData,
    interactions: Interactions<Message>,
}

impl<M, Handle: Clone> ModifyBase for InteractiveImage<M, Handle> {
    fn data_mut(&mut self) -> &mut ModifierData {
        &mut self.data
    }
}

impl<M: Clone, Handle: Clone> InteractiveImage<M, Handle> {
    /// Set press handler.
    pub fn on_press(mut self, msg: M) -> Self {
        self.interactions.on_press = Some(msg);
        self
    }
    /// Mouse enter handler.
    pub fn on_enter(mut self, msg: M) -> Self {
        self.interactions.on_enter = Some(msg);
        self
    }
    /// Mouse exit handler.
    pub fn on_exit(mut self, msg: M) -> Self {
        self.interactions.on_exit = Some(msg);
        self
    }
    /// Cursor style.
    pub fn cursor(mut self, cursor: mouse::Interaction) -> Self {
        self.interactions.cursor = Some(cursor);
        self
    }

    /// Set iced Image native width.
    pub fn image_width(mut self, width: impl Into<iced::Length>) -> Self {
        self.inner = self.inner.width(width);
        self
    }
    /// Set iced Image native height.
    pub fn image_height(mut self, height: impl Into<iced::Length>) -> Self {
        self.inner = self.inner.height(height);
        self
    }
    /// Set image opacity (0.0 transparent, 1.0 opaque).
    pub fn opacity(mut self, opacity: impl Into<f32>) -> Self {
        self.inner = self.inner.opacity(opacity);
        self
    }
    /// Set how the image content is fitted within its bounds.
    pub fn content_fit(mut self, fit: ContentFit) -> Self {
        self.inner = self.inner.content_fit(fit);
        self
    }
}

/// InteractiveImage → Element
impl<Message, Handle, Theme, Renderer> From<InteractiveImage<Message, Handle>>
    for Element<'static, Message, Theme, Renderer>
where
    Message: Clone + 'static,
    Handle: Clone + 'static,
    Theme: container::Catalog + scrollable::Catalog + iced_text::Catalog + 'static,
    Renderer: iced::advanced::Renderer
        + advanced_text::Renderer
        + iced_image_renderer::Renderer<Handle = Handle>
        + 'static,
    <Theme as container::Catalog>::Class<'static>: From<container::StyleFn<'static, Theme>>,
{
    fn from(img: InteractiveImage<Message, Handle>) -> Self {
        if img.data.extras.hidden {
            return iced::widget::Space::new().into();
        }
        let element: Element<'static, Message, Theme, Renderer> = img.inner.into();
        let mut layers = img.data.layers;
        if !img.data.current.is_empty() {
            layers.push(img.data.current);
        }
        build_element(element, &layers, img.data.extras, img.interactions)
    }
}
