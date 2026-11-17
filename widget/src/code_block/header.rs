//! The row above the content: a title and actions.
use std::borrow::Cow;

use iced_core::Element;
use iced_core::Length::Fill;
use iced_core::alignment::Vertical;
use iced_core::text::Wrapping;
use iced_widget::{Renderer, Space, Widget};
use icedcn_core::Theme;
use icedcn_core::theme::container::Style;

use super::{Actions, Placement, Spec, frame};
use crate::compose::{Compose, Inherited, Slot};
use crate::text::Ellipsis;
use crate::{Weight, container, row};

/// Build a header from a title, actions, or any other child.
///
/// ```ignore
/// code_block::header!["Terminal", code_block::actions![code_block::copy_button(OUTPUT)]]
/// ```
#[macro_export]
macro_rules! header {
    ($first:expr $(, $rest:expr)* $(,)?) => {
        $crate::code_block::header($first)$(.push($rest))*
    };
}

/// Build a title from an icon and a label. A bare identifier first is read as
/// `icon::name()` at 14px, like `button!`.
///
/// ```ignore
/// code_block::title![file_code, "app/api/chat/route.ts"]
/// ```
#[macro_export]
macro_rules! title {
    ($single:expr $(,)?) => {
        $crate::code_block::title($single)
    };
    ($first:ident, $($rest:expr),+ $(,)?) => {
        $crate::code_block::title(icon::$first().size(14))$(.push($rest))+
    };
    ($first:expr, $($rest:expr),+ $(,)?) => {
        $crate::code_block::title($first)$(.push($rest))+
    };
}

/// A direct child of a [`Header`]. A string is the title.
pub enum HeaderItem<'a, Message> {
    Title(Title<'a, Message>),
    Actions(Actions<'a, Message>),
    Other(Slot<'a, Message>),
}

impl<'a, Message> From<Title<'a, Message>> for HeaderItem<'a, Message> {
    fn from(title: Title<'a, Message>) -> Self {
        HeaderItem::Title(title)
    }
}
impl<'a, Message> From<Actions<'a, Message>> for HeaderItem<'a, Message> {
    fn from(actions: Actions<'a, Message>) -> Self {
        HeaderItem::Actions(actions)
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
    items: Vec<HeaderItem<'a, Message>>,
}

/// A header holding one child. A string is the title.
pub fn header<'a, Message>(item: impl Into<HeaderItem<'a, Message>>) -> Header<'a, Message> {
    Header {
        items: vec![item.into()],
    }
}

impl<'a, Message: 'a> Header<'a, Message> {
    /// Add a child after the others.
    pub fn push(mut self, item: impl Into<HeaderItem<'a, Message>>) -> Self {
        self.items.push(item.into());
        self
    }

    pub(super) fn inherit(&mut self, from: &Inherited) {
        for item in &mut self.items {
            match item {
                HeaderItem::Title(title) => title.inherit(from),
                HeaderItem::Actions(actions) => actions.inherit(from),
                HeaderItem::Other(slot) => slot.inherit(from),
            }
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
        let titled = self
            .items
            .iter()
            .any(|item| !matches!(item, HeaderItem::Actions(_)));
        let mut header = row![].width(Fill).spacing(8).align_y(Vertical::Center);
        for item in self.items {
            match item {
                HeaderItem::Title(mut title) => {
                    title.inherited = text.or(title.inherited);
                    let spec = spec.clone();
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
                HeaderItem::Actions(mut actions) => {
                    if !titled {
                        header = header.push(Space::new().width(Fill).boxed().into());
                    }
                    actions.inherit(&Inherited::default().with_code_actions(Placement::Header));
                    header = header.push(
                        frame::Frame::actions(Slot::from(actions).compose())
                            .boxed()
                            .into(),
                    );
                }
                HeaderItem::Other(mut slot) => {
                    slot.inherit(&text);
                    header = header.push(slot);
                }
            }
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
    items: Vec<TitleItem<'a, Message>>,
    inherited: Inherited,
}

/// A title holding one child: a label, or an icon to follow with [`Title::push`].
pub fn title<'a, Message>(item: impl Into<TitleItem<'a, Message>>) -> Title<'a, Message> {
    Title {
        items: vec![item.into()],
        inherited: Inherited::default(),
    }
}

impl<'a, Message: 'a> Title<'a, Message> {
    /// Add a child after the others, 6px apart.
    pub fn push(mut self, item: impl Into<TitleItem<'a, Message>>) -> Self {
        self.items.push(item.into());
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
        let Self { items, inherited } = *self;
        let mut slots = items.into_iter().map(|item| {
            let mut slot = match item {
                TitleItem::Label(value) => crate::text(value)
                    .width(Fill)
                    .wrapping(Wrapping::None)
                    .ellipsis(Ellipsis::End)
                    .into(),
                TitleItem::Slot(slot) => slot,
            };
            slot.inherit(&inherited);
            slot
        });
        if slots.len() == 1 {
            return slots.next().expect("title child").compose();
        }
        let mut title = row![].width(Fill).spacing(6).align_y(Vertical::Center);
        for slot in slots {
            title = title.push(slot);
        }
        title.into()
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
