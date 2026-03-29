//! Core modifier types and the [`ModifyBase`] trait.
//!
//! This module contains [`Modifier`] (pure styling), [`Interactor`] (styling + interactions),
//! and the [`ModifyBase`] trait that provides all chainable methods.

use std::sync::Arc;

use iced::border;
use iced::widget::tooltip;
use iced::{alignment, mouse, widget, Background, Border, Color, Length, Padding, Pixels, Point, Shadow};

use super::accumulator::{Extras, Interactions, Layer, ScrollConfig, ScrollDirection, TooltipConfig};

/// Shared modifier data for styling, layout, and extras.
/// Internal storage used by both [`Modifier`] and [`Interactor`].
#[derive(Debug, Clone, Default)]
pub struct ModifierData {
    pub(crate) layers: Vec<Layer>,
    pub(crate) current: Layer,
    pub(crate) extras: Extras,
}

/// Trait providing all style, layout, and extras methods.
///
/// Implemented by both [`Modifier`] (pure styling) and [`Interactor<M>`] (styling + interactions).
/// All methods consume `self` and return `Self`, enabling method chaining.
///
/// # Style Methods (스타일)
/// [`background`](Self::background), [`background_color`](Self::background_color),
/// [`border`](Self::border), [`corner_radius`](Self::corner_radius),
/// [`border_color`](Self::border_color), [`border_width`](Self::border_width),
/// [`shadow`](Self::shadow), [`text_color`](Self::text_color)
///
/// # Layout Methods (레이아웃)
/// [`padding`](Self::padding), [`margin`](Self::margin),
/// [`width`](Self::width), [`height`](Self::height),
/// [`fill_width`](Self::fill_width), [`fill_portion`](Self::fill_portion),
/// [`center`](Self::center), [`align_x`](Self::align_x), [`clip`](Self::clip)
///
/// # Extras (추가 기능)
/// [`hidden`](Self::hidden), [`id`](Self::id), [`tooltip_text`](Self::tooltip_text),
/// [`scrollable`](Self::scrollable)
///
/// # Layer & Conditional (레이어 & 조건부)
/// [`layer`](Self::layer), [`modify_if`](Self::modify_if), [`modify_if_else`](Self::modify_if_else)
pub trait ModifyBase: Sized {
    /// Returns a mutable reference to the internal modifier data.
    fn data_mut(&mut self) -> &mut ModifierData;

    // ── Layer ──

    /// Flush the current layer and start a new one.
    ///
    /// Enables order-dependent layering like Compose/SwiftUI.
    /// Properties set before `.layer()` become an inner Container;
    /// properties after become an outer Container wrapping it.
    ///
    /// 현재 레이어를 flush하고 새 레이어를 시작합니다.
    /// `.layer()` 전에 설정된 속성은 내부 Container, 이후 속성은 외부 Container가 됩니다.
    fn layer(mut self) -> Self {
        let data = self.data_mut();
        if !data.current.is_empty() {
            data.layers.push(std::mem::take(&mut data.current));
        }
        self
    }

    // ── Conditional ──

    /// Apply modifications only when `condition` is true.
    ///
    /// 조건이 true일 때만 modifier를 적용합니다.
    ///
    /// ```ignore
    /// Modifier::new()
    ///     .padding(10)
    ///     .modify_if(is_error, |m| m.background_color(Color::RED))
    /// ```
    fn modify_if(self, condition: bool, f: impl FnOnce(Self) -> Self) -> Self {
        if condition { f(self) } else { self }
    }

    /// Apply one of two modifier branches based on `condition`.
    ///
    /// 조건에 따라 두 modifier 중 하나를 적용합니다.
    fn modify_if_else(
        self,
        condition: bool,
        if_true: impl FnOnce(Self) -> Self,
        if_false: impl FnOnce(Self) -> Self,
    ) -> Self {
        if condition { if_true(self) } else { if_false(self) }
    }

    // ── Style ──

    /// Set the background (Color or Gradient).
    /// Compose: `.background()` / SwiftUI: `.background()`
    fn background(mut self, bg: impl Into<Background>) -> Self {
        self.data_mut().current.style.background = Some(bg.into());
        self
    }

    /// Set a solid background color.
    /// Compose: `.background(Color.Blue)` / SwiftUI: `.background(Color.blue)`
    fn background_color(mut self, color: impl Into<Color>) -> Self {
        self.data_mut().current.style.background = Some(Background::Color(color.into()));
        self
    }

