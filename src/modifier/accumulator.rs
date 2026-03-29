use std::fmt;
use std::sync::Arc;

use iced::widget::{container, scrollable, tooltip};
use iced::{alignment, mouse, Background, Border, Color, Length, Padding, Point, Shadow, widget};

#[derive(Debug, Clone, Copy, Default)]
pub struct StyleAccumulator {
    pub background: Option<Background>,
    pub border: Option<Border>,
    pub shadow: Option<Shadow>,
    pub text_color: Option<Color>,
}

impl StyleAccumulator {
    pub fn to_container_style(&self) -> container::Style {
        let mut style = container::Style::default();
        if let Some(bg) = self.background {
            style.background = Some(bg);
        }
        if let Some(border) = self.border {
            style.border = border;
        }
        if let Some(shadow) = self.shadow {
            style.shadow = shadow;
        }
        if let Some(color) = self.text_color {
            style.text_color = Some(color);
        }
        style
    }

    pub fn has_content(&self) -> bool {
        self.background.is_some()
            || self.border.is_some()
            || self.shadow.is_some()
            || self.text_color.is_some()
    }

    pub fn merge(&mut self, other: &StyleAccumulator) {
        if let Some(v) = other.background {
            self.background = Some(v);
        }
        if let Some(v) = other.border {
            self.border = Some(v);
        }
        if let Some(v) = other.shadow {
            self.shadow = Some(v);
        }
        if let Some(v) = other.text_color {
            self.text_color = Some(v);
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct LayoutAccumulator {
    pub padding: Option<Padding>,
    pub margin: Option<Padding>,
    pub width: Option<Length>,
    pub height: Option<Length>,
    pub max_width: Option<f32>,
    pub max_height: Option<f32>,
    pub horizontal_alignment: Option<alignment::Horizontal>,
    pub vertical_alignment: Option<alignment::Vertical>,
    pub clip: Option<bool>,
}

impl LayoutAccumulator {
    pub fn has_content(&self) -> bool {
        self.padding.is_some()
            || self.width.is_some()
            || self.height.is_some()
            || self.max_width.is_some()
            || self.max_height.is_some()
            || self.horizontal_alignment.is_some()
            || self.vertical_alignment.is_some()
            || self.clip.is_some()
    }

    pub fn has_margin(&self) -> bool {
        self.margin.is_some()
    }

    pub fn merge(&mut self, other: &LayoutAccumulator) {
        if let Some(v) = other.padding {
            self.padding = Some(v);
        }
        if let Some(v) = other.margin {
            self.margin = Some(v);
        }
        if let Some(v) = other.width {
            self.width = Some(v);
        }
        if let Some(v) = other.height {
            self.height = Some(v);
        }
        if let Some(v) = other.max_width {
            self.max_width = Some(v);
        }
        if let Some(v) = other.max_height {
            self.max_height = Some(v);
        }
        if let Some(v) = other.horizontal_alignment {
            self.horizontal_alignment = Some(v);
        }
        if let Some(v) = other.vertical_alignment {
            self.vertical_alignment = Some(v);
        }
        if let Some(v) = other.clip {
            self.clip = Some(v);
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Layer {
    pub style: StyleAccumulator,
    pub layout: LayoutAccumulator,
}

impl Layer {
    pub fn is_empty(&self) -> bool {
        !self.style.has_content() && !self.layout.has_content() && !self.layout.has_margin()
    }
}

/// Configuration for tooltip wrapping.
#[derive(Debug, Clone)]
pub struct TooltipConfig {
    pub text: String,
    pub position: tooltip::Position,
    pub gap: Option<f32>,
    pub padding: Option<f32>,
    pub snap_within_viewport: Option<bool>,
}

/// Extra properties that apply to the entire modifier, not per-layer.
#[derive(Debug, Clone, Default)]
pub struct Extras {
    pub hidden: bool,
    pub widget_id: Option<widget::Id>,
    pub tooltip: Option<TooltipConfig>,
    pub scrollable: Option<ScrollConfig>,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ScrollDirection {
    #[default]
    Vertical,
    Horizontal,
    Both,
}

/// Configuration for scrollable wrapping.
#[derive(Debug, Clone)]
pub struct ScrollConfig {
    pub direction: ScrollDirection,
    pub id: Option<widget::Id>,
    pub anchor_x: Option<scrollable::Anchor>,
    pub anchor_y: Option<scrollable::Anchor>,
    pub spacing: Option<f32>,
}

impl ScrollConfig {
    pub fn new(direction: ScrollDirection) -> Self {
        Self {
            direction,
            id: None,
            anchor_x: None,
            anchor_y: None,
            spacing: None,
        }
    }
}

/// Interaction handlers for MouseArea wrapping.
pub struct Interactions<Message> {
    pub on_press: Option<Message>,
    pub on_release: Option<Message>,
    pub on_double_click: Option<Message>,
    pub on_right_press: Option<Message>,
    pub on_right_release: Option<Message>,
    pub on_middle_press: Option<Message>,
    pub on_middle_release: Option<Message>,
    pub on_enter: Option<Message>,
    pub on_exit: Option<Message>,
    pub on_scroll: Option<Arc<dyn Fn(mouse::ScrollDelta) -> Message + Send + Sync>>,
    pub on_move: Option<Arc<dyn Fn(Point) -> Message + Send + Sync>>,
    pub cursor: Option<mouse::Interaction>,
}

impl<Message: Clone> Clone for Interactions<Message> {
    fn clone(&self) -> Self {
        Self {
            on_press: self.on_press.clone(),
            on_release: self.on_release.clone(),
            on_double_click: self.on_double_click.clone(),
            on_right_press: self.on_right_press.clone(),
            on_right_release: self.on_right_release.clone(),
            on_middle_press: self.on_middle_press.clone(),
            on_middle_release: self.on_middle_release.clone(),
            on_enter: self.on_enter.clone(),
            on_exit: self.on_exit.clone(),
            on_scroll: self.on_scroll.clone(),
            on_move: self.on_move.clone(),
            cursor: self.cursor,
        }
    }
}

impl<Message: fmt::Debug> fmt::Debug for Interactions<Message> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Interactions")
            .field("on_press", &self.on_press)
            .field("on_release", &self.on_release)
            .field("on_double_click", &self.on_double_click)
            .field("on_right_press", &self.on_right_press)
            .field("on_right_release", &self.on_right_release)
            .field("on_middle_press", &self.on_middle_press)
            .field("on_middle_release", &self.on_middle_release)
            .field("on_enter", &self.on_enter)
            .field("on_exit", &self.on_exit)
            .field("on_scroll", &self.on_scroll.as_ref().map(|_| ".."))
            .field("on_move", &self.on_move.as_ref().map(|_| ".."))
            .field("cursor", &self.cursor)
            .finish()
    }
}

impl<Message> Interactions<Message> {
    pub fn empty() -> Self {
        Self {
            on_press: None,
            on_release: None,
            on_double_click: None,
            on_right_press: None,
            on_right_release: None,
            on_middle_press: None,
            on_middle_release: None,
            on_enter: None,
            on_exit: None,
            on_scroll: None,
            on_move: None,
            cursor: None,
        }
    }

    pub fn has_content(&self) -> bool {
        self.on_press.is_some()
            || self.on_release.is_some()
            || self.on_double_click.is_some()
            || self.on_right_press.is_some()
            || self.on_right_release.is_some()
            || self.on_middle_press.is_some()
            || self.on_middle_release.is_some()
            || self.on_enter.is_some()
            || self.on_exit.is_some()
            || self.on_scroll.is_some()
            || self.on_move.is_some()
            || self.cursor.is_some()
    }
}

