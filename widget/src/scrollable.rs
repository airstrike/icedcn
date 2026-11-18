use iced_widget::Widget;
mod element;
pub mod widget;

pub use widget::{
    AbsoluteOffset, Anchor, AutoScroll, Direction, FADE, HIDE_DELAY, Rail, RelativeOffset,
    Scrollbar, Scroller, Status, Style, StyleFn, Viewport, Visibility,
};

use crate::compose::{Compose, Inherited, Slot};
use iced_core::Length;
use iced_core::time::Duration;
use iced_core::widget::Id;
use iced_widget::Renderer;
use icedcn_core::Theme;

/// Width of the scrollbar's rail: 1px, invisible.
pub const RAIL_WIDTH: f32 = 1.0;

/// Width of the scroller drawn over the rail.
pub const SCROLLER_WIDTH: f32 = 2.0;

/// The shared thin scrollbar uses an invisible one-pixel rail and a two-pixel thumb. Its colors
/// come from `core::theme::scrollable`.
pub fn scrollbar() -> Scrollbar {
    Scrollbar::default()
        .width(RAIL_WIDTH)
        .scroller_width(SCROLLER_WIDTH)
}

/// Create a [`Scrollable`] wrapping the given content. Mirrors
/// [`iced_widget::scrollable`] but flows through our [`Compose`](crate::Compose) system
/// so size/weight propagation and the thin [`scrollbar`] shape are
/// picked up automatically.
pub fn scrollable<'a, Message: 'a>(
    content: impl Into<Slot<'a, Message>>,
) -> Scrollable<'a, Message> {
    Scrollable::new(content)
}

/// User-facing scrollable builder. Mirrors [`iced_widget::scrollable`]'s
/// surface (id / width / height / direction / on_scroll / spacing /
/// auto_scroll / anchor_x / anchor_y) but stores style as
/// `iced_widget::scrollable::StyleFn` so `core::theme::scrollable::*`
/// variants flow through unchanged. Bridge laundering to the vendored
/// widget happens at [`Compose::compose`](crate::Compose::compose) time.
pub struct Scrollable<'a, Message> {
    content: Slot<'a, Message>,
    id: Option<Id>,
    width: Length,
    height: Length,
    direction: Direction,
    auto_scroll: bool,
    focus_group: bool,
    visibility: Visibility,
    hide_delay: Duration,
    fade: Duration,
    on_scroll: Option<Box<dyn Fn(Viewport) -> Message + 'a>>,
    style: Option<iced_widget::scrollable::StyleFn<'static, Theme>>,
}