    /// Set the full border (color, width, radius).
    /// Compose: `.border()` / SwiftUI: `.border()`
    fn border(mut self, border: impl Into<Border>) -> Self {
        self.data_mut().current.style.border = Some(border.into());
        self
    }

    /// Set corner radius without affecting other border properties.
    /// Compose: `.clip(RoundedCornerShape(8.dp))` / SwiftUI: `.cornerRadius(8)`
    fn corner_radius(mut self, radius: impl Into<border::Radius>) -> Self {
        let radius = radius.into();
        let d = self.data_mut();
        let mut b = d.current.style.border.unwrap_or_default();
        b.radius = radius;
        d.current.style.border = Some(b);
        self
    }

    /// Set border color without affecting width or radius.
    fn border_color(mut self, color: impl Into<Color>) -> Self {
        let color = color.into();
        let d = self.data_mut();
        let mut b = d.current.style.border.unwrap_or_default();
        b.color = color;
        d.current.style.border = Some(b);
        self
    }

    /// Set border width without affecting color or radius.
    fn border_width(mut self, width: impl Into<Pixels>) -> Self {
        let width = width.into();
        let d = self.data_mut();
        let mut b = d.current.style.border.unwrap_or_default();
        b.width = width.0;
        d.current.style.border = Some(b);
        self
    }

    /// Set drop shadow.
    /// Compose: `.shadow()` / SwiftUI: `.shadow()`
    fn shadow(mut self, shadow: impl Into<Shadow>) -> Self {
        self.data_mut().current.style.shadow = Some(shadow.into());
        self
    }

    /// Set text color for child text widgets.
    /// Compose: `color` parameter / SwiftUI: `.foregroundColor()`
    fn text_color(mut self, color: impl Into<Color>) -> Self {
        self.data_mut().current.style.text_color = Some(color.into());
        self
    }

    // ── Layout ──

