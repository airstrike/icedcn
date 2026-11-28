//! A muted text surface that participates in button-group joining.
use crate::compose::{Compose, Inherited, View};
use crate::input_group::widget::Surface;
use crate::{Size, Weight, container};
use iced_core::{Element, Padding};
use iced_widget::{Renderer, Widget};
use icedcn_core::theme::text_input;
use icedcn_core::{Theme, radius};

/// A bordered, muted label. Content accepts the same slots as a button.
pub struct Text<'a, Message> {
    content: View<'a, Message>,
    inherited: Inherited,
}
impl<'a, Message: 'a> Text<'a, Message> {
    pub(super) fn new(content: impl Into<View<'a, Message>>) -> Self {
        Self {
            content: content.into(),
            inherited: Inherited::default(),
        }
    }
}
impl<'a, Message: 'a> Compose<Message> for Text<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.inherited = self.inherited.or(*from);
    }
    fn compose<'b>(mut self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let inherited = self.inherited.without_join();
        self.content.inherit(
            &inherited.or(Inherited::default()
                .with_size(Size::Default)
                .with_weight(Weight::Medium)),
        );
        let padding = Padding::from([1., 11.]);
        let padding = self
            .inherited
            .join()
            .map_or(padding, |join| join.padding(padding, 1.));
        let content = container(self.content)
            .padding(padding)
            .center_y(inherited.size().unwrap_or_default().height())
            .into();
        Surface::new(
            content,
            Box::new(|theme, status| {
                let mut style = text_input::default(theme, status);
                style.background = theme.muted.base.into();
                style.border.color = theme.neutral.border;
                style.border.radius = radius::LG.into();
                style.ring = None;
                style
            }),
        )
        .join(self.inherited.join())
        .disabled(inherited.is_disabled())
        .boxed()
    }
}
impl<'a, Message: 'a> From<Text<'a, Message>> for View<'a, Message> {
    fn from(value: Text<'a, Message>) -> Self {
        View::new(value)
    }
}
impl<'a, Message: 'a> From<Text<'a, Message>> for Element<'a, Message, Theme, Renderer> {
    fn from(value: Text<'a, Message>) -> Self {
        Box::new(value).compose()
    }
}
