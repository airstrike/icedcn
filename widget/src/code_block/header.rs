//! The row above the content: a title and actions.
use std::borrow::Cow;

use iced_core::Element;
use iced_core::Length::Fill;
use iced_core::alignment::Vertical;
use iced_core::text::Wrapping;
use iced_widget::{Renderer, Widget};
use icedcn_core::Theme;
use icedcn_core::theme::container::Style;

use super::{Actions, Placement, Spec, frame};
use crate::compose::{Compose, Inherited, Slot};
use crate::text::Ellipsis;
use crate::{Weight, container, row};

/// The lead of a [`Header`]: a [`Title`], a string (which becomes one), or any
/// other child, such as a link around a title.
pub enum HeaderItem<'a, Message> {
    Title(Title<'a, Message>),
    Other(Slot<'a, Message>),
}

impl<'a, Message> From<Title<'a, Message>> for HeaderItem<'a, Message> {
    fn from(title: Title<'a, Message>) -> Self {
        HeaderItem::Title(title)
    }
}
impl<'a, Message> From<Slot<'a, Message>> for HeaderItem<'a, Message> {
    fn from(slot: Slot<'a, Message>) -> Self {
        HeaderItem::Other(slot)
    }
}
impl<'a, Message: Clone + 'a> From<crate::Button<'a, Message>> for HeaderItem<'a, Message> {
    fn from(button: crate::Button<'a, Message>) -> Self {
        HeaderItem::Other(button.into())
    }
}
impl<'a, Message: 'a> From<&'a str> for HeaderItem<'a, Message> {
    fn from(value: &'a str) -> Self {
        HeaderItem::Title(title(value))
    }
}
impl<'a, Message: 'a> From<String> for HeaderItem<'a, Message> {
    fn from(value: String) -> Self {
        HeaderItem::Title(title(value))
    }
}
impl<'a, Message: 'a> From<Cow<'a, str>> for HeaderItem<'a, Message> {
    fn from(value: Cow<'a, str>) -> Self {
        HeaderItem::Title(title(value))
    }
}

/// A row above the content for a title and actions.
pub struct Header<'a, Message> {
    title: HeaderItem<'a, Message>,
    actions: Option<Actions<'a, Message>>,
}

/// A header led by `title`. A string is the title.
pub fn header<'a, Message>(title: impl Into<HeaderItem<'a, Message>>) -> Header<'a, Message> {
    Header {
        title: title.into(),
        actions: None,
    }
}

impl<'a, Message: 'a> Header<'a, Message> {
    /// Actions at the end of the header: [`actions`](super::actions), or a single button.
    pub fn actions(mut self, actions: impl Into<Actions<'a, Message>>) -> Self {
        self.actions = Some(actions.into());
        self
    }

    pub(super) fn inherit(&mut self, from: &Inherited) {
        match &mut self.title {
            HeaderItem::Title(title) => title.inherit(from),
            HeaderItem::Other(slot) => slot.inherit(from),
        }
        if let Some(actions) = &mut self.actions {
            actions.inherit(from);
        }
    }

    pub(super) fn view(self, compact: bool, spec: Spec) -> Element<'a, Message, Theme, Renderer> {
        let text = Inherited::default()
            .with_size(if compact {
                crate::Size::Xs
            } else {
                crate::Size::Default
            })
            .with_weight(Weight::Medium);
        let mut header = row![].width(Fill).spacing(8).align_y(Vertical::Center);
        match self.title {
            HeaderItem::Title(mut title) => {
                title.inherited = text.or(title.inherited);
                header = header.push(
                    container(title)
                        .width(Fill)
                        .clip(true)
                        .style(move |theme: &Theme| Style {
                            text_color: spec.resolve(theme).text_color,
                            ..Style::default()
                        })
                        .into(),
                );
            }
            HeaderItem::Other(mut slot) => {
                slot.inherit(&text);
                header = header.push(slot);
            }
        }
        if let Some(mut actions) = self.actions {
            actions.inherit(&Inherited::default().with_code_actions(Placement::Header));
            header = header.push(
                frame::Frame::actions(Slot::from(actions).compose())
                    .boxed()
                    .into(),
            );
        }
        header.into()
    }
}

/// A child of a [`Title`]. A string is a label that truncates.
pub enum TitleItem<'a, Message> {
    Label(Cow<'a, str>),
    Slot(Slot<'a, Message>),
}

impl<'a, Message> From<&'a str> for TitleItem<'a, Message> {
    fn from(value: &'a str) -> Self {
        TitleItem::Label(value.into())
    }
}
impl<'a, Message> From<String> for TitleItem<'a, Message> {
    fn from(value: String) -> Self {
        TitleItem::Label(value.into())
    }
}
impl<'a, Message> From<Cow<'a, str>> for TitleItem<'a, Message> {
    fn from(value: Cow<'a, str>) -> Self {
        TitleItem::Label(value)
    }
}
impl<'a, Message> From<Slot<'a, Message>> for TitleItem<'a, Message> {
    fn from(slot: Slot<'a, Message>) -> Self {
        TitleItem::Slot(slot)
    }
}
impl<'a, Message: 'a> From<iced_widget::Text<'a, Theme>> for TitleItem<'a, Message> {
    fn from(icon: iced_widget::Text<'a, Theme>) -> Self {
        TitleItem::Slot(icon.into())
    }
}
impl<'a, Message: 'a> From<crate::Text<'a>> for TitleItem<'a, Message> {
    fn from(text: crate::Text<'a>) -> Self {
        TitleItem::Slot(text.into())
    }
}
impl<'a, Message: 'a> From<Element<'a, Message, Theme, Renderer>> for TitleItem<'a, Message> {
    fn from(element: Element<'a, Message, Theme, Renderer>) -> Self {
        TitleItem::Slot(element.into())
    }
}

/// A filename or a label, with an optional icon. It truncates.
pub struct Title<'a, Message> {
    icon: Option<Slot<'a, Message>>,
    label: TitleItem<'a, Message>,
    inherited: Inherited,
}

/// A title showing `label`.
pub fn title<'a, Message>(label: impl Into<TitleItem<'a, Message>>) -> Title<'a, Message> {
    Title {
        icon: None,
        label: label.into(),
        inherited: Inherited::default(),
    }
}

impl<'a, Message: 'a> Title<'a, Message> {
    /// An icon before the label, sized 14px (`[&_svg]:size-3.5`) and 6px apart.
    pub fn icon(mut self, icon: impl Into<iced_widget::Text<'a, Theme>>) -> Self {
        self.icon = Some(icon.into().size(14).into());
        self
    }
}

impl<'a, Message: 'a> Compose<Message> for Title<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.inherited = self.inherited.or(*from);
    }

    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Self {
            icon,
            label,
            inherited,
        } = *self;
        let mut label = match label {
            TitleItem::Label(value) => crate::text(value)
                .width(Fill)
                .wrapping(Wrapping::None)
                .ellipsis(Ellipsis::End)
                .into(),
            TitleItem::Slot(slot) => slot,
        };
        label.inherit(&inherited);
        let Some(icon) = icon else {
            return label.compose();
        };
        row![icon, label]
            .width(Fill)
            .spacing(6)
            .align_y(Vertical::Center)
            .into()
    }
}

impl<'a, Message: 'a> From<Title<'a, Message>> for Slot<'a, Message> {
    fn from(title: Title<'a, Message>) -> Self {
        Slot::new(title)
    }
}
impl<'a, Message: 'a> From<Title<'a, Message>> for Element<'a, Message, Theme, Renderer> {
    fn from(title: Title<'a, Message>) -> Self {
        Box::new(title).compose()
    }
}
