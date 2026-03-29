//! Modifier-aware Text widget with direct chaining support.
//!
//! # Example
//! ```ignore
//! use iced_modifier::prelude::*;
//!
//! // Direct chaining (SwiftUI style)
//! Text::new("Hello")
//!     .font_size(16)
//!     .padding(12)
//!     .background_color(Color::WHITE)
//!     .on_press(Msg::Click)
//!     .cursor(mouse::Interaction::Pointer)
//!
//! // Backward compatible
//! text("Hello").modify(Modifier::new().font_size(16).padding(12))
//! ```

use std::sync::Arc;

use iced::advanced::text as advanced_text;
use iced::widget::{container, scrollable, text as iced_text};
use iced::{alignment, mouse, Color, Element, Pixels, Point};

use crate::modifier::build::build_element;
use crate::modifier::bundle::{ModifierData, ModifyBase};
use crate::modifier::accumulator::Interactions;
use crate::IntoModified;

// ═══════════════════════════════════════════════════════════════
// Text: pure styling (no Message generic)
// ═══════════════════════════════════════════════════════════════

/// Modifier-aware Text widget.
///
/// Wraps [`iced::widget::Text`] with an embedded [`ModifierData`] so that
/// style/layout methods can be chained directly on the widget.
///
/// Interaction methods (`.on_press()`, `.on_enter()`, etc.) transition to
/// [`InteractiveText`] which carries the `Message` generic.
pub struct Text<'a, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Theme: iced_text::Catalog,
    Renderer: advanced_text::Renderer,
{
    inner: iced::widget::Text<'a, Theme, Renderer>,
    data: ModifierData,
}

/// Create a new modifier-aware [`Text`] widget.
///
/// Shadows [`iced::widget::text`] when imported via `iced_modifier::prelude::*`.
pub fn text<'a, Theme, Renderer>(
    content: impl iced::widget::text::IntoFragment<'a>,
) -> Text<'a, Theme, Renderer>
where
    Theme: iced_text::Catalog + 'a,
    Renderer: advanced_text::Renderer,
{
    Text::new(content)
}

// ModifyBase: all 40+ style/layout methods for free
impl<'a, Theme, Renderer> ModifyBase for Text<'a, Theme, Renderer>
where
    Theme: iced_text::Catalog + 'a,
    Renderer: advanced_text::Renderer,
{
    fn data_mut(&mut self) -> &mut ModifierData {
        &mut self.data
    }
}

impl<'a, Theme, Renderer> Text<'a, Theme, Renderer>
where
    Theme: iced_text::Catalog + 'a,
    Renderer: advanced_text::Renderer,
{
    /// Create a new modifier-aware Text widget.
    pub fn new(content: impl iced::widget::text::IntoFragment<'a>) -> Self {
        Self {
            inner: iced::widget::Text::new(content),
            data: ModifierData::default(),
        }
    }

    // ── Backward-compatible .modify() ──

    /// Apply a [`Modifier`] or [`Interactor`] to this Text widget.
    ///
    /// Merges the modifier's data with this widget's accumulated data.
    /// Prefer direct chaining (`.font_size().padding()`) for new code.
    pub fn modify<Message>(
        self,
        modifier: impl IntoModified<Message>,
    ) -> Element<'a, Message, Theme, Renderer>
    where
        Message: Clone + 'a,
        Theme: container::Catalog + scrollable::Catalog + iced_text::Catalog + 'a,
        Renderer: iced::advanced::Renderer + advanced_text::Renderer + 'a,
        <Theme as container::Catalog>::Class<'a>: From<container::StyleFn<'a, Theme>>,
    {
        let (mod_layers, mod_extras, mod_interactions) = modifier.into_parts();

        // font_size: self takes priority, then modifier
        let mut inner = self.inner;
        let font_size = self.data.extras.font_size.or(mod_extras.font_size);
        if let Some(size) = font_size {
            inner = inner.size(size);
        }

        let element: Element<'a, Message, Theme, Renderer> = inner.into();

        // Merge layers: self layers first, then modifier layers
        let mut layers = self.data.layers;
        if !self.data.current.is_empty() {
            layers.push(self.data.current);
        }
        for layer in mod_layers {
            layers.push(layer);
        }

        // Merge extras (modifier extras override for non-set fields)
        let mut extras = self.data.extras;
        if !extras.hidden { extras.hidden = mod_extras.hidden; }
        if extras.widget_id.is_none() { extras.widget_id = mod_extras.widget_id; }
        if extras.tooltip.is_none() { extras.tooltip = mod_extras.tooltip; }
        if extras.scrollable.is_none() { extras.scrollable = mod_extras.scrollable; }

        if layers.is_empty()
            && !extras.hidden
            && !mod_interactions.has_content()
            && extras.tooltip.is_none()
            && extras.scrollable.is_none()
            && extras.widget_id.is_none()
        {
            return element;
        }

        build_element(element, &layers, extras, mod_interactions)
    }
}

