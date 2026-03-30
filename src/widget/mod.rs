//! Wrapped iced widgets with modifier-aware capabilities.
//!
//! These types wrap iced's native widgets to support widget-specific properties
//! (like `font_size`, `spacing`) through the modifier chain.
//!
//! # Exported widgets
//!
//! | Widget | Helper function |
//! |--------|----------------|
//! | [`Button`] | [`button()`] |
//! | [`Checkbox`] | [`checkbox()`] |
//! | [`Column`] | *(constructed directly)* |
//! | `Image` / `InteractiveImage` | `image()` *(feature = "image")* |
//! | [`PickList`] | *(constructed via `PickList::new`)* |
//! | [`Radio`] | [`radio()`] |
//! | [`Row`] | *(constructed directly)* |
//! | [`Slider`] | [`slider()`] |
//! | [`Text`] / [`InteractiveText`] | [`text()`] |
//! | [`TextEditor`] | [`text_editor()`] |
//! | [`TextInput`] | [`text_input()`] |
//! | [`Toggler`] | [`toggler()`] |
//!
//! # Widget patterns
//!
//! **Pattern A** -- No `Message` generic (e.g. `Image`, [`Text`]).
//! The widget carries only styling data. Adding an interaction handler
//! (`.on_press()`, `.on_enter()`, etc.) transitions into a companion
//! interactive type (e.g. `InteractiveImage`, [`InteractiveText`]).
//!
//! **Pattern B** -- Has `Message` generic from construction (e.g. [`Button`],
//! [`TextInput`], [`Slider`]). Interaction methods live directly on the
//! widget struct alongside the native iced methods.
//!
//! ```ignore
//! use iced_modifier::widget::{button, text};
//!
//! // Pattern B: Message generic from the start
//! let btn = button(text("Click me")).on_press(MyMessage::Clicked);
//!
//! // Pattern A: no Message until interaction is added
//! let txt = text("Hello").on_press(MyMessage::Tapped);
//! ```

mod button;
mod checkbox;
mod column;
#[cfg(feature = "image")]
mod image;
mod pick_list;
mod radio;
mod row;
mod slider;
mod text;
mod text_editor;
mod text_input;
mod toggler;

pub use button::{Button, button};
pub use checkbox::{Checkbox, checkbox};
pub use column::Column;
#[cfg(feature = "image")]
pub use image::{Image, InteractiveImage, image};
pub use pick_list::PickList;
pub use radio::{Radio, radio};
pub use row::Row;
pub use slider::{Slider, slider};
pub use text::{InteractiveText, Text, text};
pub use text_editor::{TextEditor, text_editor};
pub use text_input::{TextInput, text_input};
pub use toggler::{Toggler, toggler};

/// Macro to implement MouseArea interaction methods on a pattern-B widget.
/// Pattern B widgets have `interactions: Interactions<Message>` field.
macro_rules! impl_area_interactions {
    ($ty:ident <'a, Message $(, $extra:ident)*> $(, where $($bound:tt)*)?) => {
        impl<'a, Message: Clone, $($extra,)* Theme, Renderer> $ty<'a, Message, $($extra,)* Theme, Renderer>
        where
            Renderer: iced::advanced::Renderer,
            $( $($bound)* )?
        {
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
            pub fn cursor(mut self, cursor: iced::mouse::Interaction) -> Self {
                self.interactions.cursor = Some(cursor);
                self
            }
            /// Scroll event handler (MouseArea).
            pub fn on_scroll(mut self, f: impl Fn(iced::mouse::ScrollDelta) -> Message + Send + Sync + 'static) -> Self {
                self.interactions.on_scroll = Some(std::sync::Arc::new(f));
                self
            }
            /// Mouse move handler (MouseArea).
            pub fn on_move(mut self, f: impl Fn(iced::Point) -> Message + Send + Sync + 'static) -> Self {
                self.interactions.on_move = Some(std::sync::Arc::new(f));
                self
            }
        }
    };
}

/// Macro to implement From<Widget> for Element with standard build pattern.
macro_rules! impl_from_element {
    ($ty:ident <'a, Message $(, $extra:ident)*>, |$self:ident| $prepare:expr $(, where $($bound:tt)*)?) => {
        impl<'a, Message, $($extra,)* Theme, Renderer> From<$ty<'a, Message, $($extra,)* Theme, Renderer>>
            for iced::Element<'a, Message, Theme, Renderer>
        where
            Message: Clone + 'a,
            Theme: iced::widget::container::Catalog
                + iced::widget::scrollable::Catalog
                + iced::widget::text::Catalog
                $( + $($bound)* )?
                + 'a,
            Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer + 'a,
            <Theme as iced::widget::container::Catalog>::Class<'a>:
                From<iced::widget::container::StyleFn<'a, Theme>>,
        {
            fn from($self: $ty<'a, Message, $($extra,)* Theme, Renderer>) -> Self {
                let (element, data, interactions) = $prepare;

                if data.extras.hidden {
                    return iced::widget::Space::new().into();
                }

                let element: iced::Element<'a, Message, Theme, Renderer> = element.into();

                let mut layers = data.layers;
                if !data.current.is_empty() {
                    layers.push(data.current);
                }

                if layers.is_empty()
                    && !interactions.has_content()
                    && data.extras.tooltip.is_none()
                    && data.extras.scrollable.is_none()
                    && data.extras.widget_id.is_none()
                {
                    return element;
                }

                crate::modifier::build::build_element(element, &layers, data.extras, interactions)
            }
        }
    };
}

pub(crate) use impl_area_interactions;
pub(crate) use impl_from_element;