    /// Set padding on all sides. Accepts `u16`, `f32`, `[f32; 2]`, or `Padding`.
    /// Compose: `.padding()` / SwiftUI: `.padding()`
    fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.data_mut().current.layout.padding = Some(padding.into());
        self
    }

    /// Set horizontal padding (left + right) only.
    fn padding_x(mut self, padding: impl Into<Pixels>) -> Self {
        let px = padding.into();
        let d = self.data_mut();
        let mut p = d.current.layout.padding.unwrap_or(Padding::ZERO);
        p.left = px.0;
        p.right = px.0;
        d.current.layout.padding = Some(p);
        self
    }

    /// Set vertical padding (top + bottom) only.
    fn padding_y(mut self, padding: impl Into<Pixels>) -> Self {
        let px = padding.into();
        let d = self.data_mut();
        let mut p = d.current.layout.padding.unwrap_or(Padding::ZERO);
        p.top = px.0;
        p.bottom = px.0;
        d.current.layout.padding = Some(p);
        self
    }

    /// Set outer margin. Implemented as padding on a wrapping Container.
    fn margin(mut self, margin: impl Into<Padding>) -> Self {
        self.data_mut().current.layout.margin = Some(margin.into());
        self
    }

    /// Set width. Accepts `Length`, `f32` (Fixed), `u32` (Fixed).
    /// Compose: `.width()` / SwiftUI: `.frame(width:)`
    fn width(mut self, width: impl Into<Length>) -> Self {
        self.data_mut().current.layout.width = Some(width.into());
        self
    }

    /// Set height. Accepts `Length`, `f32` (Fixed), `u32` (Fixed).
    /// Compose: `.height()` / SwiftUI: `.frame(height:)`
    fn height(mut self, height: impl Into<Length>) -> Self {
        self.data_mut().current.layout.height = Some(height.into());
        self
    }

    /// Set maximum width.
    fn max_width(mut self, max_width: impl Into<Pixels>) -> Self {
        self.data_mut().current.layout.max_width = Some(max_width.into().0);
        self
    }

    /// Set maximum height.
    fn max_height(mut self, max_height: impl Into<Pixels>) -> Self {
        self.data_mut().current.layout.max_height = Some(max_height.into().0);
        self
    }

    /// Fill all available width. Shorthand for `.width(Length::Fill)`.
    /// Compose: `.fillMaxWidth()`
    fn fill_width(self) -> Self { self.width(Length::Fill) }

    /// Fill all available height. Shorthand for `.height(Length::Fill)`.
    /// Compose: `.fillMaxHeight()`
    fn fill_height(self) -> Self { self.height(Length::Fill) }

    /// Fill both axes. Shorthand for `.fill_width().fill_height()`.
    /// Compose: `.fillMaxSize()`
    fn fill(self) -> Self { self.fill_width().fill_height() }

    /// Fill a proportional portion of available width.
    /// Compose: `.weight()` / `.fillMaxWidth(fraction)`
    fn fill_portion(self, portion: u16) -> Self {
        self.width(Length::FillPortion(portion))
    }

    /// Center horizontally within the given width.
    /// Sets horizontal alignment to center and width.
    fn center_x(mut self, width: impl Into<Length>) -> Self {
        let d = self.data_mut();
        d.current.layout.horizontal_alignment = Some(alignment::Horizontal::Center);
        d.current.layout.width = Some(width.into());
        self
    }

    /// Center vertically within the given height.
    /// Sets vertical alignment to center and height.
    fn center_y(mut self, height: impl Into<Length>) -> Self {
        let d = self.data_mut();
        d.current.layout.vertical_alignment = Some(alignment::Vertical::Center);
        d.current.layout.height = Some(height.into());
        self
    }

    /// Center both axes. Shorthand for `.center_x(length).center_y(length)`.
    fn center(self, length: impl Into<Length>) -> Self {
        let length = length.into();
        self.center_x(length).center_y(length)
    }

    /// Set horizontal alignment.
    fn align_x(mut self, a: impl Into<alignment::Horizontal>) -> Self {
        self.data_mut().current.layout.horizontal_alignment = Some(a.into());
        self
    }

    /// Set vertical alignment.
    fn align_y(mut self, a: impl Into<alignment::Vertical>) -> Self {
        self.data_mut().current.layout.vertical_alignment = Some(a.into());
        self
    }

    /// Align left and set width.
    fn align_left(self, width: impl Into<Length>) -> Self {
        let mut this = self.align_x(alignment::Horizontal::Left);
        this.data_mut().current.layout.width = Some(width.into());
        this
    }

    /// Align right and set width.
    fn align_right(self, width: impl Into<Length>) -> Self {
        let mut this = self.align_x(alignment::Horizontal::Right);
        this.data_mut().current.layout.width = Some(width.into());
        this
    }

    /// Align top and set height.
    /// SwiftUI: `.frame(alignment: .top)` / Compose: `Alignment.Top`
    fn align_top(self, height: impl Into<Length>) -> Self {
        let mut this = self.align_y(alignment::Vertical::Top);
        this.data_mut().current.layout.height = Some(height.into());
        this
    }

    /// Align bottom and set height.
    /// SwiftUI: `.frame(alignment: .bottom)` / Compose: `Alignment.Bottom`
    fn align_bottom(self, height: impl Into<Length>) -> Self {
        let mut this = self.align_y(alignment::Vertical::Bottom);
        this.data_mut().current.layout.height = Some(height.into());
        this
    }

    /// Enable or disable content clipping.
    fn clip(mut self, clip: bool) -> Self {
        self.data_mut().current.layout.clip = Some(clip);
        self
    }

    // ── Extras ──

    /// Hide or show the widget. When hidden, replaced with an empty `Space`.
    /// Compose: `AnimatedVisibility` / SwiftUI: `.hidden()`
    fn hidden(mut self, hidden: bool) -> Self {
        self.data_mut().extras.hidden = hidden;
        self
    }

    /// Set a widget ID for identification and targeting.
    /// Compose: `.testTag()` / SwiftUI: `.id()`
    fn id(mut self, id: impl Into<widget::Id>) -> Self {
        self.data_mut().extras.widget_id = Some(id.into());
        self
    }

    /// Add a text tooltip at the given position.
    /// Compose: `TooltipBox` / SwiftUI: `.help()`
    fn tooltip_text(mut self, text: impl Into<String>, position: tooltip::Position) -> Self {
        self.data_mut().extras.tooltip = Some(TooltipConfig {
            text: text.into(),
            position,
            gap: None,
            padding: None,
            snap_within_viewport: None,
        });
        self
    }

    /// Set the gap between the content and the tooltip.
    /// Only effective after calling [`tooltip_text`](Self::tooltip_text).
    fn tooltip_gap(mut self, gap: f32) -> Self {
        if let Some(ref mut config) = self.data_mut().extras.tooltip {
            config.gap = Some(gap);
        }
        self
    }

    /// Set internal padding of the tooltip.
    /// Only effective after calling [`tooltip_text`](Self::tooltip_text).
    fn tooltip_padding(mut self, padding: impl Into<Pixels>) -> Self {
        if let Some(ref mut config) = self.data_mut().extras.tooltip {
            config.padding = Some(padding.into().0);
        }
        self
    }

    /// Snap the tooltip within the viewport bounds.
    /// Only effective after calling [`tooltip_text`](Self::tooltip_text).
    fn tooltip_snap(mut self, snap: bool) -> Self {
        if let Some(ref mut config) = self.data_mut().extras.tooltip {
            config.snap_within_viewport = Some(snap);
        }
        self
    }

    /// Make content vertically scrollable.
    /// Compose: `.verticalScroll()` / SwiftUI: `ScrollView`
    fn scrollable(mut self) -> Self {
        let d = self.data_mut();
        match d.extras.scrollable {
            Some(ref mut config) => config.direction = ScrollDirection::Vertical,
            None => d.extras.scrollable = Some(ScrollConfig::new(ScrollDirection::Vertical)),
        }
        self
    }

    /// Make content horizontally scrollable.
    /// Compose: `.horizontalScroll()` / SwiftUI: `ScrollView(.horizontal)`
    fn scrollable_x(mut self) -> Self {
        let d = self.data_mut();
        match d.extras.scrollable {
            Some(ref mut config) => config.direction = ScrollDirection::Horizontal,
            None => d.extras.scrollable = Some(ScrollConfig::new(ScrollDirection::Horizontal)),
        }
        self
    }

    /// Make content scrollable in both directions.
    fn scrollable_xy(mut self) -> Self {
        let d = self.data_mut();
        match d.extras.scrollable {
            Some(ref mut config) => config.direction = ScrollDirection::Both,
            None => d.extras.scrollable = Some(ScrollConfig::new(ScrollDirection::Both)),
        }
        self
    }

    /// Set a widget ID for the scrollable wrapper.
    /// Only effective after calling a scrollable method.
    fn scrollable_id(mut self, id: impl Into<widget::Id>) -> Self {
        if let Some(ref mut config) = self.data_mut().extras.scrollable {
            config.id = Some(id.into());
        }
        self
    }

    /// Anchor scrollable content to the bottom (vertical scroll starts at end).
    /// Only effective after calling a scrollable method.
    fn scroll_anchor_bottom(mut self) -> Self {
        if let Some(ref mut config) = self.data_mut().extras.scrollable {
            config.anchor_y = Some(iced::widget::scrollable::Anchor::End);
        }
        self
    }

    /// Anchor scrollable content to the right (horizontal scroll starts at end).
    /// Only effective after calling a scrollable method.
    fn scroll_anchor_right(mut self) -> Self {
        if let Some(ref mut config) = self.data_mut().extras.scrollable {
            config.anchor_x = Some(iced::widget::scrollable::Anchor::End);
        }
        self
    }

    /// Set spacing between scrollbar and content.
    /// Only effective after calling a scrollable method.
    fn scroll_spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        if let Some(ref mut config) = self.data_mut().extras.scrollable {
            config.spacing = Some(spacing.into().0);
        }
        self
    }
}

