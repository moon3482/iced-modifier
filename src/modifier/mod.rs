//! Modifier system for iced widgets.
//!
//! This module provides [`Modifier`], [`Interactor`], and the [`Modify`] extension trait
//! that enable SwiftUI/Compose-style chainable modifications on any iced widget.
//!
//! # Exports
//!
//! - [`Modifier`] -- pure styling/layout modifier (no `Message` generic).
//! - [`Interactor`] -- modifier with interaction handlers (click, hover, scroll, etc.).
//! - [`ModifyBase`] -- trait with all chainable style, layout, and extras methods.
//! - [`Modify`] -- extension trait adding `.modify()` to any iced widget.
//! - [`IntoModified`] -- conversion trait used internally by [`Modify`] and [`modify()`].
//! - [`modify()`] -- free function alternative that works without importing traits.
//!
//! See the [crate-level documentation](crate) for usage examples.

pub(crate) mod accumulator;
pub(crate) mod build;
pub(crate) mod bundle;
mod ext;

pub use bundle::{Interactor, Modifier, ModifyBase};
pub use ext::{IntoModified, Modify};

/// Apply a `Modifier` to any widget, returning an `Element` directly.
///
/// This free function works without importing the `Modify` trait.
///
/// # Example
/// ```ignore
/// use iced_modifier::{modify, Modifier, ModifyBase};
/// use iced::widget::text;
///
/// let element: Element<Message> = modify(text("Hello"), Modifier::new().padding(16));
/// ```
pub fn modify<'a, Message>(
    widget: impl Into<iced::Element<'a, Message>>,
    modifier: impl ext::IntoModified<Message>,
) -> iced::Element<'a, Message>
where
    Message: Clone + 'a,
{
    use build::build_element;

    let element = widget.into();
    let (layers, extras, interactions) = modifier.into_parts();

    if layers.is_empty()
        && !extras.hidden
        && !interactions.has_content()
        && extras.tooltip.is_none()
        && extras.scrollable.is_none()
        && extras.widget_id.is_none()
    {
        return element;
    }

    build_element(element, &layers, extras, interactions)
}
