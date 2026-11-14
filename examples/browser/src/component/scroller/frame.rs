use super::demo::{Demo, Event, Model};
use super::fixtures::Role;
use super::tick;
use crate::icon;
use iced::Alignment::Center;
use iced::alignment::Vertical::Bottom;
use iced::widget::button::StyleFn;
use iced::widget::stack;
use iced::{Element, Fill, Widget};
use icedxdy::theme::container::Style;
use icedxdy::widget::scroller::{Position, Target};
use icedxdy::widget::{
    button, column, container, hover_card, motion, row, scroller, separator, space, status_marker,
    text,
};
use icedxdy::{Size, Slot, Theme, card};
use std::time::Duration;

impl Demo {
    pub(super) fn frame(self, state: &Model) -> Element<'static, Event, Theme> {
        let (title, description) = self.info();
        let mut header = card::header(title).description(description);
        if !matches!(
            self,
            Self::Opening | Self::Commands | Self::Visibility | Self::Scrollable
        ) {
            let initial = self.initial();
            header = header.action(
                button(icon::refresh_ccw())
                    .size(Size::Sm)
                    .square()
                    .style(button::Outline)
                    .disabled(
                        (state.count == initial.count && state.first == initial.first)
                            || (self == Self::Animation && state.stream.is_some()),
                    )
                    .on_press(Event::Reset),
            );
        }
        if self == Self::Commands {
            header = header.action(self.controls(state));
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
            container(
                column![
                    icon::message_square().size(26),
                    text::large(title).semibold(),
                    text::muted(desc).width(Fill).align_x(Center)
                ]
                .spacing(8)
                .width(Fill)
                .align_x(Center),
            )
            .center_x(Fill)
            .center_y(Fill)
            .padding(12)
            .into()
        } else {
            let items = (state.first..state.count).map(|i| {
                let turn = self.turns()[i];
                let value = if i + 1 == state.count {
                    state.stream.map_or(turn.text, |n| &turn.text[..n])
                } else {
                    turn.text
                };
                let body = turn.view(value);
                let body = if self == Self::Animation && turn.role == Role::User {
                    motion(body)
                        .preset(state.preset)
                        .animate(state.newest == Some(i))
                        .into()
                } else {
                    body
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
                .auto_scroll(self == Self::Streaming)
                .scroll_margin(match self {
                    Self::Context => 24.0,
                    Self::Opening => 64.0,
                    _ => 12.0,
                })
                .previous_item_peek(f32::from(state.peek))
                .default_scroll_position(if self == Self::Commands {
                    Position::End
                } else if self == Self::Scrollable {
                    Position::Start
                } else {
                    state.opening
                })
                .smooth_commands(matches!(self, Self::Commands | Self::Visibility))
                .command(state.command)
                .on_reader_change(Event::Reader);
            let mut layers = vec![Element::from(transcript)];
            if state.reader.can_scroll_down {
                layers.push(
                    container(
                        button(icon::arrow_down())
                            .style(button::Outline)
                            .rounded()
                            .size(Size::Sm)
                            .square()
                            .on_press(Event::Jump(Target::End)),
                    )
                    .center_x(Fill)
                    .align_y(Bottom)
                    .height(Fill)
                    .padding(8)
                    .into(),
                );
            }
            stack(layers).width(Fill).height(Fill).boxed().into()
        };
        let mut body = column![
            container(header).width(Fill).padding([12, 0]),
            separator(),
            panel,
        ]
        .spacing(0)
        .width(Fill)
        .height(560);
        if !matches!(self, Self::Commands | Self::Visibility) {
            let controls = self.controls(state);
            let footer = if matches!(self, Self::Chat | Self::Context | Self::Streaming) {
                card::footer_with_spacing(controls, 0.0)
            } else {
                card::footer(controls)
            };
            body = body.push(footer);
        }
        // Keep the transcript at the same tree path when the last chunk lands.
        let mut layers = vec![body.into()];
        if state.stream.is_some() {
            layers.push(tick::Tick.boxed());
        }
        let body = stack(layers).width(Fill).height(560).boxed();
        let card = card(body).width(Fill).boxed();
        let card: Slot<'static, Event> = if self == Self::Visibility {
            let outline = column(
                self.turns()
                    .iter()
                    .enumerate()
                    .filter(|(_, t)| t.role == Role::User)
                    .map(|(i, t)| {
                        let style: StyleFn<'static, Theme> =
                            if state.reader.current_anchor == Some(i as u64) {
                                button::Secondary.into()
                            } else {
                                button::Ghost.into()
                            };
                        button(text::small(t.short()))
                            .size(Size::Sm)
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
            let trigger = button(ticks).square().style(button::Ghost);
            let nav = hover_card(trigger, outline)
                .side(hover_card::Side::Left)
                .open_delay(Duration::ZERO)
                .close_delay(Duration::ZERO)
                .width(256);
            row![card, nav]
                .width(Fill)
                .spacing(4)
                .align_y(Center)
                .into()
        } else {
            card.into()
        };
        column![
            card,
            text::muted(self.caption())
                .size(12)
                .width(Fill)
                .align_x(Center)
        ]
        .spacing(16)
        .width(Fill)
        .max_width(if self == Self::Visibility {
            420.0
        } else {
            384.0
        })
        .into()
    }
}
