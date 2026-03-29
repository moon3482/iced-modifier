//! Modifier-aware Column widget with direct chaining support.
//!
//! # Example
//! ```ignore
//! use iced_modifier::prelude::*;
//!
//! Column::new()
//!     .push(Text::new("A").font_size(14))
//!     .push(Text::new("B").font_size(18))
//!     .spacing(8)
//!     .padding(12)
//!     .background_color(Color::WHITE)
//! ```

use std::sync::Arc;

use iced::advanced::text as advanced_text;
use iced::widget::{container, scrollable, text as iced_text};
use iced::{Element, Pixels, Point, alignment, mouse};

use crate::modifier::accumulator::Interactions;
use crate::modifier::build::build_element;
use crate::modifier::bundle::{ModifierData, ModifyBase};

/// Modifier-aware Column widget.
///
/// Wraps [`iced::widget::Column`] with embedded [`ModifierData`] and [`Interactions`]
/// so that style, layout, and interaction methods can be chained directly.
///
/// Since Column's children carry `Message`, this type always has the `Message` generic.
pub struct Column<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer> {
    inner: iced::widget::Column<'a, Message, Theme, Renderer>,
    data: ModifierData,
    interactions: Interactions<Message>,
}

impl<'a, Message, Theme, Renderer> ModifyBase for Column<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    fn data_mut(&mut self) -> &mut ModifierData {
        &mut self.data
    }
}

impl<'a, Message, Theme, Renderer> Column<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    /// Create a new empty Column.
    pub fn new() -> Self {
        Self {
            inner: iced::widget::Column::new(),
            data: ModifierData::default(),
            interactions: Interactions::empty(),
        }
    }

    /// Create a Column with the given children.
    pub fn with_children(
        children: impl IntoIterator<Item = Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        Self {
            inner: iced::widget::Column::with_children(children),
            data: ModifierData::default(),
            interactions: Interactions::empty(),
        }
    }

    // ── Widget-specific methods ──

    /// Set spacing between children.
    /// Compose: `Arrangement.spacedBy()` / SwiftUI: `VStack(spacing:)`
    pub fn spacing(mut self, amount: impl Into<Pixels>) -> Self {
        self.inner = self.inner.spacing(amount);
        self
    }

    /// Add a child element.
    pub fn push(mut self, child: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        self.inner = self.inner.push(child);
        self
    }

    /// Add multiple child elements.
    pub fn extend(
        mut self,
        children: impl IntoIterator<Item = Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        self.inner = self.inner.extend(children);
        self
    }

    /// Set horizontal alignment of children.
    pub fn column_align_x(mut self, align: impl Into<alignment::Horizontal>) -> Self {
        self.inner = self.inner.align_x(align);
        self
    }
}

// ── Interaction methods ──

impl<'a, Message: Clone, Theme, Renderer> Column<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    pub fn on_press(mut self, msg: Message) -> Self {
        self.interactions.on_press = Some(msg);
        self
    }
    pub fn on_release(mut self, msg: Message) -> Self {
        self.interactions.on_release = Some(msg);
        self
    }
    pub fn on_double_click(mut self, msg: Message) -> Self {
        self.interactions.on_double_click = Some(msg);
        self
    }
    pub fn on_right_press(mut self, msg: Message) -> Self {
        self.interactions.on_right_press = Some(msg);
        self
    }
    pub fn on_right_release(mut self, msg: Message) -> Self {
        self.interactions.on_right_release = Some(msg);
        self
    }
    pub fn on_middle_press(mut self, msg: Message) -> Self {
        self.interactions.on_middle_press = Some(msg);
        self
    }
    pub fn on_middle_release(mut self, msg: Message) -> Self {
        self.interactions.on_middle_release = Some(msg);
        self
    }
    pub fn on_enter(mut self, msg: Message) -> Self {
        self.interactions.on_enter = Some(msg);
        self
    }
    pub fn on_exit(mut self, msg: Message) -> Self {
        self.interactions.on_exit = Some(msg);
        self
    }
    pub fn cursor(mut self, cursor: mouse::Interaction) -> Self {
        self.interactions.cursor = Some(cursor);
        self
    }
    pub fn on_scroll(
        mut self,
        f: impl Fn(mouse::ScrollDelta) -> Message + Send + Sync + 'static,
    ) -> Self {
        self.interactions.on_scroll = Some(Arc::new(f));
        self
    }
    pub fn on_move(mut self, f: impl Fn(Point) -> Message + Send + Sync + 'static) -> Self {
        self.interactions.on_move = Some(Arc::new(f));
        self
    }
}

// ── From<Column> for Element ──

impl<'a, Message, Theme, Renderer> From<Column<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: container::Catalog + scrollable::Catalog + iced_text::Catalog + 'a,
    Renderer: iced::advanced::Renderer + advanced_text::Renderer + 'a,
    <Theme as container::Catalog>::Class<'a>: From<container::StyleFn<'a, Theme>>,
{
    fn from(c: Column<'a, Message, Theme, Renderer>) -> Self {
        // Apply spacing from extras if set
        let mut inner = c.inner;
        if let Some(spacing) = c.data.extras.spacing {
            inner = inner.spacing(spacing);
        }

        // Hidden → Space
        if c.data.extras.hidden {
            return iced::widget::Space::new().into();
        }

        let element: Element<'a, Message, Theme, Renderer> = inner.into();

        let mut layers = c.data.layers;
        if !c.data.current.is_empty() {
            layers.push(c.data.current);
        }

        if layers.is_empty()
            && !c.interactions.has_content()
            && c.data.extras.tooltip.is_none()
            && c.data.extras.scrollable.is_none()
            && c.data.extras.widget_id.is_none()
        {
            return element;
        }

        build_element(element, &layers, c.data.extras, c.interactions)
    }
}

/// Create a [`Column`] with the given children.
#[macro_export]
macro_rules! column {
    () => {
        $crate::widget::Column::new()
    };
    ($($child:expr),+ $(,)?) => {
        $crate::widget::Column::new()
            $(.push($child))+
    };
}