impl<'a, Message: 'a> Scrollable<'a, Message> {
    /// Create a new scrollable wrapping the given content, scrolling
    /// vertically with the thin [`scrollbar`].
    pub fn new(content: impl Into<Slot<'a, Message>>) -> Self {
        Self {
            content: content.into(),
            id: None,
            width: Length::Shrink,
            height: Length::Shrink,
            direction: Direction::Vertical(scrollbar()),
            auto_scroll: false,
            focus_group: false,
            visibility: Visibility::Always,
            hide_delay: HIDE_DELAY,
            fade: Duration::ZERO,
            on_scroll: None,
            style: None,
        }
    }

    /// Include a horizontal attachment strip and its off-screen controls in keyboard navigation.
    pub(crate) fn focus_group(mut self) -> Self {
        self.focus_group = true;
        self
    }

    /// Set this scrollable's widget [`Id`] — required for
    /// `scroll_into_view` and other Id-based operations.
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the layout width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Set the layout height.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Configure the scroll direction (Vertical, Horizontal, or Both).
    pub fn direction(mut self, direction: impl Into<Direction>) -> Self {
        self.direction = direction.into();
        self
    }

    /// Shorthand for switching to a horizontal direction with the
    /// thin [`scrollbar`].
    pub fn horizontal(self) -> Self {
        self.direction(Direction::Horizontal(scrollbar()))
    }

    /// Set a callback fired whenever the scrollable scrolls; receives the
    /// new [`Viewport`].
    pub fn on_scroll(mut self, f: impl Fn(Viewport) -> Message + 'a) -> Self
    where
        Message: 'a,
    {
        self.on_scroll = Some(Box::new(f));
        self
    }

    /// Set a spacing between the scrollbar and the contents (embeds the
    /// scrollbar so it takes layout space rather than floating on top).
    pub fn spacing(mut self, spacing: impl Into<iced_core::Pixels>) -> Self {
        match &mut self.direction {
            Direction::Horizontal(scrollbar) | Direction::Vertical(scrollbar) => {
                *scrollbar = scrollbar.spacing(spacing);
            }
            Direction::Both { .. } => {}
        }
        self
    }

    /// When the scrollbars are shown, like Radix ScrollArea's `type`. [`Visibility::Always`]
    /// by default.
    pub fn visibility(mut self, visibility: impl Into<Visibility>) -> Self {
        self.visibility = visibility.into();
        self
    }

    /// How long an idle scrollbar stays before it hides in [`Visibility::Hover`] and
    /// [`Visibility::Scroll`], like Radix's `scrollHideDelay`. [`HIDE_DELAY`] (600ms) by
    /// default.
    pub fn hide_delay(mut self, delay: impl Into<Duration>) -> Self {
        self.hide_delay = delay.into();
        self
    }

    /// Fade a hiding scrollbar out over `duration` once its hide delay has passed, in
    /// [`Visibility::Hover`] and [`Visibility::Scroll`]. By default it disappears at once,
    /// like Radix; [`FADE`] (200ms) suits overlay-style scrollbars.
    pub fn fade(mut self, duration: impl Into<Duration>) -> Self {
        self.fade = duration.into();
        self
    }

    /// Whether middle-click should trigger auto-scrolling. Off by default.
    pub fn auto_scroll(mut self, auto_scroll: bool) -> Self {
        self.auto_scroll = auto_scroll;
        self
    }

    /// Anchor the vertical direction's scroll position.
    pub fn anchor_y(mut self, anchor: Anchor) -> Self {
        match &mut self.direction {
            Direction::Vertical(scrollbar)
            | Direction::Both {
                vertical: scrollbar,
                ..
            } => {
                *scrollbar = scrollbar.anchor(anchor);
            }
            Direction::Horizontal(_) => {}
        }
        self
    }

    /// Anchor the horizontal direction's scroll position.
    pub fn anchor_x(mut self, anchor: Anchor) -> Self {
        match &mut self.direction {
            Direction::Horizontal(scrollbar)
            | Direction::Both {
                horizontal: scrollbar,
                ..
            } => {
                *scrollbar = scrollbar.anchor(anchor);
            }
            Direction::Vertical(_) => {}
        }
        self
    }

    /// Set the style of this scrollable via a closure or fn-pointer that
    /// returns an [`iced_widget::scrollable::Style`]. Mirrors iced's stock
    /// `Scrollable::style` so existing callsites (and the variant unit
    /// structs in `theme::scrollable` whose `From<X>` impls produce
    /// `iced_widget::scrollable::StyleFn`) flow through naturally.
    pub fn style(
        mut self,
        style: impl Fn(&Theme, iced_widget::scrollable::Status) -> iced_widget::scrollable::Style
        + 'static,
    ) -> Self {
        self.style = Some(Box::new(style));
        self
    }

    /// Set the style class — accepts anything convertible into an
    /// [`iced_widget::scrollable::StyleFn`], which is how variant unit
    /// structs in `theme::scrollable` flow through.
    pub fn class(
        mut self,
        class: impl Into<iced_widget::scrollable::StyleFn<'static, Theme>>,
    ) -> Self {
        self.style = Some(class.into());
        self
    }
}

impl<'a, Message: 'a> From<Scrollable<'a, Message>>
    for iced_core::Element<'a, Message, Theme, Renderer>
{
    fn from(s: Scrollable<'a, Message>) -> Self {
        Box::new(s).compose()
    }
}

impl<'a, Message: 'a> From<Scrollable<'a, Message>> for Slot<'a, Message> {
    fn from(widget: Scrollable<'a, Message>) -> Self {
        Slot::new(widget)
    }
}

impl<'a, Message: 'a> Compose<Message> for Scrollable<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.content.inherit(from);
    }

    fn compose<'b>(self: Box<Self>) -> iced_core::Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let mut s = widget::Scrollable::with_direction(self.content.compose(), self.direction)
            .width(self.width)
            .height(self.height)
            .auto_scroll(self.auto_scroll)
            .visibility(self.visibility)
            .hide_delay(self.hide_delay)
            .fade(self.fade)
            .focus_group(self.focus_group);

        if let Some(id) = self.id {
            s = s.id(id);
        }
        if let Some(on_scroll) = self.on_scroll {
            s = s.on_scroll(move |scroll| on_scroll(scroll.viewport));
        }
        if let Some(style) = self.style {
            // Bridge: wrapper's `style` is `iced_widget::scrollable::StyleFn`
            // (so existing `From<Variant> for iced::scrollable::StyleFn` impls
            // in `core::theme::scrollable` flow naturally). At construction we
            // wrap it so the vendored widget receives a `widget::StyleFn`
            // returning our local `Style` via the field-by-field `From` impl.
            s = s.style(move |t: &Theme, status: widget::Status| {
                let iced_status: iced_widget::scrollable::Status = status.into();
                widget::Style::from((style)(t, iced_status))
            });
        }

        s.boxed()
    }
}
