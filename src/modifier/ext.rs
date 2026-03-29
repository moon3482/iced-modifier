use iced::Element;
use iced::widget::{container, scrollable, text};

use super::accumulator::{Extras, Interactions, Layer};
use super::build::build_element;
use super::bundle::{Interactor, Modifier};

/// Trait for types that can be materialized into an Element.
/// Implemented by both `Modifier` (pure styling) and `Interactor<M>` (with interactions).
pub trait IntoModified<Message> {
    fn into_parts(self) -> (Vec<Layer>, Extras, Interactions<Message>);
}

impl<M> IntoModified<M> for Modifier {
    fn into_parts(self) -> (Vec<Layer>, Extras, Interactions<M>) {
        let mut layers = self.data.layers;
        if !self.data.current.is_empty() {
            layers.push(self.data.current);
        }
        (layers, self.data.extras, Interactions::empty())
    }
}

impl<M> IntoModified<M> for Interactor<M> {
    fn into_parts(self) -> (Vec<Layer>, Extras, Interactions<M>) {
        let mut layers = self.data.layers;
        if !self.data.current.is_empty() {
            layers.push(self.data.current);
        }
        (layers, self.data.extras, self.interactions)
    }
}

/// Extension trait that adds `.modify(modifier)` to any iced widget.
///
/// Returns `Element` directly — no `.into()` needed.
///
/// Accepts both `Modifier` (pure styling) and `Interactor<Message>` (with interactions).
pub trait Modify<'a, Message, Theme, Renderer>: Sized {
    fn modify(self, modifier: impl IntoModified<Message>) -> Element<'a, Message, Theme, Renderer>;
}

impl<'a, Message, Theme, Renderer, W> Modify<'a, Message, Theme, Renderer> for W
where
    W: Into<Element<'a, Message, Theme, Renderer>>,
    Message: Clone + 'a,
    Theme: container::Catalog + scrollable::Catalog + text::Catalog + 'a,
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer + 'a,
    <Theme as container::Catalog>::Class<'a>: From<container::StyleFn<'a, Theme>>,
{
    fn modify(self, modifier: impl IntoModified<Message>) -> Element<'a, Message, Theme, Renderer> {
        let element = self.into();
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
}
