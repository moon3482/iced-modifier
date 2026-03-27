use iced::widget::{container, mouse_area, scrollable, text, tooltip, Container, Space};
use iced::Element;

use super::accumulator::{Extras, Interactions, Layer, ScrollDirection};

pub(crate) fn build_element<'a, Message, Theme, Renderer>(
    element: Element<'a, Message, Theme, Renderer>,
    layers: &[Layer],
    extras: Extras,
    interactions: Interactions<Message>,
) -> Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: container::Catalog + scrollable::Catalog + text::Catalog + 'a,
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer + 'a,
    <Theme as container::Catalog>::Class<'a>: From<container::StyleFn<'a, Theme>>,
{
    // Hidden: replace with empty Space
    if extras.hidden {
        return Space::new().into();
    }

    let mut current = element;

    // Apply style/layout layers via Container wrapping
    for (i, layer) in layers.iter().enumerate() {
        let style_acc = layer.style;
        let mut inner = Container::new(current);

        if let Some(padding) = layer.layout.padding {
            inner = inner.padding(padding);
        }
        if let Some(width) = layer.layout.width {
            inner = inner.width(width);
        }
        if let Some(height) = layer.layout.height {
            inner = inner.height(height);
        }
        if let Some(max_width) = layer.layout.max_width {
            inner = inner.max_width(max_width);
        }
        if let Some(max_height) = layer.layout.max_height {
            inner = inner.max_height(max_height);
        }
        if let Some(h_align) = layer.layout.horizontal_alignment {
            inner = inner.align_x(h_align);
        }
        if let Some(v_align) = layer.layout.vertical_alignment {
            inner = inner.align_y(v_align);
        }
        if let Some(clip) = layer.layout.clip {
            inner = inner.clip(clip);
        }

        // Apply widget ID on the first (innermost) container
        if i == 0 {
            if let Some(ref id) = extras.widget_id {
                inner = inner.id(id.clone());
            }
        }

        if style_acc.has_content() {
            inner = inner.style(move |_theme: &Theme| style_acc.to_container_style());
        }

        current = inner.into();

        if let Some(margin) = layer.layout.margin {
            current = Container::new(current).padding(margin).into();
        }
    }

    // Wrap with MouseArea if any interaction is set
    if interactions.has_content() {
        let mut area = mouse_area(current);

        if let Some(msg) = interactions.on_press {
            area = area.on_press(msg);
        }
        if let Some(msg) = interactions.on_release {
            area = area.on_release(msg);
        }
        if let Some(msg) = interactions.on_double_click {
            area = area.on_double_click(msg);
        }
        if let Some(msg) = interactions.on_right_press {
            area = area.on_right_press(msg);
        }
        if let Some(msg) = interactions.on_right_release {
            area = area.on_right_release(msg);
        }
        if let Some(msg) = interactions.on_middle_press {
            area = area.on_middle_press(msg);
        }
        if let Some(msg) = interactions.on_middle_release {
            area = area.on_middle_release(msg);
        }
        if let Some(msg) = interactions.on_enter {
            area = area.on_enter(msg);
        }
        if let Some(msg) = interactions.on_exit {
            area = area.on_exit(msg);
        }
        if let Some(cursor) = interactions.cursor {
            area = area.interaction(cursor);
        }

        current = area.into();
    }

    // Wrap with Tooltip if set
    if let Some((tip_text, position)) = extras.tooltip_text {
        current = tooltip(current, text(tip_text), position)
            .gap(4)
            .into();
    }

    // Wrap with Scrollable if set
    if let Some(direction) = extras.scrollable {
        current = match direction {
            ScrollDirection::Vertical => scrollable(current).into(),
            ScrollDirection::Horizontal => scrollable(current).horizontal().into(),
            ScrollDirection::Both => {
                scrollable::Scrollable::with_direction(
                    current,
                    scrollable::Direction::Both {
                        vertical: scrollable::Scrollbar::default(),
                        horizontal: scrollable::Scrollbar::default(),
                    },
                )
                .into()
            }
        };
    }

    current
}
