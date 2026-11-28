use crate::button::widget::{Button, Style, StyleFn};
use crate::{Compose, Inherited, View};
use iced_core::Length::Fill;
use iced_core::{Element, mouse};
use iced_widget::{Renderer, Widget};
use icedcn_core::Theme;

/// A transparent, keyboard-accessible target for an entire attachment.
pub struct Trigger<'a, Message> {
    content: View<'a, Message>,
    press: Option<Message>,
    disabled: bool,
}

pub fn trigger<'a, Message>(content: impl Into<View<'a, Message>>) -> Trigger<'a, Message> {
    Trigger {
        content: content.into(),
        press: None,
        disabled: false,
    }
}
impl<Message> Trigger<'_, Message> {
    pub fn on_press(mut self, message: Message) -> Self {
        self.press = Some(message);
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}
impl<'a, Message: Clone + 'a> Compose<Message> for Trigger<'a, Message> {
    fn inherit(&mut self, inherited: &Inherited) {
        self.disabled |= inherited.is_disabled();
        self.content.inherit(&(*inherited).with_selectable(false));
    }
    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        Button::new(self.content.compose())
            .width(Fill)
            .height(Fill)
            .padding(0)
            .translate(false)
            .interaction(mouse::Interaction::Pointer)
            .disabled(self.disabled)
            .on_press_maybe(self.press)
            .style(Box::new(|_: &Theme, _| Style::default()) as StyleFn<Theme>)
            .boxed()
    }
}
impl<'a, Message: Clone + 'a> From<Trigger<'a, Message>> for View<'a, Message> {
    fn from(trigger: Trigger<'a, Message>) -> Self {
        View::new(trigger)
    }
}
