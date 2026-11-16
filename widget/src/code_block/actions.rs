//! Buttons at the end of the header, or floating over the content.

use iced_core::Element;
use iced_core::alignment::Vertical;
use iced_widget::Renderer;
use icedcn_core::theme::container::Style;
use icedcn_core::{Border, Theme, radius};

use super::Placement;
use crate::compose::{Compose, Inherited, Slot};
use crate::{Row, Size, container};

/// Build actions from any number of buttons.
///
/// ```ignore
/// code_block::actions![
///     button(icon::download()).style(button::Ghost).square().on_press(Message::Save),
///     code_block::copy_button(CODE),
/// ]
/// ```
#[macro_export]
macro_rules! actions {
    ($first:expr $(, $rest:expr)* $(,)?) => {
        $crate::code_block::actions($first)$(.push($rest))*
    };
}

/// Buttons 4px apart. In a header they sit at the end; as a direct child of
/// the block they float at the top end of the content and show on hover or
/// focus, each on the page background with a faint ring.
///
/// Buttons default to `Size::Xs`. A ghost button in the header is muted until
/// hovered or [`pressed`](crate::Button::pressed).
pub struct Actions<'a, Message> {
    children: Vec<Slot<'a, Message>>,
    placement: Option<Placement>,
}

/// Actions holding one button.
pub fn actions<'a, Message>(child: impl Into<Slot<'a, Message>>) -> Actions<'a, Message> {
    Actions {
        children: vec![child.into()],
        placement: None,
    }
}

impl<'a, Message: 'a> Actions<'a, Message> {
    /// Add a button after the others.
    pub fn push(mut self, child: impl Into<Slot<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }
}

impl<'a, Message: 'a> Compose<Message> for Actions<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.placement = self.placement.or(from.code_actions());
        for child in &mut self.children {
            child.inherit(from);
        }
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Self {
            children,
            placement,
        } = *self;
        let size = Inherited::default().with_size(Size::Xs);
        Row::with_children(children.into_iter().map(|mut child| {
            child.inherit(&size);
            if placement == Some(Placement::Floating) {
                container(child)
                    .style(|theme: &Theme| Style {
                        background: Some(theme.neutral.base.into()),
                        border: Border {
                            width: 1.,
                            color: theme.neutral.foreground.scale_alpha(0.1),
                            radius: radius::SM.into(),
                        },
                        ..Style::default()
                    })
                    .into()
            } else {
                child
            }
        }))
        .spacing(4)
        .align_y(Vertical::Center)
        .into()
    }
}

impl<'a, Message: 'a> From<Actions<'a, Message>> for Slot<'a, Message> {
    fn from(actions: Actions<'a, Message>) -> Self {
        Slot::new(actions)
    }
}

impl<'a, Message: 'a> From<Actions<'a, Message>> for Element<'a, Message, Theme, Renderer> {
    fn from(actions: Actions<'a, Message>) -> Self {
        Box::new(actions).compose()
    }
}
