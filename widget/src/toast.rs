//! Short-lived notifications displayed in a corner of the window.
//!
//! A [`Toaster`] wraps part of the interface (usually all of it) and shows
//! the toasts raised inside it in a corner of the window, each for a few
//! seconds. A toast is raised by pressing a [`trigger`] inside the toaster,
//! or from `update` with the [`show`] task:
//!
//! ```ignore
//! use icedcn::widget::{button, toast, toaster};
//!
//! toaster(column![
//!     toast::trigger(
//!         button("Show Toast").style(button::Outline),
//!         toast::toast("Event has been created")
//!             .description("Sunday, December 03, 2023 at 9:00 AM")
//!             .action("Undo", Message::Undo),
//!     ),
//!     page,
//! ])
//!
//! // From `update`:
//! return toast::show(toast::toast("Saved").kind(toast::Kind::Success));
//! ```
//!
//! The toaster times its toasts out itself: it asks the runtime for a
//! redraw at the next expiry, so no app subscription is needed.

mod widget;

use std::any::Any;
use std::time::Duration;

use iced_core::Element;
use iced_runtime::Task;
use iced_runtime::task;
use iced_widget::{Renderer, Widget};
use icedcn_core::Theme;

use crate::compose::{Compose, Inherited, View};
use crate::core::Rectangle;
use crate::core::widget::{Id, Operation};
use crate::icon::Icon;
use crate::marker::{self, Press};
#[cfg(not(feature = "spinner"))]
use crate::text::Text;

/// Kind of notification, which determines the icon.
#[derive(Debug, Clone, Copy, PartialEq, Eq, std::default::Default)]
pub enum Kind {
    /// No icon.
    #[default]
    Default,
    /// A pending asynchronous operation.
    Loading,
    /// A check mark.
    Success,
    /// An information mark.
    Info,
    /// A warning triangle.
    Warning,
    /// An error mark.
    Error,
}

/// Where a toast stacks in the window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, std::default::Default)]
pub enum Position {
    /// Top left corner.
    TopLeft,
    /// Top edge, centered.
    TopCenter,
    /// Top right corner.
    TopRight,
    /// Bottom left corner.
    BottomLeft,
    /// Bottom edge, centered.
    BottomCenter,
    /// Bottom right corner (the default).
    #[default]
    BottomRight,
}

/// A toast: a title, an optional description and action.
#[derive(Debug, Clone)]
pub struct Toast<Message> {
    title: String,
    key: Option<Id>,
    description: Option<String>,
    kind: Kind,
    position: Option<Position>,
    duration: Option<Duration>,
    action: Option<(String, Message)>,
}

/// Create a [`Toast`] with the given title.
pub fn toast<Message>(title: impl Into<String>) -> Toast<Message> {
    Toast {
        title: title.into(),
        key: None,
        description: None,
        kind: Kind::Default,
        position: None,
        duration: None,
        action: None,
    }
}

impl<Message> Toast<Message> {
    /// Add a description below the title.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set what the toast is about, which picks its icon.
    pub fn kind(mut self, kind: Kind) -> Self {
        self.kind = kind;
        self
    }

    /// Stack this toast at `position` instead of the toaster's.
    pub fn position(mut self, position: Position) -> Self {
        self.position = Some(position);
        self
    }

    /// Show this toast for `duration` instead of the toaster's.
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Add an action button labelled `label`: pressing it publishes
    /// `message` and dismisses the toast.
    pub fn action(mut self, label: impl Into<String>, message: Message) -> Self {
        self.action = Some((label.into(), message));
        self
    }
}

/// Make `child` raise `toast` when pressed. The child keeps its own
/// handlers.
pub fn trigger<'a, Message>(
    child: impl Into<View<'a, Message>>,
    toast: Toast<Message>,
) -> View<'a, Message>
where
    Message: Clone + Send + 'static,
{
    View::new(Trigger {
        child: child.into(),
        toast,
    })
}

/// A [`Task`] that raises `toast` in the toasters of the interface.
pub fn show<T, Message>(toast: Toast<Message>) -> Task<T>
where
    T: Send + 'static,
    Message: Send + 'static,
{
    task::widget(Show(Some(toast)))
}