// ═══════════════════════════════════════════════════════════════
// Modifier: pure styling (no Message generic)
// ═══════════════════════════════════════════════════════════════

/// A composable set of visual and layout modifications for iced widgets.
///
/// `Modifier` has no generic parameters, making it easy to store, reuse, and compose.
/// For interactions (click, hover), call `.on_press()` etc., which converts to [`Interactor<Message>`].
///
/// `Modifier`는 제네릭 파라미터가 없어 저장, 재사용, 합성이 쉽습니다.
/// 인터랙션이 필요하면 `.on_press()` 등을 호출하여 [`Interactor`]로 전환됩니다.
///
/// # Example
/// ```ignore
/// use iced_modifier::prelude::*;
/// use iced::Color;
///
/// // Reusable style function
/// fn card() -> Modifier {
///     Modifier::new()
///         .background_color(Color::WHITE)
///         .corner_radius(12)
///         .padding(16)
/// }
///
/// // Use directly
/// text("A").modify(card());
///
/// // Compose with then()
/// text("B").modify(card().then(Modifier::new().shadow(shadow)));
///
/// // Add interactions
/// text("C").modify(card().on_press(Message::Clicked));
/// ```
#[derive(Debug, Clone, Default)]
pub struct Modifier {
    pub(crate) data: ModifierData,
}

