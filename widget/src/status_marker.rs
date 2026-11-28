//! Inline notes and labeled separators for an activity or conversation.
use crate::text;
use iced_core::Length::Fill;
use iced_core::{Alignment, Element};
use iced_widget::Renderer;
use icedcn_core::Theme;
use icedcn_core::theme::{Fg, container::Transparent};

use crate::{Compose, Inherited, View, column, container, row, separator};

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
    content: View<'a, Message>,
    icon: Option<View<'a, Message>>,
    style: Style,
    stacked: bool,
}

pub fn status_marker<'a, Message: 'a>(
    content: impl Into<View<'a, Message>>,
) -> StatusMarker<'a, Message> {
    StatusMarker {
        content: content.into(),
        icon: None,
        style: Style::Default,
        stacked: false,
    }
}

impl<'a, Message: 'a> StatusMarker<'a, Message> {
    pub fn icon(mut self, icon: impl Into<View<'a, Message>>) -> Self {
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
        self.content
            .inherit(&Inherited::default().with_size(text::size::SM));
        let icon: Option<View<'a, Message>> = self
            .icon
            .take()
            .map(|icon| container(icon).width(16).height(16).center(16).into());
        let row: View<'a, Message> = match icon {
            Some(icon) if self.stacked => column![icon, self.content]
                .spacing(8)
                .align_x(Alignment::Center)
                .into(),
            Some(icon) => row![icon, self.content].spacing(8).into(),
            None => self.content,
        };
        let area: View<'a, Message> = match self.style {
            Style::Default => row,
            Style::Separator => row![separator(), row, separator()]
                .spacing(12)
                .width(Fill)
                .into(),
            Style::Border => column![row, separator()].spacing(8).width(Fill).into(),
        };
        container(area)
            .width(Fill)
            .style(Transparent + Fg::MutedForeground)
            .into()
    }
}

impl<'a, Message: 'a> From<StatusMarker<'a, Message>> for View<'a, Message> {
    fn from(marker: StatusMarker<'a, Message>) -> Self {
        View::new(marker)
    }
}
impl<'a, Message: 'a> From<StatusMarker<'a, Message>> for Element<'a, Message, Theme, Renderer> {
    fn from(marker: StatusMarker<'a, Message>) -> Self {
        Box::new(marker).compose()
    }
}