/// Show a pending toast until the future resolves, then update that same card.
/// Concurrent operations each own a unique card.
pub fn promise<T, Message, Value, Error>(
    loading: impl Into<String>,
    future: impl Future<Output = Result<Value, Error>> + Send + 'static,
    success: impl FnOnce(Value) -> String + Send + 'static,
    error: impl FnOnce(Error) -> String + Send + 'static,
) -> Task<T>
where
    T: Send + 'static,
    Message: Send + 'static,
    Value: Send + 'static,
    Error: Send + 'static,
{
    let key = Id::unique();
    let mut pending: Toast<Message> = toast(loading).kind(Kind::Loading);
    pending.key = Some(key.clone());
    let settled = Task::perform(
        async move {
            let mut result: Toast<Message> = match future.await {
                Ok(value) => toast(success(value)).kind(Kind::Success),
                Err(cause) => toast(error(cause)).kind(Kind::Error),
            };
            result.key = Some(key);
            result
        },
        |result| result,
    )
    .then(show);
    show(pending).chain(settled)
}

/// The signal a [`trigger`] records when pressed.
#[derive(Debug, Clone)]
struct Raise<Message>(Toast<Message>);

/// A child that raises a toast when pressed.
struct Trigger<'a, Message> {
    child: View<'a, Message>,
    toast: Toast<Message>,
}

impl<'a, Message: Clone + Send + 'static> Compose<Message> for Trigger<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.child.inherit(from);
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        marker::marker(self.child.compose(), Raise(self.toast), Press::Captured).into()
    }
}

/// The toasts an operation hands to a toaster.
struct Inbox<Message>(Vec<Toast<Message>>);

/// The [`Operation`] behind [`show`]: it gives its toast to the first
/// toaster it reaches.
struct Show<Message>(Option<Toast<Message>>);

impl<T, Message: Send + 'static> Operation<T> for Show<Message> {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<T>)) {
        if self.0.is_some() {
            operate(self);
        }
    }

    fn custom(&mut self, _id: Option<&Id>, _bounds: Rectangle, state: &mut dyn Any) {
        if let Some(inbox) = state.downcast_mut::<Inbox<Message>>()
            && let Some(toast) = self.0.take()
        {
            inbox.0.push(toast);
        }
    }
}

fn default_icon<'a, Message: 'a>(kind: Kind) -> Option<View<'a, Message>> {
    let icon = match kind {
        Kind::Default => return None,
        Kind::Success => Icon::CircleCheck,
        Kind::Info => Icon::Info,
        Kind::Warning => Icon::TriangleAlert,
        Kind::Error => Icon::OctagonX,
        #[cfg(feature = "spinner")]
        Kind::Loading => return Some(crate::spinner().size(16.).into()),
        #[cfg(not(feature = "spinner"))]
        Kind::Loading => return Some(Text::new("…").size(16).into()),
    };
    Some(icon.boxed().into())
}

/// Builds the icon for a toast of the given [`Kind`].
type Icons<'a, Message> = Box<dyn Fn(Kind) -> Option<View<'a, Message>> + 'a>;

/// Displays toasts raised inside its content.
pub struct Toaster<'a, Message> {
    content: View<'a, Message>,
    position: Position,
    duration: Duration,
    icons: Icons<'a, Message>,
}

/// Create a [`Toaster`] showing the toasts raised inside `content`.
pub fn toaster<'a, Message: 'a>(content: impl Into<View<'a, Message>>) -> Toaster<'a, Message> {
    Toaster {
        content: content.into(),
        position: Position::BottomRight,
        duration: Duration::from_secs(4),
        icons: Box::new(default_icon),
    }
}

impl<'a, Message: 'a> Toaster<'a, Message> {
    /// Stack toasts at `position` (bottom right by default).
    pub fn position(mut self, position: Position) -> Self {
        self.position = position;
        self
    }

    /// Show each toast for `duration` (4 seconds by default).
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Build the icons of the toast kinds, typically lucide's
    /// `circle-check`, `info`, `triangle-alert` and `octagon-x`; `None`
    /// leaves a kind without one.
    pub fn icons(mut self, icons: impl Fn(Kind) -> Option<View<'a, Message>> + 'a) -> Self {
        self.icons = Box::new(icons);
        self
    }
}

impl<'a, Message: Clone + Send + 'static> From<Toaster<'a, Message>> for View<'a, Message> {
    fn from(toaster: Toaster<'a, Message>) -> Self {
        View::new(toaster)
    }
}

impl<'a, Message: Clone + Send + 'static> From<Toaster<'a, Message>>
    for Element<'a, Message, Theme, Renderer>
{
    fn from(toaster: Toaster<'a, Message>) -> Self {
        Box::new(toaster).compose()
    }
}

impl<'a, Message: Clone + Send + 'static> Compose<Message> for Toaster<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.content.inherit(from);
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Toaster {
            content,
            position,
            duration,
            icons,
        } = *self;

        widget::Toaster::new(content.compose(), position, duration, icons).boxed()
    }
}