impl ModifyBase for Modifier {
    fn data_mut(&mut self) -> &mut ModifierData { &mut self.data }
}

impl Modifier {
    /// Create a new empty `Modifier`.
    pub fn new() -> Self { Self::default() }

    /// Compose two modifiers. Values in `other` override values in `self`.
    /// Flushed layers from `other` are appended after `self`'s layers.
    ///
    /// 두 modifier를 합성합니다. `other`의 값이 `self`의 값을 덮어씁니다.
    pub fn then(mut self, other: Modifier) -> Self {
        self.data.current.style.merge(&other.data.current.style);
        self.data.current.layout.merge(&other.data.current.layout);
        for layer in other.data.layers {
            self.data.layers.push(layer);
        }
        self
    }

    // ── Interaction methods (convert Modifier → Interactor<M>) ──

    /// Set click handler. Converts to [`Interactor<M>`].
    /// Compose: `.clickable {}` / SwiftUI: `.onTapGesture {}`
    pub fn on_press<M: Clone>(self, msg: M) -> Interactor<M> {
        let mut i = Interactor::from_modifier(self);
        i.interactions.on_press = Some(msg);
        i
    }

    /// Set release handler. Converts to [`Interactor<M>`].
    pub fn on_release<M: Clone>(self, msg: M) -> Interactor<M> {
        let mut i = Interactor::from_modifier(self);
        i.interactions.on_release = Some(msg);
        i
    }

    /// Set double-click handler. Converts to [`Interactor<M>`].
    /// Compose: `.combinedClickable(onDoubleClick=)` / SwiftUI: `.onTapGesture(count: 2)`
    pub fn on_double_click<M: Clone>(self, msg: M) -> Interactor<M> {
        let mut i = Interactor::from_modifier(self);
        i.interactions.on_double_click = Some(msg);
        i
    }

    /// Set right-click handler. Converts to [`Interactor<M>`].
    pub fn on_right_press<M: Clone>(self, msg: M) -> Interactor<M> {
        let mut i = Interactor::from_modifier(self);
        i.interactions.on_right_press = Some(msg);
        i
    }

    /// Set right-button release handler. Converts to [`Interactor<M>`].
    pub fn on_right_release<M: Clone>(self, msg: M) -> Interactor<M> {
        let mut i = Interactor::from_modifier(self);
        i.interactions.on_right_release = Some(msg);
        i
    }

    /// Set middle-click handler. Converts to [`Interactor<M>`].
    pub fn on_middle_press<M: Clone>(self, msg: M) -> Interactor<M> {
        let mut i = Interactor::from_modifier(self);
        i.interactions.on_middle_press = Some(msg);
        i
    }

    /// Set middle-button release handler. Converts to [`Interactor<M>`].
    pub fn on_middle_release<M: Clone>(self, msg: M) -> Interactor<M> {
        let mut i = Interactor::from_modifier(self);
        i.interactions.on_middle_release = Some(msg);
        i
    }

    /// Set hover enter handler. Converts to [`Interactor<M>`].
    /// Compose: `.hoverable()` / SwiftUI: `.onHover {}`
    pub fn on_enter<M: Clone>(self, msg: M) -> Interactor<M> {
        let mut i = Interactor::from_modifier(self);
        i.interactions.on_enter = Some(msg);
        i
    }

    /// Set hover exit handler. Converts to [`Interactor<M>`].
    pub fn on_exit<M: Clone>(self, msg: M) -> Interactor<M> {
        let mut i = Interactor::from_modifier(self);
        i.interactions.on_exit = Some(msg);
        i
    }

    /// Set cursor style. Converts to [`Interactor<M>`].
    /// Compose: `.pointerInput()` / SwiftUI: `.cursor()`
    pub fn cursor<M: Clone>(self, cursor: mouse::Interaction) -> Interactor<M> {
        let mut i = Interactor::from_modifier(self);
        i.interactions.cursor = Some(cursor);
        i
    }

