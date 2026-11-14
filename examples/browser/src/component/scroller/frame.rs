use super::demo::{Demo, Event, Model};
use super::fixtures::Role;
use super::skin;
use crate::icon;
use iced::Alignment::Center;
use iced::alignment::Vertical::Bottom;
use iced::widget::button::StyleFn;
use iced::widget::text::{Ellipsis, LineHeight, Wrapping};
use iced::widget::{stack, svg};
use iced::{Element, Fill, Widget};
use icedxdy::theme::container::Style;
use icedxdy::widget::scroller::Target;
use icedxdy::widget::{
    button, column, container, hover_card, motion, row, scroller, separator, space, status_marker,
    text, tooltip,
};
use icedxdy::{Size, Slot, Theme};
use std::time::Duration;

impl Demo {
    pub(super) fn frame(self, state: &Model) -> Element<'static, Event, Theme> {
        let (title, description) = self.info();
        let mut header = row![
            column![
                text(title)
                    .size(16)
                    .medium()
                    .line_height(LineHeight::Absolute(24.into()))
                    .letter_spacing(-0.025),
                text::muted(description)
                    .width(Fill)
                    .line_height(LineHeight::Absolute(20.into())),
            ]
            .spacing(if matches!(self, Self::Animation | Self::Anchoring) {
                6
            } else {
                4
            })
            .width(Fill)
        ]
        .spacing(if matches!(self, Self::Animation | Self::Anchoring) {
            6
        } else {
            4
        })
        .align_y(iced::Top)
        .width(Fill)
        .padding(20);
        if self == Self::Commands {
            header = header.push(self.controls(state));
        } else if !matches!(self, Self::Opening | Self::Visibility | Self::Scrollable) {
            let reset = button(icon::rotate_cw().size(16))
                .height(32)
                .square()
                .rounded()
                .style(skin::outlined())
                .disabled(match self {
                    Self::Chat | Self::Context => state.busy(),
                    Self::History => state.first == self.initial().first,
                    Self::Group => state.count == self.initial().count,
                    _ => state.count == 0 || state.busy(),
                })
                .on_press(Event::Reset);
            let action = if matches!(self, Self::Anchoring | Self::Animation) {
                reset.into()
            } else {
                tooltip(reset, "Reset").into()
            };
            header = header.push(action);
        }
        let panel: Slot<'static, Event> = if state.count == 0 {
            let (title, desc) = match self {
                Self::Chat => (
                    "Morning, shadcn!",
                    "What are we working on today? Press send to start a new conversation",
                ),
                Self::Anchoring => (
                    "No anchored messages yet",
                    "Send the first message to see the selected role anchor.",
                ),
                Self::Streaming => (
                    "Ready to Stream",
                    "Press send to stream a scripted launch summary.",
                ),
                _ => (
                    "No Messages Yet",
                    "Click the button below to send the first message.",
                ),
            };
            let bubble = svg(svg::Handle::from_memory(include_bytes!(
                "../../../assets/scroller/message-circle-dashed.svg"
            )))
            .width(20)
            .height(20)
            .style(|theme: &Theme, _| svg::Style {
                color: Some(theme.neutral.foreground.into()),
            });
            let tile = container(bubble)
                .center_x(40)
                .center_y(40)
                .style(skin::tile);
            let copy = column![
                text(title)
                    .size(18)
                    .medium()
                    .letter_spacing(-0.025)
                    .line_height(LineHeight::Absolute(28.into())),
                text::muted(desc)
                    .width(Fill)
                    .align_x(Center)
                    .wrapping(Wrapping::Balanced)
                    .line_height(LineHeight::Absolute(22.75.into())),
            ]
            .spacing(8)
            .width(Fill)
            .align_x(Center);
            container(column![tile, copy].spacing(16).width(Fill).align_x(Center))
                .center_x(Fill)
                .center_y(Fill)
                .padding(48)
                .into()
        } else {
            let items = (state.first..state.count).map(|i| {
                let turn = self.turns()[i];
                let value = state.text(self, i);
                let animated = matches!(
                    self,
                    Self::Chat
                        | Self::Anchoring
                        | Self::Context
                        | Self::Streaming
                        | Self::Animation
                        | Self::Scrollable
                );
                let body = if animated && turn.role == Role::User {
                    let preset = if self == Self::Animation {
                        state.preset
                    } else {
                        motion::Preset::SlideUp
                    };
                    turn.motion(
                        value,
                        preset,
                        self == Self::Scrollable || state.newest == Some(i),
                    )
                } else {
                    turn.view(value)
                };
                scroller::item(i as u64, body).anchor(self.anchor(state, i))
            });
            let history_end = (self == Self::History).then(|| {
                scroller::item(
                    u64::MAX,
                    status_marker("End of Conversation").style(status_marker::Separator),
                )
            });
            let transcript = scroller(items.chain(history_end))
                .height(Fill)
                .padding(20.0)
                .spacing(if self == Self::Scrollable { 16.0 } else { 32.0 })
                .auto_scroll(self == Self::Streaming)
                .scroll_margin(match self {
                    Self::Context => 24.0,
                    Self::Opening => 64.0,
                    Self::Visibility => 12.0,
                    _ => 0.0,
                })
                .previous_item_peek(f32::from(state.peek))
                .default_scroll_position(state.opening)
                .smooth_commands(state.smooth)
                .command(state.command)
                .on_reader_change(Event::Reader);
            let mut layers = vec![Element::from(transcript)];
            if state.reader.can_scroll_down {
                layers.push(
                    container(
                        button(icon::arrow_down().size(16))
                            .style(skin::outlined())
                            .rounded()
                            .height(28)
                            .square()
                            .on_press(Event::Jump(Target::End)),
                    )
                    .center_x(Fill)
                    .align_y(Bottom)
                    .height(Fill)
                    .padding(16)
                    .into(),
                );
            }
            stack(layers).width(Fill).height(Fill).boxed().into()
        };
        let mut body = column![header, separator(), panel]
            .spacing(0)
            .width(Fill)
            .height(Fill);
        if !matches!(self, Self::Commands | Self::Visibility) {
            let divided = !matches!(
                self,
                Self::Chat | Self::Context | Self::Streaming | Self::Anchoring
            );
            if divided {
                body = body.push(separator().into());
            }
            let top = if divided { 20 } else { 0 };
            body = body.push(
                container(self.controls(state))
                    .padding([top, 20, 0, 20])
                    .width(Fill)
                    .into(),
            );
        }
        let card = container(body)
            .width(Fill)
            .height(560)
            .padding([0, 0, 20, 0])
            .style(skin::card);
        let card: Slot<'static, Event> = if self == Self::Visibility {
            let outline = column(
                self.turns()
                    .iter()
                    .enumerate()
                    .filter(|(_, t)| t.role == Role::User)
                    .map(|(i, t)| {
                        let style: StyleFn<'static, Theme> =
                            if state.reader.current_anchor == Some(i as u64) {
                                (button::Secondary + icedxdy::radius(14)).into()
                            } else {
                                (button::Ghost + icedxdy::radius(14)).into()
                            };
                        button(
                            text(t.short())
                                .size(14)
                                .width(Fill)
                                .height(20)
                                .line_height(LineHeight::Absolute(20.into()))
                                .ellipsis(Ellipsis::End),
                        )
                        .size(Size::Sm)
                        .width(Fill)
                        .style(style)
                        .on_press(Event::Jump(Target::Item(i as u64)))
                        .into()
                    }),
            )
            .spacing(4)
            .width(Fill);
            let ticks = column(
                self.turns()
                    .iter()
                    .enumerate()
                    .filter(|(_, t)| t.role == Role::User)
                    .map(|(i, _)| {
                        let current = state.reader.current_anchor == Some(i as u64);
                        container(space::horizontal())
                            .width(16)
                            .height(2)
                            .style(move |theme| Style {
                                background: Some(
                                    if current {
                                        theme.neutral.foreground
                                    } else {
                                        theme.muted.foreground.alpha(0.4)
                                    }
                                    .into(),
                                ),
                                ..Style::default()
                            })
                            .into()
                    }),
            )
            .spacing(4);
            let trigger = button(ticks).height(36).square().style(button::Ghost);
            let nav = hover_card(trigger, outline)
                .side(hover_card::Side::Left)
                .side_offset(-28.0)
                .padding(4)
                .style(skin::outline)
                .open_delay(Duration::ZERO)
                .close_delay(Duration::ZERO)
                .width(256);
            let outlined = container(row![card, nav].width(Fill).spacing(12).align_y(Center))
                .width(Fill)
                .max_width(432);
            container(outlined).width(Fill).align_x(iced::Right).into()
        } else {
            card.into()
        };
        let caption_width = if self == Self::Anchoring {
            320.0
        } else {
            384.0
        };
        let caption = text::muted(self.caption())
            .size(12)
            .line_height(LineHeight::Absolute(16.into()))
            .width(Fill)
            .align_x(Center);
        let caption = if matches!(
            self,
            Self::Animation | Self::Commands | Self::Group | Self::History
        ) {
            caption.wrapping(Wrapping::Balanced)
        } else {
            caption
        };
        let caption = container(caption)
            .width(Fill)
            .max_width(caption_width)
            .padding([0, 2]);
        column![card, caption]
            .spacing(16)
            .align_x(Center)
            .width(Fill)
            .max_width(if self == Self::Visibility {
                480.0
            } else {
                384.0
            })
            .into()
    }
}
