//! File cards with independent preview and action targets.
mod group;
pub use group::{Group, group};
mod surface;
mod trigger;
use crate::status_shimmer;
use crate::{Compose, Inherited, Row, Size, Slot, button, column, container, row, text};
use iced_core::Length::{Fill, Shrink};
use iced_core::alignment::{Horizontal, Vertical};
use iced_core::font::Weight;
use iced_core::text::{Ellipsis, LineHeight, Wrapping};
use iced_core::{Element, Length};
use iced_widget::text::StyleFn;
use iced_widget::{Renderer, Widget, stack};
use icedcn_core::theme::container::Style;
use icedcn_core::{Border, Theme, radius};
pub use trigger::{Trigger, trigger};

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
    inherited: Inherited,
    style: Option<Box<dyn Fn(&Theme) -> Style + 'a>>,
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
        inherited: Inherited::default(),
        style: None,
    }
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
    /// Supply the full-card target; actions receive input independently above it.
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
    /// Customize the card surface using the active theme.
    pub fn style(mut self, style: impl Fn(&Theme) -> Style + 'a) -> Self {
        self.style = Some(Box::new(style));
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
    let mut label = text(label)
        .size(pixels)
        .line_height(LineHeight::Absolute((pixels * 1.25).into()))
        .height(pixels * 1.25)
        .width(Fill)
        .wrapping(Wrapping::None)
        .ellipsis(Ellipsis::End);
    if medium {
        label = label.medium();
    }
    label.into()
}
impl<'a, Message: 'a> Compose<Message> for Attachment<'a, Message> {
    fn inherit(&mut self, from: &Inherited) {
        self.inherited = self.inherited.clone().or(from.clone());
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
            inherited,
            style,
        } = *self;
        let (pad, gap, side, px) = match size {
            Size::Xs => (4., 6., 28., 12.),
            Size::Sm => (6., 10., 32., 12.),
            _ => (8., 8., 40., 14.),
        };
        let inherited = if trigger.is_some() {
            inherited.with_selectable(false)
        } else {
            inherited
        };
        let has_content = !title.is_empty() || description.is_some();
        let mut title = if matches!(state, State::Uploading | State::Processing) {
            status_shimmer::title(
                iced_widget::text(title)
                    .size(px)
                    .weight(Weight::Medium)
                    .line_height(LineHeight::Absolute((px * 1.25).into()))
                    .height(px * 1.25)
                    .width(Fill)
                    .wrapping(Wrapping::None)
                    .ellipsis(Ellipsis::End)
                    .selectable(inherited.selectable().unwrap_or(true)),
            )
        } else {
            truncate(title, px, true)
        };
        title.inherit(&inherited);
        let mut labels = column![title].spacing(2).width(Fill);
        if let Some(description) = description {
            let mut description: Slot<'a, Message> = text(description)
                .size(12.)
                .width(Fill)
                .line_height(LineHeight::Absolute(16.into()))
                .wrapping(Wrapping::None)
                .ellipsis(Ellipsis::End)
                .style(Box::new(move |theme: &Theme| iced_widget::text::Style {
                    color: Some(
                        if state == State::Error {
                            theme.destructive.base.alpha(0.8)
                        } else {
                            theme.muted.foreground
                        }
                        .into(),
                    ),
                    ..Default::default()
                }) as StyleFn<Theme>)
                .into();
            description.inherit(&inherited);
            labels = labels.push(description);
        }
        let width = width.unwrap_or(if vertical { Length::Fixed(120.) } else { Fill });
        let media_side = match (vertical, width) {
            (true, Length::Fixed(width)) => (width - 2. * (pad + 1.)).max(0.),
            (true, _) => 102.,
            _ => side,
        };
        let media = media.map(|mut item| {
            item.inherit(&inherited.clone().with_size(if vertical {
                24.
            } else if size == Size::Xs {
                14.
            } else {
                16.
            }));
            container(item)
                .center(media_side)
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
                        radius: if size == Size::Xs {
                            radius::MD
                        } else {
                            radius::LG
                        }
                        .into(),
                        ..Border::default()
                    },
                    ..Style::default()
                })
        });
        let mut body: Slot<'a, Message> = if vertical {
            let mut col = column(Vec::new()).spacing(gap);
            if let Some(media) = media {
                col = col.push(media.into());
            }
            if has_content {
                col = col.push(
                    container(labels)
                        .padding([0, 4])
                        .clip(true)
                        .width(Fill)
                        .into(),
                );
            }
            col.width(Fill).into()
        } else {
            let mut line = row().spacing(gap).align_y(Vertical::Center);
            if let Some(media) = media {
                line = line.push(media.into());
            }
            line.push(container(labels).clip(true).width(Fill).into())
                .width(Fill)
                .into()
        };
        if !actions.is_empty() {
            let actions = Row::with_children(actions).spacing(if vertical { 4 } else { 0 });
            body = if vertical {
                let base = body.compose();
                let overlay: Element<'a, Message, Theme, Renderer> = container(actions)
                    .width(Fill)
                    .height(Fill)
                    .align_x(Horizontal::Right)
                    .align_y(Vertical::Top)
                    .padding(3)
                    .into();
                stack![base, overlay].boxed().into()
            } else {
                row![container(body).width(Fill), actions]
                    .width(Fill)
                    .spacing(gap)
                    .align_y(Vertical::Center)
                    .into()
            };
        }
        body.inherit(&inherited);
        let content = container(body)
            .padding(pad + 1.)
            .width(width)
            .height(Shrink)
            .clip(true)
            .into();
        let trigger = trigger.map(|wrap| {
            let mut target = wrap(
                iced_widget::Space::new()
                    .width(Fill)
                    .height(Fill)
                    .boxed()
                    .into(),
            );
            target.inherit(&inherited);
            target.compose()
        });
        surface::Surface {
            content,
            trigger,
            idle: state == State::Idle,
            style: style.unwrap_or_else(|| {
                Box::new(move |theme| Style {
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
            }),
        }
        .boxed()
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