// ── Interaction methods: Text → InteractiveText ──

macro_rules! impl_text_to_interactive {
    ($method:ident, $field:ident) => {
        pub fn $method<M: Clone>(self, msg: M) -> InteractiveText<'a, M, Theme, Renderer> {
            let mut interactions = Interactions::empty();
            interactions.$field = Some(msg);
            InteractiveText {
                inner: self.inner,
                data: self.data,
                interactions,
            }
        }
    };
}

impl<'a, Theme, Renderer> Text<'a, Theme, Renderer>
where
    Theme: iced_text::Catalog + 'a,
    Renderer: advanced_text::Renderer,
{
    impl_text_to_interactive!(on_press, on_press);
    impl_text_to_interactive!(on_release, on_release);
    impl_text_to_interactive!(on_double_click, on_double_click);
    impl_text_to_interactive!(on_right_press, on_right_press);
    impl_text_to_interactive!(on_right_release, on_right_release);
    impl_text_to_interactive!(on_middle_press, on_middle_press);
    impl_text_to_interactive!(on_middle_release, on_middle_release);
    impl_text_to_interactive!(on_enter, on_enter);
    impl_text_to_interactive!(on_exit, on_exit);

    /// Set cursor style. Transitions to [`InteractiveText`].
    pub fn cursor<M: Clone>(self, cursor: mouse::Interaction) -> InteractiveText<'a, M, Theme, Renderer> {
        let mut interactions = Interactions::empty();
        interactions.cursor = Some(cursor);
        InteractiveText {
            inner: self.inner,
            data: self.data,
            interactions,
        }
    }

    /// Set scroll event handler. Transitions to [`InteractiveText`].
    pub fn on_scroll<M: Clone>(self, f: impl Fn(mouse::ScrollDelta) -> M + Send + Sync + 'static) -> InteractiveText<'a, M, Theme, Renderer> {
        let mut interactions = Interactions::empty();
        interactions.on_scroll = Some(Arc::new(f));
        InteractiveText {
            inner: self.inner,
            data: self.data,
            interactions,
        }
    }

    /// Set mouse move handler. Transitions to [`InteractiveText`].
    pub fn on_move<M: Clone>(self, f: impl Fn(Point) -> M + Send + Sync + 'static) -> InteractiveText<'a, M, Theme, Renderer> {
        let mut interactions = Interactions::empty();
        interactions.on_move = Some(Arc::new(f));
        InteractiveText {
            inner: self.inner,
            data: self.data,
            interactions,
        }
    }
}

// ═══════════════════════════════════════════════════════════════
// InteractiveText: styling + interactions (has Message generic)
// ═══════════════════════════════════════════════════════════════

/// Text widget with interaction handlers.
///
/// Created by calling `.on_press()`, `.on_enter()`, etc. on a [`Text`].
/// All style/layout methods from [`ModifyBase`] remain available.
pub struct InteractiveText<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Theme: iced_text::Catalog,
    Renderer: advanced_text::Renderer,
{
    inner: iced::widget::Text<'a, Theme, Renderer>,
    data: ModifierData,
    interactions: Interactions<Message>,
}

// ModifyBase for InteractiveText
impl<'a, M, Theme, Renderer> ModifyBase for InteractiveText<'a, M, Theme, Renderer>
where
    Theme: iced_text::Catalog + 'a,
    Renderer: advanced_text::Renderer,
{
    fn data_mut(&mut self) -> &mut ModifierData {
        &mut self.data
    }
}

// Interaction methods on InteractiveText (override/add)
impl<'a, M: Clone, Theme, Renderer> InteractiveText<'a, M, Theme, Renderer>
where
    Theme: iced_text::Catalog + 'a,
    Renderer: advanced_text::Renderer,
{
    pub fn on_press(mut self, msg: M) -> Self {
        self.interactions.on_press = Some(msg);
        self
    }
    pub fn on_release(mut self, msg: M) -> Self {
        self.interactions.on_release = Some(msg);
        self
    }
    pub fn on_double_click(mut self, msg: M) -> Self {
        self.interactions.on_double_click = Some(msg);
        self
    }
    pub fn on_right_press(mut self, msg: M) -> Self {
        self.interactions.on_right_press = Some(msg);
        self
    }
    pub fn on_right_release(mut self, msg: M) -> Self {
        self.interactions.on_right_release = Some(msg);
        self
    }
    pub fn on_middle_press(mut self, msg: M) -> Self {
        self.interactions.on_middle_press = Some(msg);
        self
    }
    pub fn on_middle_release(mut self, msg: M) -> Self {
        self.interactions.on_middle_release = Some(msg);
        self
    }
    pub fn on_enter(mut self, msg: M) -> Self {
        self.interactions.on_enter = Some(msg);
        self
    }
    pub fn on_exit(mut self, msg: M) -> Self {
        self.interactions.on_exit = Some(msg);
        self
    }
    pub fn cursor(mut self, cursor: mouse::Interaction) -> Self {
        self.interactions.cursor = Some(cursor);
        self
    }
    pub fn on_scroll(mut self, f: impl Fn(mouse::ScrollDelta) -> M + Send + Sync + 'static) -> Self {
        self.interactions.on_scroll = Some(Arc::new(f));
        self
    }
    pub fn on_move(mut self, f: impl Fn(Point) -> M + Send + Sync + 'static) -> Self {
        self.interactions.on_move = Some(Arc::new(f));
        self
    }
}

// ═══════════════════════════════════════════════════════════════
// Widget-specific methods (shared by Text and InteractiveText)
// ═══════════════════════════════════════════════════════════════

macro_rules! impl_text_widget_methods {
    ($ty:ident $(, $msg:ident)?) => {
        impl<'a, $($msg,)? Theme, Renderer> $ty<'a, $($msg,)? Theme, Renderer>
        where
            Theme: iced_text::Catalog + 'a,
            Renderer: advanced_text::Renderer,
        {
            /// Set font size via modifier system. Applied during Element conversion.
            /// Compose: `fontSize` / SwiftUI: `.font(.system(size:))`
            pub fn font_size(mut self, size: impl Into<Pixels>) -> Self {
                self.data.extras.font_size = Some(size.into());
                self
            }

            /// Set font size directly on the iced Text widget (iced native API).
            pub fn size(mut self, size: impl Into<Pixels>) -> Self {
                self.inner = self.inner.size(size);
                self
            }

            /// Set the font family.
            pub fn font(mut self, font: impl Into<Renderer::Font>) -> Self {
                self.inner = self.inner.font(font);
                self
            }

            /// Set the line height.
            pub fn line_height(mut self, line_height: impl Into<iced_text::LineHeight>) -> Self {
                self.inner = self.inner.line_height(line_height);
                self
            }

            /// Center the text horizontally and vertically.
            pub fn text_center(self) -> Self {
                self.text_align_x(alignment::Horizontal::Center)
                    .text_align_y(alignment::Vertical::Center)
            }

            /// Set horizontal text alignment within the text boundaries.
            pub fn text_align_x(mut self, a: impl Into<iced_text::Alignment>) -> Self {
                self.inner = self.inner.align_x(a);
                self
            }

            /// Set vertical text alignment within the text boundaries.
            pub fn text_align_y(mut self, a: impl Into<alignment::Vertical>) -> Self {
                self.inner = self.inner.align_y(a);
                self
            }

            /// Set the wrapping strategy.
            pub fn wrapping(mut self, wrapping: iced_text::Wrapping) -> Self {
                self.inner = self.inner.wrapping(wrapping);
                self
            }

            /// Set the shaping strategy.
            pub fn shaping(mut self, shaping: iced_text::Shaping) -> Self {
                self.inner = self.inner.shaping(shaping);
                self
            }

            /// Set the text color.
            pub fn color(self, color: impl Into<Color>) -> Self
            where
                <Theme as iced_text::Catalog>::Class<'a>:
                    From<iced_text::StyleFn<'a, Theme>>,
            {
                Self {
                    inner: self.inner.color(color),
                    ..self
                }
            }
        }
    };
}

impl_text_widget_methods!(Text);
impl_text_widget_methods!(InteractiveText, Message);

// ═══════════════════════════════════════════════════════════════
// From impls: Widget → Element
// ═══════════════════════════════════════════════════════════════

/// Helper to build Element from inner text + modifier data + interactions.
fn build_text_element<'a, Message, Theme, Renderer>(
    mut inner: iced::widget::Text<'a, Theme, Renderer>,
    data: ModifierData,
    interactions: Interactions<Message>,
) -> Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: container::Catalog + scrollable::Catalog + iced_text::Catalog + 'a,
    Renderer: iced::advanced::Renderer + advanced_text::Renderer + 'a,
    <Theme as container::Catalog>::Class<'a>: From<container::StyleFn<'a, Theme>>,
{
    // 1. Apply font_size to inner widget
    if let Some(size) = data.extras.font_size {
        inner = inner.size(size);
    }

    // 2. Hidden → Space
    if data.extras.hidden {
        return iced::widget::Space::new().into();
    }

    // 3. Convert to Element
    let element: Element<'a, Message, Theme, Renderer> = inner.into();

    // 4. Build layers
    let mut layers = data.layers;
    if !data.current.is_empty() {
        layers.push(data.current);
    }

    // 5. Optimize: no modifications → return as-is
    if layers.is_empty()
        && !interactions.has_content()
        && data.extras.tooltip.is_none()
        && data.extras.scrollable.is_none()
        && data.extras.widget_id.is_none()
    {
        return element;
    }

    // 6. Wrap with Container/MouseArea/Tooltip/Scrollable
    build_element(element, &layers, data.extras, interactions)
}

/// Text (no Message) → Element<AnyMessage>
impl<'a, Message, Theme, Renderer> From<Text<'a, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: container::Catalog + scrollable::Catalog + iced_text::Catalog + 'a,
    Renderer: iced::advanced::Renderer + advanced_text::Renderer + 'a,
    <Theme as container::Catalog>::Class<'a>: From<container::StyleFn<'a, Theme>>,
{
    fn from(t: Text<'a, Theme, Renderer>) -> Self {
        build_text_element(t.inner, t.data, Interactions::empty())
    }
}

/// InteractiveText<Message> → Element<Message>
impl<'a, Message, Theme, Renderer> From<InteractiveText<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: container::Catalog + scrollable::Catalog + iced_text::Catalog + 'a,
    Renderer: iced::advanced::Renderer + advanced_text::Renderer + 'a,
    <Theme as container::Catalog>::Class<'a>: From<container::StyleFn<'a, Theme>>,
{
    fn from(t: InteractiveText<'a, Message, Theme, Renderer>) -> Self {
        build_text_element(t.inner, t.data, t.interactions)
    }
}
