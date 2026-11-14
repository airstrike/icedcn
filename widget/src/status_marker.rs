//! Inline notes and labeled separators for an activity or conversation.
use iced_core::Length::Fill;
use iced_core::{Alignment, Element};
use iced_widget::Renderer;
use icedcn_core::theme::{
    container::Transparent,
    tweak::{Fg, Token},
};
use icedcn_core::{Theme, font_size};

use crate::{Compose, Inherited, Slot, column, container, row, separator};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Style {
    #[default]
    Default,
    Separator,
    Border,
}
pub use crate::status_shimmer::shimmer;
pub use Style::{Border, Separator};

pub struct StatusMarker<'a, Message> {
    content: Slot<'a, Message>,
    icon: Option<Slot<'a, Message>>,
    style: Style,
    stacked: bool,
}

pub fn status_marker<'a, Message: 'a>(
    content: impl Into<Slot<'a, Message>>,
) -> StatusMarker<'a, Message> {
    StatusMarker {
        content: content.into(),
        icon: None,
        style: Style::Default,
        stacked: false,
    }
}

impl<'a, Message: 'a> StatusMarker<'a, Message> {
    pub fn icon(mut self, icon: impl Into<Slot<'a, Message>>) -> Self {
        self.icon = Some(icon.into());
        self
    }
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
    pub fn stacked(mut self, stacked: bool) -> Self {
        self.stacked = stacked;
        self
    }
}

impl<'a, Message: 'a> Compose<Message> for StatusMarker<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.content.inherit(from);
        if let Some(icon) = &mut self.icon {
            icon.inherit(from);
        }
    }
    fn compose<'b>(mut self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        self.content.inherit(
            &Inherited::default()
                .with_size(font_size::SM)
                .with_line_height(20.0),
        );
        let icon: Option<Slot<'a, Message>> = self
            .icon
            .take()
            .map(|icon| container(icon).width(16).height(16).center(16).into());
        let row: Slot<'a, Message> = match icon {
            Some(icon) if self.stacked => column![icon, self.content]
                .spacing(8)
                .align_x(Alignment::Center)
                .into(),
            Some(icon) => row![icon, self.content].spacing(8).into(),
            None => self.content,
        };
        let area: Slot<'a, Message> = match self.style {
            Style::Default => row,
            Style::Separator => row![separator(), row, separator()]
                .spacing(12)
                .width(Fill)
                .into(),
            Style::Border => column![row, separator()].spacing(8).width(Fill).into(),
        };
        container(area)
            .width(Fill)
            .class(Transparent + Fg(Token::MutedForeground))
            .into()
    }
}

impl<'a, Message: 'a> From<StatusMarker<'a, Message>> for Slot<'a, Message> {
    fn from(marker: StatusMarker<'a, Message>) -> Self {
        Slot::new(marker)
    }
}
impl<'a, Message: 'a> From<StatusMarker<'a, Message>> for Element<'a, Message, Theme, Renderer> {
    fn from(marker: StatusMarker<'a, Message>) -> Self {
        Box::new(marker).compose()
    }
}
