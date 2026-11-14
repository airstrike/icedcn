use super::demo::{Demo, Event, Model};
use super::fixtures::Role;
use crate::icon;
use iced::Alignment::Center;
use iced::Fill;
use iced::widget::button::{Status, StyleFn};
use iced::widget::text::LineHeight;
use icedxdy::core::Border;
use icedxdy::theme::button::{outline, secondary};
use icedxdy::theme::container::Style;
use icedxdy::theme::text::{Foreground, Muted};
use icedxdy::theme::tweak::{Alpha, hover};
use icedxdy::widget::button::widget;
use icedxdy::widget::scroller::{Position, Target};
use icedxdy::widget::slider::Catalog;
use icedxdy::widget::text::Ellipsis;
use icedxdy::widget::{
    button, column, container, dropdown_menu, menu, motion, row, select, slider, space, tabs, text,
    toggle_group,
};
use icedxdy::{Padding, Slot, Theme, radius};

impl Demo {
    pub(super) fn controls(self, state: &Model) -> Slot<'static, Event> {
        let disabled = state.next(self).is_none() || state.busy();
        let send = button(icon::arrow_up().size(16))
            .square()
            .rounded()
            .inset(1)
            .style(button::Default + hover(Alpha(0.8)))
            .disabled(disabled)
            .on_press(Event::Send);
        match self {
            Self::Chat | Self::Context | Self::Streaming => self.composer(state, send.into()),
            Self::Anchoring => {
                let value = if state.assistant_anchor {
                    "Assistant"
                } else {
                    "User"
                };
                row![
                    toggle_group(|| ["User", "Assistant"].map(|s| toggle_group::item(s, s)))
                        .value([value])
                        .style(toggle_group::Default + radius(18))
                        .spacing(8)
                        .on_value_change(move |v| Event::Anchor(
                            v.into_iter().next().unwrap_or_else(|| value.to_owned())
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
                tabs(|t| t.list(["start", "end", "last-anchor"].map(|s| t.trigger(s, s))))
                    .width(Fill)
                    .list_width(Fill)
                    .rounded()
                    .value(current)
                    .on_value_change(Event::Open)
                    .into()
            }
            Self::Group => column![
                button(if state.count < 4 {
                    "Add Rocky"
                } else {
                    "Send Message as Rocky"
                })
                .rounded()
                .style(secondary_surface())
                .width(Fill)
                .disabled(disabled)
                .on_press(Event::Send),
                hint(if state.count < 4 {
                    "This will create a marker and make it the anchor"
                } else {
                    "Now send Rocky's reply into the conversation"
                })
            ]
            .width(Fill)
            .spacing(8)
            .into(),
            Self::History => column![
                container(
                    button(if state.first == 0 {
                        "History Loaded"
                    } else {
                        "Load History"
                    })
                    .rounded()
                    .style(secondary_surface())
                    .width(Fill)
                    .disabled(state.first == 0)
                    .on_press(Event::Send)
                )
                .id("transcript-load-history")
                .width(Fill),
                hint("Restore earlier messages while keeping your place.")
            ]
            .width(Fill)
            .spacing(8)
            .into(),
            Self::Commands => self.commands(),
            Self::Visibility => hint("Track the current anchored turn.").into(),
            Self::Scrollable => text::muted(
                match (state.reader.can_scroll_up, state.reader.can_scroll_down) {
                    (true, true) => "You can scroll both ways.",
                    (false, true) => "You are at the top. You can only scroll down.",
                    (true, false) => "You are at the bottom. You can only scroll up.",
                    (false, false) => "All messages fit in the viewport.",
                },
            )
            .line_height(LineHeight::Absolute(20.into()))
            .width(Fill)
            .align_x(Center)
            .into(),
        }
    }

    fn commands(self) -> Slot<'static, Event> {
        dropdown_menu(
            |menu| menu.trigger(button("Jump to...").rounded().style(secondary_surface())),
            move |_| {
                let title = container(
                    text::muted("Conversations")
                        .size(12)
                        .line_height(LineHeight::Absolute(16.into())),
                )
                .padding([4, 8]);
                let mut items = column![title].width(Fill);
                for (i, turn) in self
                    .turns()
                    .iter()
                    .enumerate()
                    .filter(|(_, turn)| turn.role == Role::User)
                {
                    let label = text::small(turn.short())
                        .width(Fill)
                        .height(20)
                        .line_height(LineHeight::Absolute(20.into()))
                        .ellipsis(Ellipsis::End);
                    let item = menu::item(container(label).width(Fill).padding(2))
                        .on_select(Event::Jump(Target::Item(i as u64)));
                    items = items.push(item.into());
                }
                items.into()
            },
        )
        .width(256)
        .align(dropdown_menu::Align::End)
        .into()
    }

    fn composer(self, state: &Model, send: Slot<'static, Event>) -> Slot<'static, Event> {
        let next = state.next(self);
        let fallback = match self {
            Self::Streaming => "No messages queued. Reset the stream.",
            Self::Context => "No messages queued. Reset the context.",
            _ => "No messages queued. Reset the conversation.",
        };
        let opacity = Alpha(if state.busy() { 0.6 } else { 1.0 });
        let prompt = text::small(next.map_or(fallback, |t| t.text))
            .style(if next.is_some() {
                Foreground + opacity
            } else {
                Muted + opacity
            })
            .line_height(LineHeight::Absolute(20.into()))
            .width(Fill)
            .height(40)
            .ellipsis(Ellipsis::End);
        let prompt = container(prompt)
            .padding(Padding::X3 + Padding::T2_5)
            .width(Fill)
            .height(56);
        let mut toolbar = row![attachments()].spacing(8).align_y(Center).width(Fill);
        if self == Self::Context {
            toolbar = toolbar.push(
                row![
                    text::muted(format!("{}px", state.peek)).size(12),
                    slider(64.0..=128.0)
                        .step(1)
                        .value(f32::from(state.peek))
                        .style(peek)
                        .disabled(state.busy())
                        .on_value_change(Event::Peek)
                ]
                .spacing(8)
                .width(112)
                .into(),
            );
        }
        let toolbar = toolbar
            .push(space::horizontal().into())
            .push(send)
            .padding(Padding::X2_5 + Padding::T1 + Padding::B2);
        container(column![prompt, toolbar].width(Fill))
            .id("transcript-composer")
            .width(Fill)
            .padding(1)
            .style(|theme: &Theme| Style {
                background: Some(theme.input.scale_alpha(0.5).into()),
                border: Border {
                    radius: 18.0.into(),
                    ..Default::default()
                },
                ..Style::default()
            })
            .into()
    }
}

fn hint(label: &'static str) -> text::Text<'static> {
    text::muted(label)
        .size(12)
        .line_height(LineHeight::Absolute(16.into()))
        .width(Fill)
        .align_x(Center)
}

fn peek(theme: &Theme, status: slider::Status) -> slider::Style {
    let paint = <Theme as Catalog>::default();
    let mut style = paint(theme, status).with_circular_handle(8);
    let opacity = if status == slider::Status::Disabled {
        0.5
    } else {
        1.0
    };
    style.rail.backgrounds.1 = theme.input.scale_alpha(0.9 * opacity).into();
    style.handle.border_color = iced::Color::BLACK.scale_alpha(0.1 * opacity);
    if let Some(ring) = &mut style.ring {
        ring.width = 4.;
        ring.color = theme.neutral.ring.scale_alpha(0.3).into();
    }
    style
}

fn secondary_surface() -> StyleFn<'static, Theme> {
    Box::new(|theme, status| {
        let mut style = secondary(theme, status);
        if matches!(status, Status::Hovered | Status::Pressed) {
            style.background = Some(
                theme
                    .secondary
                    .base
                    .blend(theme.neutral.foreground, 0.05)
                    .into(),
            );
        }
        style.into()
    })
}

fn attachments() -> Slot<'static, Event> {
    dropdown_menu(
        |menu| {
            let open = menu.is_open();
            let paint: StyleFn<'_, Theme> = Box::new(move |theme, status| {
                let mut style = outline(theme, status);
                let highlighted = open || matches!(status, Status::Hovered | Status::Pressed);
                style.border.color = theme.neutral.border;
                style.text_color = if highlighted {
                    theme.neutral.foreground
                } else {
                    theme.muted.foreground
                };
                if theme.neutral.is_dark {
                    style.background = highlighted.then(|| theme.input.scale_alpha(0.3).into());
                }
                style.into()
            });
            menu.trigger(
                button(icon::plus().size(16))
                    .square()
                    .rounded()
                    .style(paint),
            )
        },
        |_| {
            column![
                action(icon::paperclip(), "Add Photos & Files"),
                menu::separator(),
                action(icon::image(), "Create Image"),
                action(icon::telescope(), "Deep Research"),
                action(icon::globe(), "Web Search")
            ]
            .width(Fill)
            .into()
        },
    )
    .width(176)
    .align(dropdown_menu::Align::Start)
    .side(dropdown_menu::Side::Top)
    .into()
}

