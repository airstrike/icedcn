use super::demo::{Demo, Event, Model};
use super::fixtures::Role;
use crate::icon;
use iced::Alignment::Center;
use iced::Fill;
use icedxdy::widget::scroller::{Position, Target};
use icedxdy::widget::{
    button, column, container, dropdown_menu, menu, motion, row, slider, space, tabs, text,
    toggle_group,
};
use icedxdy::{Size, Slot};

impl Demo {
    pub(super) fn controls(self, state: &Model) -> Slot<'static, Event> {
        let next = self.turns().get(state.count);
        let disabled = next.is_none() || state.stream.is_some();
        let send = button(icon::arrow_up())
            .size(Size::Sm)
            .square()
            .disabled(disabled)
            .on_press(Event::Send);
        match self {
            Self::Chat | Self::Context | Self::Streaming => {
                let plus = dropdown_menu(
                    |m| {
                        m.trigger(
                            button(icon::plus())
                                .size(Size::Sm)
                                .square()
                                .style(button::Outline),
                        )
                    },
                    |_| {
                        column![
                            menu::item(row![icon::file_plus(), "Add Photos & Files"].spacing(8)),
                            menu::separator(),
                            menu::item(row![icon::image(), "Create Image"].spacing(8)),
                            menu::item(row![icon::search(), "Deep Research"].spacing(8)),
                            menu::item(row![icon::globe(), "Web Search"].spacing(8))
                        ]
                        .width(Fill)
                        .into()
                    },
                )
                .width(176)
                .side(dropdown_menu::Side::Top);
                let prompt = next.map_or("No messages queued. Reset the conversation.", |t| t.text);
                let mut line = row![plus].spacing(8).align_y(Center).width(Fill);
                if self == Self::Context {
                    line = line
                        .push(text::xs(format!("{}px", state.peek)).into())
                        .push(
                            container(
                                slider(64.0..=128.0)
                                    .value(f32::from(state.peek))
                                    .disabled(state.stream.is_some())
                                    .on_value_change(Event::Peek),
                            )
                            .width(80)
                            .into(),
                        );
                }
                let line = line.push(space::horizontal().into()).push(send.into());
                let prompt = container(text::small(prompt).width(Fill))
                    .width(Fill)
                    .height(48)
                    .clip(true);
                column![prompt, line]
                    .width(Fill)
                    .spacing(4)
                    .padding([8, 12])
                    .into()
            }
            Self::Anchoring => {
                let value = if state.assistant_anchor {
                    "Assistant"
                } else {
                    "User"
                };
                row![
                    toggle_group(|| ["User", "Assistant"].map(|s| toggle_group::item(s, s)))
                        .value([value])
                        .on_value_change(move |v| Event::Anchor(
                            v.first().cloned().unwrap_or_else(|| value.to_owned())
                        )),
                    space::horizontal(),
                    send
                ]
                .width(Fill)
                .into()
            }
            Self::Animation => row![presets(state.preset), space::horizontal(), send]
                .width(Fill)
                .into(),
            Self::Opening => {
                let current = match state.opening {
                    Position::Start => "start",
                    Position::End => "end",
                    Position::LastAnchor => "last-anchor",
                };
                container(
                    tabs(|t| t.list(["start", "end", "last-anchor"].map(|s| t.trigger(s, s))))
                        .value(current)
                        .on_value_change(Event::Open),
                )
                .center_x(Fill)
                .into()
            }
            Self::Group => column![
                button(if state.count < 4 {
                    "Add Rocky"
                } else {
                    "Send Message as Rocky"
                })
                .style(button::Secondary)
                .width(Fill)
                .disabled(disabled)
                .on_press(Event::Send),
                text::muted(if state.count < 4 {
                    "This will create a marker and make it the anchor"
                } else {
                    "Now send Rocky's reply into the conversation"
                })
                .size(12)
            ]
            .width(Fill)
            .spacing(8)
            .align_x(Center)
            .into(),
            Self::History => column![
                container(
                    button(if state.first == 0 {
                        "History Loaded"
                    } else {
                        "Load History"
                    })
                    .style(button::Secondary)
                    .width(Fill)
                    .disabled(state.first == 0)
                    .on_press(Event::Send)
                )
                .id("transcript-load-history")
                .width(Fill),
                text::xs("Restore earlier messages while keeping your place.")
            ]
            .width(Fill)
            .align_x(Center)
            .spacing(6)
            .into(),
            Self::Commands => dropdown_menu(
                |m| m.trigger(button("Jump to...").style(button::Secondary)),
                move |_| {
                    column(
                        std::iter::once(menu::label("Conversations")).chain(
                            self.turns()
                                .iter()
                                .enumerate()
                                .filter(|(_, t)| t.role == Role::User)
                                .map(|(i, t)| {
                                    menu::item(text::small(t.short()))
                                        .on_select(Event::Jump(Target::Item(i as u64)))
                                        .into()
                                }),
                        ),
                    )
                    .width(Fill)
                    .into()
                },
            )
            .width(256)
            .into(),
            Self::Visibility => text::muted("Track the current anchored turn.")
                .size(12.0)
                .into(),
            Self::Scrollable => text::muted(
                match (state.reader.can_scroll_up, state.reader.can_scroll_down) {
                    (true, true) => "You can scroll both ways.",
                    (false, true) => "You are at the top. You can only scroll down.",
                    (true, false) => "You are at the bottom. You can only scroll up.",
                    (false, false) => "All messages fit in the viewport.",
                },
            )
            .size(14.0)
            .width(Fill)
            .align_x(Center)
            .into(),
        }
    }
}
fn presets(selected: motion::Preset) -> Slot<'static, Event> {
    dropdown_menu(
        move |menu| {
            menu.trigger(
                button![
                    text::small(selected.to_string()),
                    icon::chevron_down().size(14)
                ]
                .style(button::Outline),
            )
        },
        move |_| {
            menu::group(motion::Preset::ALL.map(|preset| {
                menu::radio_item(preset.to_string(), preset, Some(selected), Event::Preset).into()
            }))
        },
    )
    .align(dropdown_menu::Align::Start)
    .side(dropdown_menu::Side::Top)
    .width(152)
    .into()
}
