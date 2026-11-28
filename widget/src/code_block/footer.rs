//! The row below the content, such as for a Show More button.

use iced_core::Element;
use iced_core::Length::Fill;
use iced_core::alignment::{Horizontal, Vertical};
use iced_widget::Renderer;
use icedcn_core::Theme;

use crate::compose::{Compose, Inherited, View};
use crate::{Row, container};

/// A row below the content, 8px apart and muted.
pub struct Footer<'a, Message> {
    children: Vec<View<'a, Message>>,
    align_x: Horizontal,
}

/// A footer holding one child.
pub fn footer<'a, Message>(child: impl Into<View<'a, Message>>) -> Footer<'a, Message> {
    Footer {
        children: vec![child.into()],
        align_x: Horizontal::Left,
    }
}

impl<'a, Message: 'a> Footer<'a, Message> {
    /// Add a child after the others.
    pub fn push(mut self, child: impl Into<View<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Place the children along the row, like `justify-center`.
    pub fn align_x(mut self, align: impl Into<Horizontal>) -> Self {
        self.align_x = align.into();
        self
    }
}

impl<'a, Message: 'a> Compose<Message> for Footer<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        for child in &mut self.children {
            child.inherit(from);
        }
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Self { children, align_x } = *self;
        let row = Row::with_children(children)
            .spacing(8)
            .align_y(Vertical::Center);
        container(row).width(Fill).align_x(align_x).into()
    }
}

impl<'a, Message: 'a> From<Footer<'a, Message>> for View<'a, Message> {
    fn from(footer: Footer<'a, Message>) -> Self {
        View::new(footer)
    }
}

impl<'a, Message: 'a> From<Footer<'a, Message>> for Element<'a, Message, Theme, Renderer> {
    fn from(footer: Footer<'a, Message>) -> Self {
        Box::new(footer).compose()
    }
}
