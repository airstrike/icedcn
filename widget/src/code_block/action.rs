//! Compact, labeled controls for code-block toolbars.

use iced_core::Element;
use iced_widget::Renderer;
use icedcn_core::Theme;
use icedcn_core::theme::tweak::{Fg, Token, hover};

use crate::compose::{Compose, Inherited, Slot};
use crate::{Size, button, tooltip};

/// A toolbar button with a tooltip and optional active state.
pub struct Action<'a, Message> {
    button: button::Button<'a, Message>,
    label: Slot<'a, Message>,
    active: bool,
}

/// Create a compact code action; attach the application's message with `.on_press`.
pub fn action<'a, Message: Clone + 'a>(
    content: impl Into<Slot<'a, Message>>,
    label: impl Into<Slot<'a, Message>>,
) -> Action<'a, Message> {
    Action {
        button: button(content).square().size(Size::Xs),
        label: label.into(),
        active: false,
    }
}

impl<'a, Message: Clone + 'a> Action<'a, Message> {
    /// Publish a message when activated.
    pub fn on_press(mut self, message: Message) -> Self {
        self.button = self.button.on_press(message);
        self
    }

    /// Emphasize an enabled option, such as line wrapping.
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// Prevent activation while the action is unavailable.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.button = self.button.disabled(disabled);
        self
    }
}

impl<'a, Message: Clone + 'a> Compose<Message> for Action<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.button.inherit(from);
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let ink = if self.active {
            Token::Foreground
        } else {
            Token::MutedForeground
        };
        Box::new(tooltip(
            self.button
                .style(button::Ghost + Fg(ink) + hover(Fg(Token::Foreground))),
            self.label,
        ))
        .compose()
    }
}

impl<'a, Message: Clone + 'a> From<Action<'a, Message>> for Slot<'a, Message> {
    fn from(action: Action<'a, Message>) -> Self {
        Slot::new(action)
    }
}

impl<'a, Message: Clone + 'a> From<Action<'a, Message>> for Element<'a, Message, Theme, Renderer> {
    fn from(action: Action<'a, Message>) -> Self {
        Box::new(action).compose()
    }
}
