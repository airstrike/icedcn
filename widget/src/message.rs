//! A conversation turn with an avatar, content and lightweight metadata.
use crate::{Column, Compose, Inherited, Size, Slot, Weight, column, container, row};
use iced_core::Element;
use iced_core::Length::Fill;
use iced_core::alignment::{Horizontal, Vertical};
use iced_widget::Renderer;
use icedxdy_core::Theme;
use icedxdy_core::theme::{
    container::Transparent,
    tweak::{Fg, Token},
};

pub struct Message<'a, Event> {
    content: Slot<'a, Event>,
    avatar: Option<Slot<'a, Event>>,
    header: Option<Slot<'a, Event>>,
    footer: Option<Slot<'a, Event>>,
    align: Horizontal,
}
pub fn message<'a, Event: 'a>(content: impl Into<Slot<'a, Event>>) -> Message<'a, Event> {
    Message {
        content: content.into(),
        avatar: None,
        header: None,
        footer: None,
        align: Horizontal::Left,
    }
}
pub fn group<'a, Event: 'a>(items: impl IntoIterator<Item = Slot<'a, Event>>) -> Column<'a, Event> {
    column(items).spacing(8).width(Fill)
}
impl<'a, Event: 'a> Message<'a, Event> {
    pub fn align(mut self, align: Horizontal) -> Self {
        self.align = align;
        self
    }
    pub fn avatar(mut self, avatar: impl Into<Slot<'a, Event>>) -> Self {
        self.avatar = Some(avatar.into());
        self
    }
    pub fn header(mut self, header: impl Into<Slot<'a, Event>>) -> Self {
        self.header = Some(header.into());
        self
    }
    pub fn footer(mut self, footer: impl Into<Slot<'a, Event>>) -> Self {
        self.footer = Some(footer.into());
        self
    }
}
fn metadata<'a, Event: 'a>(mut content: Slot<'a, Event>, align: Horizontal) -> Slot<'a, Event> {
    content.inherit(
        &Inherited::default()
            .with_size(Size::Xs)
            .with_weight(Weight::Medium),
    );
    container(content)
        .padding([0, 12])
        .width(Fill)
        .align_x(align)
        .class(Transparent + Fg(Token::MutedForeground))
        .into()
}
impl<'a, Event: 'a> Compose<Event> for Message<'a, Event> {
    fn inherit(&mut self, from: &Inherited) {
        self.content.inherit(from);
        for slot in [&mut self.avatar, &mut self.header, &mut self.footer]
            .into_iter()
            .flatten()
        {
            slot.inherit(from);
        }
    }
    fn compose<'b>(mut self: Box<Self>) -> Element<'b, Event, Theme, Renderer>
    where
        Self: 'b,
    {
        self.content.inherit(
            &Inherited::default()
                .with_size(Size::Sm)
                .with_conversation_alignment(self.align),
        );
        let footer_present = self.footer.is_some();
        let mut body = column(Vec::new())
            .width(Fill)
            .align_x(self.align)
            .spacing(10);
        if let Some(header) = self.header {
            body = body.push(metadata(header, self.align));
        }
        body = body.push(self.content);
        if let Some(footer) = self.footer {
            body = body.push(metadata(footer, self.align));
        }
        let Some(avatar) = self.avatar else {
            return body.into();
        };
        let avatar =
            container(avatar)
                .center_x(32)
                .padding([0, 0, if footer_present { 32 } else { 0 }, 0]);
        let line = if self.align == Horizontal::Right {
            row![body, avatar]
        } else {
            row![avatar, body]
        };
        line.width(Fill).align_y(Vertical::Bottom).spacing(8).into()
    }
}
impl<'a, Event: 'a> From<Message<'a, Event>> for Slot<'a, Event> {
    fn from(message: Message<'a, Event>) -> Self {
        Slot::new(message)
    }
}
impl<'a, Event: 'a> From<Message<'a, Event>> for Element<'a, Event, Theme, Renderer> {
    fn from(message: Message<'a, Event>) -> Self {
        Box::new(message).compose()
    }
}
