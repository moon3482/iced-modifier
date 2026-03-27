/// Drag-and-drop extension trait for Element (requires `drag-drop` feature).
///
/// This wraps any `Element` with `iced_drop::droppable()`, enabling drag-and-drop
/// behavior via method chaining after `.modify()`.
///
/// # Example
/// ```ignore
/// use iced_modifier::prelude::*;
/// use iced_modifier::drag::DragExt;
///
/// text("Drag me")
///     .modify(Modifier::new().padding(10).background_color(Color::WHITE))
///     .on_drop(|point, rect| Message::Dropped(point))
///     .on_drag(|point, rect| Message::Dragging(point))
/// ```
use iced::{Element, Point, Rectangle};

pub trait DragExt<'a, Message, Theme, Renderer>: Sized {
    /// Make the element droppable. Called when the element is dropped.
    fn on_drop(
        self,
        f: impl Fn(Point, Rectangle) -> Message + 'a,
    ) -> Element<'a, Message, Theme, Renderer>;

    /// Track drag movement. Called while the element is being dragged.
    fn on_drag(
        self,
        f: impl Fn(Point, Rectangle) -> Message + 'a,
    ) -> Element<'a, Message, Theme, Renderer>;

    /// Make the element fully draggable with both drag and drop handlers.
    fn draggable(
        self,
        on_drag: impl Fn(Point, Rectangle) -> Message + 'a,
        on_drop: impl Fn(Point, Rectangle) -> Message + 'a,
    ) -> Element<'a, Message, Theme, Renderer>;
}

impl<'a, Message, Theme, Renderer> DragExt<'a, Message, Theme, Renderer>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: 'a,
    Renderer: iced::advanced::Renderer + 'a,
{
    fn on_drop(
        self,
        f: impl Fn(Point, Rectangle) -> Message + 'a,
    ) -> Element<'a, Message, Theme, Renderer> {
        iced_drop::droppable(self).on_drop(f).into()
    }

    fn on_drag(
        self,
        f: impl Fn(Point, Rectangle) -> Message + 'a,
    ) -> Element<'a, Message, Theme, Renderer> {
        iced_drop::droppable(self).on_drag(f).into()
    }

    fn draggable(
        self,
        on_drag: impl Fn(Point, Rectangle) -> Message + 'a,
        on_drop: impl Fn(Point, Rectangle) -> Message + 'a,
    ) -> Element<'a, Message, Theme, Renderer> {
        iced_drop::droppable(self)
            .on_drag(on_drag)
            .on_drop(on_drop)
            .into()
    }
}
