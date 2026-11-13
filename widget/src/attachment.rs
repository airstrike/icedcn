//! File cards with independent preview and action targets.
use crate::scrollable::Direction;
use crate::status_marker::shimmer;
use crate::{Compose, Inherited, Row, Size, Slot, button, column, container, row, scroll_area};
use iced_core::Length::{Fill, Shrink};
use iced_core::alignment::{Horizontal, Vertical};
use iced_core::text::{Ellipsis, Wrapping};
use iced_core::{Element, Length};
use iced_widget::{Renderer, Widget, stack};
use icedxdy_core::theme::container::Style;
use icedxdy_core::{Border, Theme, radius};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Idle,
    Uploading,
    Processing,
    Error,
    #[default]
    Done,
}

pub struct Attachment<'a, Message> {
    title: String,
    description: Option<String>,
    media: Option<Slot<'a, Message>>,
    actions: Vec<Slot<'a, Message>>,
    trigger: Option<Box<dyn FnOnce(Slot<'a, Message>) -> Slot<'a, Message> + 'a>>,
    state: State,
    size: Size,
    vertical: bool,
    width: Option<Length>,
}
pub fn attachment<'a, Message>(title: impl Into<String>) -> Attachment<'a, Message> {
    Attachment {
        title: title.into(),
        description: None,
        media: None,
        actions: Vec::new(),
        trigger: None,
        state: State::Done,
        size: Size::Default,
        vertical: false,
        width: None,
    }
}

/// A horizontally scrollable strip that keeps the cards at their intrinsic widths.
pub fn group<'a, Message: 'a>(
    cards: impl IntoIterator<Item = Slot<'a, Message>>,
) -> Slot<'a, Message> {
    scroll_area(container(Row::with_children(cards).spacing(12)).padding([4, 0]))
        .direction(Direction::Horizontal(scroll_area::bar()))
        .width(Fill)
        .into()
}

impl<'a, Message: 'a> Attachment<'a, Message> {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }
    pub fn media(mut self, value: impl Into<Slot<'a, Message>>) -> Self {
        self.media = Some(value.into());
        self
    }
    pub fn action(mut self, value: impl Into<Slot<'a, Message>>) -> Self {
        self.actions.push(value.into());
        self
    }
    /// The trigger wraps the content beneath the independent action buttons.
    pub fn trigger(
        mut self,
        wrap: impl FnOnce(Slot<'a, Message>) -> Slot<'a, Message> + 'a,
    ) -> Self {
        self.trigger = Some(Box::new(wrap));
        self
    }
    pub fn state(mut self, state: State) -> Self {
        self.state = state;
        self
    }
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }
    pub fn vertical(mut self, vertical: bool) -> Self {
        self.vertical = vertical;
        self
    }
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }
}

/// An independent icon button in a file card.
pub fn action<'a, Message: Clone + 'a>(
    icon: impl Into<Slot<'a, Message>>,
) -> crate::Button<'a, Message> {
    button(icon).square().size(Size::Xs).style(button::Ghost)
}
fn truncate<'a, Message: 'a>(label: String, pixels: f32, medium: bool) -> Slot<'a, Message> {
    use iced_core::font::Weight;
    let mut label = iced_widget::text(label)
        .size(pixels)
        .wrapping(Wrapping::None)
        .ellipsis(Ellipsis::End);
    if medium {
        label = label.weight(Weight::Medium);
    }
    label.boxed().into()
}
impl<'a, Message: 'a> Compose<Message> for Attachment<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        if let Some(media) = &mut self.media {
            media.inherit(from);
        }
        for action in &mut self.actions {
            action.inherit(from);
        }
    }
    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let Self {
            title,
            description,
            media,
            actions,
            trigger,
            state,
            size,
            vertical,
            width,
        } = *self;
        let (pad, gap, side, px) = match size {
            Size::Xs => (4., 6., 28., 12.),
            Size::Sm => (6., 10., 32., 12.),
            _ => (8., 8., 40., 14.),
        };
        let title = if matches!(state, State::Uploading | State::Processing) {
            shimmer(title)
        } else {
            truncate(title, px, true)
        };
        let mut labels = column![title].spacing(2);
        if let Some(description) = description {
            labels = labels.push(
                container(truncate(description, 12., false))
                    .style(move |theme| Style {
                        text_color: Some(if state == State::Error {
                            theme.destructive.base.alpha(0.8)
                        } else {
                            theme.muted.foreground
                        }),
                        ..Style::default()
                    })
                    .into(),
            );
        }
        let media = media.map(|item| {
            container(item)
                .center(if vertical { 120. - 2. * pad } else { side })
                .clip(true)
                .style(move |theme| Style {
                    background: Some(
                        if state == State::Error {
                            theme.destructive.base.alpha(0.1)
                        } else {
                            theme.muted.base
                        }
                        .into(),
                    ),
                    text_color: Some(if state == State::Error {
                        theme.destructive.base
                    } else {
                        theme.neutral.foreground
                    }),
                    border: Border {
                        radius: radius::MD.into(),
                        ..Border::default()
                    },
                    ..Style::default()
                })
        });
        let width = width.unwrap_or(if vertical { Length::Fixed(120.) } else { Fill });
        let mut body: Slot<'a, Message> = if vertical {
            let mut col = column(Vec::new()).spacing(gap);
            if let Some(media) = media {
                col = col.push(media.into());
            }
            col.push(
                container(labels)
                    .padding([0, 4])
                    .clip(true)
                    .width(Fill)
                    .into(),
            )
            .width(Fill)
            .into()
        } else {
            let mut line = row().spacing(gap).align_y(Vertical::Center);
            if let Some(media) = media {
                line = line.push(media.into());
            }
            line.push(container(labels).clip(true).width(Fill).into())
                .width(Fill)
                .into()
        };
        if let Some(trigger) = trigger {
            body = trigger(body);
        }
        if !actions.is_empty() {
            let actions = Row::with_children(actions).spacing(if vertical { 4 } else { 0 });
            body = if vertical {
                let base = body.compose();
                let overlay: Element<'a, Message, Theme, Renderer> = container(actions)
                    .width(Fill)
                    .height(Fill)
                    .align_x(Horizontal::Right)
                    .align_y(Vertical::Top)
                    .padding(2)
                    .into();
                stack![base, overlay].boxed().into()
            } else {
                row![container(body).width(Fill), actions]
                    .spacing(gap)
                    .align_y(Vertical::Center)
                    .into()
            };
        }
        container(body)
            .padding(pad)
            .width(width)
            .height(Shrink)
            .clip(true)
            .style(move |theme| Style {
                background: Some(theme.card.base.into()),
                text_color: Some(theme.card.foreground),
                border: Border {
                    radius: if size == Size::Xs {
                        radius::LG
                    } else {
                        radius::XL
                    }
                    .into(),
                    width: 1.,
                    color: if state == State::Error {
                        theme.destructive.base.alpha(0.3)
                    } else {
                        theme.neutral.border
                    },
                },
                ..Style::default()
            })
            .into()
    }
}
impl<'a, Message: 'a> From<Attachment<'a, Message>> for Slot<'a, Message> {
    fn from(a: Attachment<'a, Message>) -> Self {
        Slot::new(a)
    }
}
impl<'a, Message: 'a> From<Attachment<'a, Message>> for Element<'a, Message, Theme, Renderer> {
    fn from(a: Attachment<'a, Message>) -> Self {
        Box::new(a).compose()
    }
}