    /// Set scroll event handler. Converts to [`Interactor<M>`].
    /// Receives [`mouse::ScrollDelta`] with scroll direction and amount.
    pub fn on_scroll<M: Clone>(self, f: impl Fn(mouse::ScrollDelta) -> M + Send + Sync + 'static) -> Interactor<M> {
        let mut i = Interactor::from_modifier(self);
        i.interactions.on_scroll = Some(Arc::new(f));
        i
    }

    /// Set mouse move handler. Converts to [`Interactor<M>`].
    /// Receives [`Point`] with cursor position.
    /// Compose: `.pointerInput()` / SwiftUI: `.onContinuousHover()`
    pub fn on_move<M: Clone>(self, f: impl Fn(Point) -> M + Send + Sync + 'static) -> Interactor<M> {
        let mut i = Interactor::from_modifier(self);
        i.interactions.on_move = Some(Arc::new(f));
        i
    }
}

// ═══════════════════════════════════════════════════════════════
// Interactor<Message>: styling + interactions
// ═══════════════════════════════════════════════════════════════

/// A modifier with interaction handlers (click, hover, cursor, etc.).
///
/// Created by calling `.on_press()`, `.on_enter()`, `.cursor()`, etc. on a [`Modifier`].
/// All styling methods from [`ModifyBase`] remain available via method chaining.
///
/// [`Modifier`]에서 `.on_press()`, `.on_enter()` 등을 호출하면 생성됩니다.
/// [`ModifyBase`]의 모든 스타일 메서드를 계속 체이닝할 수 있습니다.
///
/// # Example
/// ```ignore
/// Modifier::new()
///     .padding(10)
///     .on_press(Message::Clicked)        // → Interactor<Message>
///     .background_color(Color::WHITE)    // ModifyBase method still works
///     .on_enter(Message::Hovered(true))  // Interactor method
///     .cursor(mouse::Interaction::Pointer)
/// ```
#[derive(Debug, Clone)]
pub struct Interactor<Message> {
    pub(crate) data: ModifierData,
    pub(crate) interactions: Interactions<Message>,
}

impl<M> ModifyBase for Interactor<M> {
    fn data_mut(&mut self) -> &mut ModifierData { &mut self.data }
}

impl<M: Clone> Interactor<M> {
    fn from_modifier(m: Modifier) -> Self {
        Interactor {
            data: m.data,
            interactions: Interactions::empty(),
        }
    }

    /// Set or override click handler.
    pub fn on_press(mut self, msg: M) -> Self {
        self.interactions.on_press = Some(msg);
        self
    }

    /// Set or override release handler.
    pub fn on_release(mut self, msg: M) -> Self {
        self.interactions.on_release = Some(msg);
        self
    }

    /// Set or override double-click handler.
    pub fn on_double_click(mut self, msg: M) -> Self {
        self.interactions.on_double_click = Some(msg);
        self
    }

    /// Set or override right-click handler.
    pub fn on_right_press(mut self, msg: M) -> Self {
        self.interactions.on_right_press = Some(msg);
        self
    }

    /// Set or override right-button release handler.
    pub fn on_right_release(mut self, msg: M) -> Self {
        self.interactions.on_right_release = Some(msg);
        self
    }

    /// Set or override middle-click handler.
    pub fn on_middle_press(mut self, msg: M) -> Self {
        self.interactions.on_middle_press = Some(msg);
        self
    }

    /// Set or override middle-button release handler.
    pub fn on_middle_release(mut self, msg: M) -> Self {
        self.interactions.on_middle_release = Some(msg);
        self
    }

    /// Set or override hover enter handler.
    pub fn on_enter(mut self, msg: M) -> Self {
        self.interactions.on_enter = Some(msg);
        self
    }

    /// Set or override hover exit handler.
    pub fn on_exit(mut self, msg: M) -> Self {
        self.interactions.on_exit = Some(msg);
        self
    }

    /// Set or override cursor style.
    pub fn cursor(mut self, cursor: mouse::Interaction) -> Self {
        self.interactions.cursor = Some(cursor);
        self
    }

    /// Set or override scroll event handler.
    pub fn on_scroll(mut self, f: impl Fn(mouse::ScrollDelta) -> M + Send + Sync + 'static) -> Self {
        self.interactions.on_scroll = Some(Arc::new(f));
        self
    }

    /// Set or override mouse move handler.
    pub fn on_move(mut self, f: impl Fn(Point) -> M + Send + Sync + 'static) -> Self {
        self.interactions.on_move = Some(Arc::new(f));
        self
    }
}
