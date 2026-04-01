//! Modifier-aware Button widget.
//!
//! Style properties (`background_color`, `corner_radius`, `border`, `shadow`, `text_color`)
//! are applied directly to the iced Button via its `.style()` API, not through
//! Container wrapping. This ensures visual styles render on the button itself.

use iced::widget::{button as iced_button, container, scrollable, text as iced_text};
use iced::{Element, advanced::text as advanced_text};

use crate::modifier::accumulator::{Interactions, StyleAccumulator};
use crate::modifier::bundle::{ModifierData, ModifyBase};

use super::impl_area_interactions;

/// Modifier-aware Button widget wrapping `iced::widget::Button`.
///
/// Unlike other widgets, style properties (background, border, shadow, text_color)
/// are applied directly to the iced Button's own style, so colors and corner radii
/// render on the button itself rather than on a Container wrapper.
pub struct Button<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Theme: iced_button::Catalog,
    Renderer: iced::advanced::Renderer,
{
    inner: iced::widget::Button<'a, Message, Theme, Renderer>,
    data: ModifierData,
    interactions: Interactions<Message>,
}

/// Create a new modifier-aware Button.
pub fn button<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Button<'a, Message, Theme, Renderer>
where
    Theme: iced_button::Catalog + 'a,
    Renderer: iced::advanced::Renderer + 'a,
{
    Button::new(content)
}

impl<'a, Message, Theme, Renderer> ModifyBase for Button<'a, Message, Theme, Renderer>
where
    Theme: iced_button::Catalog,
    Renderer: iced::advanced::Renderer,
{
    fn data_mut(&mut self) -> &mut ModifierData {
        &mut self.data
    }
}

impl<'a, Message, Theme, Renderer> Button<'a, Message, Theme, Renderer>
where
    Theme: iced_button::Catalog + 'a,
    Renderer: iced::advanced::Renderer + 'a,
{
    /// Create a new Button with the given content.
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

impl_area_interactions!(Button<'a, Message>, where Theme: iced_button::Catalog,);

// Custom From<Button> for Element — applies style to the button itself.
impl<'a, Message, Theme, Renderer> From<Button<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: container::Catalog
        + scrollable::Catalog
        + iced_text::Catalog
        + iced_button::Catalog
        + 'a,
    Renderer: iced::advanced::Renderer + advanced_text::Renderer + 'a,
    <Theme as container::Catalog>::Class<'a>: From<container::StyleFn<'a, Theme>>,
    <Theme as iced_button::Catalog>::Class<'a>: From<iced_button::StyleFn<'a, Theme>>,
{
    fn from(b: Button<'a, Message, Theme, Renderer>) -> Self {
        if b.data.extras.hidden {
            return iced::widget::Space::new().into();
        }

        // Separate style from the current layer so we can apply it to the Button.
        let mut layers = b.data.layers;
        let current = b.data.current;

        // Collect style from the current (outermost) layer for the button itself.
        let button_style = current.style;
        let has_button_style = button_style.has_content();

        // Apply style directly to the iced Button via .style()
        let mut inner = b.inner;
        if has_button_style {
            inner = inner.style(move |_theme: &Theme, _status: iced_button::Status| {
                to_button_style(&button_style)
            });
        }

        // The current layer still has layout — push a layout-only version
        let layout_layer = crate::modifier::accumulator::Layer {
            style: StyleAccumulator::default(), // style already applied to button
            layout: current.layout,
        };

        if layout_layer.layout.has_content() || layout_layer.layout.has_margin() {
            layers.push(layout_layer);
        }

        let element: Element<'a, Message, Theme, Renderer> = inner.into();

        if layers.is_empty()
            && !b.interactions.has_content()
            && b.data.extras.tooltip.is_none()
            && b.data.extras.scrollable.is_none()
            && b.data.extras.widget_id.is_none()
        {
            return element;
        }

        crate::modifier::build::build_element(element, &layers, b.data.extras, b.interactions)
    }
}

/// Convert a StyleAccumulator into a button::Style.
fn to_button_style(acc: &StyleAccumulator) -> iced_button::Style {
    let mut style = iced_button::Style {
        background: None,
        text_color: iced::Color::BLACK,
        border: iced::Border::default(),
        shadow: iced::Shadow::default(),
        snap: false,
    };
    if let Some(bg) = acc.background {
        style.background = Some(bg);
    }
    if let Some(border) = acc.border {
        style.border = border;
    }
    if let Some(shadow) = acc.shadow {
        style.shadow = shadow;
    }
    if let Some(color) = acc.text_color {
        style.text_color = color;
    }
    style
}