fn action(
    icon: iced_widget::Text<'static, Theme>,
    name: &'static str,
) -> menu::Item<'static, Event> {
    let label = text::small(name).line_height(LineHeight::Absolute(20.into()));
    menu::item(row![icon.size(16), label].spacing(8).padding(2))
}

fn presets(selected: motion::Preset) -> Slot<'static, Event> {
    let items =
        motion::Preset::ALL.map(|preset| select::item(preset.to_string(), preset.to_string()));
    select([select::group(items)])
        .value(selected.to_string())
        .on_value_change(move |name| {
            Event::Preset(
                motion::Preset::ALL
                    .into_iter()
                    .find(|item| item.to_string() == name)
                    .unwrap_or(selected),
            )
        })
        .icon(icon::chevron_down)
        // Rhea's px-3 inside a one-pixel border.
        .padding([0, 13])
        .position(select::Position::Popper)
        .side(select::Side::Top)
        .align(select::Align::Start)
        .trigger_style(|theme, status, _, _| {
            let focused = matches!(status, widget::Status::Focused { .. });
            let mut border = Border::default().rounded(18);
            if focused {
                border = border.width(1).color(theme.neutral.ring);
            }
            widget::Style {
                background: Some(theme.input.scale_alpha(0.5).into()),
                text_color: theme.neutral.foreground.into(),
                border: border.into(),
                ring: focused.then(|| {
                    Border::default()
                        .width(3)
                        .rounded(21)
                        .color(theme.neutral.ring.scale_alpha(0.3))
                        .into()
                }),
                ..Default::default()
            }
        })
        .into()
}
