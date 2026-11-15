//! Theme customization controls in the Create rail.

use iced::font::Family;
use iced::widget::{space, themer};
use iced::{Center, Fill, Font, Length, Pixels, Widget};
use iced_widget::button::Status;
use icedcn::Theme;
use icedcn::core::{Border, Color, Design, oklch};
use icedcn::theme::container::Style;
use icedcn::theme::text::Muted;
use icedcn::widget::{Slot, button, column, container, dropdown_menu, menu, row, scrollable, text};

use crate::{Message, Mode, State, customize, icon};

/// The customizer keeps its neutral dark appearance independently of the preview.
pub fn view(state: &State, viewport_width: f32) -> iced::Element<'_, Message, Theme> {
    let font = match state.settings.font.family {
        Family::Name(name) => name.to_owned(),
        Family::Serif => "Serif".to_owned(),
        Family::Monospace => "Monospace".to_owned(),
        _ => "Sans Serif".to_owned(),
    };
    let fields = column![
        group([fixed(
            "Style",
            "Nova",
            Mark::Style,
            "Nova is the native icedcn style."
        )]),
        field_separator(),
        group([
            picker(
                "Base Color",
                state.settings.base,
                customize::BASES,
                Mark::Swatch(state.settings.base),
                Message::SetBase
            ),
            picker(
                "Theme",
                state.settings.accent,
                customize::ACCENTS,
                Mark::Swatch(state.settings.accent),
                Message::SetAccent
            ),
            picker(
                "Chart Color",
                state.settings.chart,
                customize::ACCENTS,
                Mark::Swatch(state.settings.chart),
                Message::SetChart
            ),
        ]),
        field_separator(),
        group([
            inherited_font(&font),
            font_picker(&font, state.host_font_picker),
        ]),
        field_separator(),
        group([
            fixed(
                "Icon Library",
                "Lucide",
                Mark::Icon,
                "Lucide is the installed icon library."
            ),
            fixed(
                "Radius",
                "Default",
                Mark::Radius,
                "Nova's default corner radii are used."
            ),
        ]),
        field_separator(),
        group([
            fixed(
                "Menu",
                "Default / Solid",
                Mark::Menu,
                "Menus use the preview's solid surface."
            ),
            picker(
                "Menu Accent",
                state.settings.menu,
                customize::MENUS,
                Mark::Accent,
                Message::SetMenu
            ),
        ]),
    ]
    .spacing(13)
    .width(Fill);
    let header = container(main_menu(state.mode))
        .padding([12, 12])
        .width(Fill);
    let actions = container(
        column![
            action(
                if state.copied { "Copied" } else { "Copy Code" },
                Message::CopyCode
            ),
            action("Reset", Message::Reset),
            action("Next Preset", Message::NextPreset),
        ]
        .spacing(8)
        .width(Fill),
    )
    .padding(12)
    .width(Fill)
    .style(footer_style);
    let export = container(
        button(text("Get Code").size(14).medium())
            .height(32)
            .padding([0, 8])
            .width(Fill)
            .style(primary_style())
            .on_press(Message::CodeOpen(true)),
    )
    .padding(12)
    .width(Fill)
    .style(footer_style);
    let rail = container(
        column![
            header,
            divider(),
            scrollable(container(fields).padding([13, 0]).width(Fill))
                .height(Fill)
                .width(Fill),
            divider(),
            actions,
            divider(),
            export,
        ]
        .width(Fill)
        .height(Fill),
    )
    .width(if viewport_width < 760.0 {
        Fill
    } else {
        Length::Fixed(if viewport_width >= 1536.0 {
            224.0
        } else {
            192.0
        })
    })
    .height(Fill)
    .max_height(986)
    .clip(true)
    .style(rail_style);
    let mut design = Design::dark();
    design.popover.base = iced::color!(0x262626).into();
    themer(Some(Theme::new(design)), iced::Element::from(rail))
        .text_color(|theme| theme.neutral.foreground.into())
        .boxed()
}

#[derive(Clone, Copy)]
enum Mark {
    Style,
    Swatch(&'static str),
    Font,
    Icon,
    Radius,
    Menu,
    Accent,
}

impl Mark {
    fn view(self) -> Slot<'static, Message> {
        match self {
            Self::Swatch(name) => container(space().width(16).height(16))
                .style(move |_| Style {
                    background: Some(swatch(name).into()),
                    border: Border {
                        radius: 8.0.into(),
                        ..Border::default()
                    },
                    ..Style::default()
                })
                .into(),
            Self::Font => text("Aa").size(16).line_height(Pixels(20.0)).into(),
            Self::Style => icon::rectangle_horizontal().size(16).into(),
            Self::Icon => icon::fingerprint().size(16).into(),
            Self::Radius => icon::radius().size(16).into(),
            Self::Menu => icon::list_filter().size(16).into(),
            Self::Accent => icon::paint_bucket().size(16).into(),
        }
    }
}

fn trigger(label: &str, selected: &str, mark: Mark) -> button::Button<'static, Message> {
    button(
        row![
            column![
                text(label.to_owned())
                    .size(12)
                    .line_height(Pixels(16.0))
                    .style(Muted),
                text(selected.to_owned())
                    .size(14)
                    .medium()
                    .line_height(Pixels(20.0)),
            ]
            .width(Fill),
            mark.view(),
        ]
        .spacing(8)
        .align_y(Center)
        .width(Fill),
    )
    .width(Fill)
    .height(52)
    .padding([8, 10])
    .translate(false)
    .style(control_style())
}

fn option(label: impl Into<String>) -> Slot<'static, Message> {
    container(
        text(label.into())
            .size(14)
            .medium()
            .line_height(Pixels(20.0)),
    )
    .padding(2)
    .width(Fill)
    .into()
}

fn picker(
    label: &'static str,
    selected: &'static str,
    choices: &'static [&'static str],
    mark: Mark,
    on_change: fn(String) -> Message,
) -> Slot<'static, Message> {
    dropdown_menu(
        move |menu| menu.trigger(trigger(label, selected, mark)),
        move |_| {
            column::Column::with_children(choices.iter().map(|value| {
                menu::radio_item(option(*value), *value, Some(selected), |value| {
                    on_change(value.to_owned())
                })
                .into()
            }))
            .padding(2)
            .width(Fill)
            .into()
        },
    )
    .side(dropdown_menu::Side::Right)
    .side_offset(20)
    .align(dropdown_menu::Align::Start)
    .width(208)
    .into()
}

fn fixed(
    label: &'static str,
    value: &'static str,
    mark: Mark,
    explanation: &'static str,
) -> Slot<'static, Message> {
    dropdown_menu(
        move |menu| menu.trigger(trigger(label, value, mark)),
        move |_| {
            column![
                menu::radio_item(option(value), value, Some(value), |_| Message::Noop),
                menu::separator(),
                container(text(explanation).size(12).style(Muted))
                    .padding([6, 8])
                    .width(Fill),
            ]
            .padding(2)
            .width(Fill)
            .into()
        },
    )
    .side(dropdown_menu::Side::Right)
    .side_offset(20)
    .width(208)
    .into()
}

fn inherited_font(font: &str) -> Slot<'static, Message> {
    let font = font.to_owned();
    dropdown_menu(
        move |menu| menu.trigger(trigger("Heading", &font, Mark::Font)),
        |_| {
            column![
                menu::radio_item(option("Inherit body font"), true, Some(true), |_| {
                    Message::Noop
                }),
                menu::label("Choose the body font below."),
            ]
            .padding(2)
            .width(Fill)
            .into()
        },
    )
    .side(dropdown_menu::Side::Right)
    .side_offset(20)
    .width(208)
    .into()
}

fn font_picker(font: &str, host: bool) -> Slot<'static, Message> {
    if host {
        return trigger("Font", font, Mark::Font)
            .on_press(Message::ChooseFont)
            .into();
    }
    let font = font.to_owned();
    dropdown_menu(
        move |menu| menu.trigger(trigger("Font", &font, Mark::Font)),
        |_| {
            column![
                menu::item(option("Sans Serif"))
                    .on_select(Message::SetFont(Font::with_family(Family::SansSerif))),
                menu::item(option("Serif"))
                    .on_select(Message::SetFont(Font::with_family(Family::Serif))),
                menu::item(option("Monospace")).on_select(Message::SetFont(Font::MONOSPACE)),
            ]
            .padding(2)
            .width(Fill)
            .into()
        },
    )
    .side(dropdown_menu::Side::Right)
    .side_offset(20)
    .width(208)
    .into()
}

fn field_separator() -> Slot<'static, Message> {
    container(divider())
        .height(4)
        .align_y(Center)
        .width(Fill)
        .into()
}

fn group<const N: usize>(fields: [Slot<'static, Message>; N]) -> Slot<'static, Message> {
    container(
        column::Column::with_children(fields)
            .spacing(13)
            .width(Fill),
    )
    .padding([0, 12])
    .width(Fill)
    .into()
}

fn main_menu(mode: Mode) -> Slot<'static, Message> {
    dropdown_menu(
        |menu| {
            menu.trigger(
                button(
                    row![
                        text("Menu").size(14).medium().width(Fill),
                        icon::menu().size(20)
                    ]
                    .align_y(Center)
                    .width(Fill),
                )
                .height(36)
                .padding([0, 8])
                .width(Fill)
                .translate(false)
                .style(control_style()),
            )
        },
        move |_| {
            column![
                menu::item(option("Get Code")).on_select(Message::CodeOpen(true)),
                menu::item(option("Next Preset")).on_select(Message::NextPreset),
                menu::item(option("Light / Dark")).on_select(Message::SetMode(
                    if mode == Mode::Dark {
                        Mode::Light
                    } else {
                        Mode::Dark
                    }
                )),
                menu::separator(),
                menu::item(option("Reset")).on_select(Message::Reset),
            ]
            .padding(2)
            .width(Fill)
            .into()
        },
    )
    .side(dropdown_menu::Side::Right)
    .side_offset(20)
    .width(208)
    .into()
}

fn action(label: &str, message: Message) -> Slot<'static, Message> {
    button(text(label.to_owned()).size(14).medium())
        .height(32)
        .padding([0, 8])
        .width(Fill)
        .style(control_style())
        .on_press(message)
        .into()
}

fn divider() -> Slot<'static, Message> {
    container(space().height(1))
        .width(Fill)
        .style(|_| Style {
            background: Some(oklch(0.28, 0.0, 0.0).into()),
            ..Style::default()
        })
        .into()
}

fn swatch(name: &str) -> Color {
    match name {
        "Neutral" => oklch(0.708, 0.0, 0.0),
        "Slate" => oklch(0.704, 0.04, 256.788),
        "Stone" => oklch(0.709, 0.01, 56.259),
        "Zinc" => oklch(0.705, 0.015, 286.067),
        _ => customize::accent(name, true),
    }
}

fn rail_style(_: &Theme) -> Style {
    Style {
        text_color: Some(oklch(0.985, 0.0, 0.0)),
        background: Some(oklch(0.205, 0.0, 0.0).into()),
        border: Border {
            radius: 18.0.into(),
            width: 1.0,
            color: oklch(0.269, 0.0, 0.0),
        },
        ..Style::default()
    }
}

fn footer_style(_: &Theme) -> Style {
    Style {
        background: Some(oklch(0.235, 0.0, 0.0).into()),
        ..Style::default()
    }
}

fn control_style() -> iced_widget::button::StyleFn<'static, Theme> {
    Box::new(|_, status| iced_widget::button::Style {
        background: if matches!(status, Status::Hovered | Status::Pressed) {
            Some(iced::color!(0x262626).into())
        } else {
            None
        },
        text_color: iced::color!(0xfafafa),
        border: iced::Border {
            radius: 10.0.into(),
            width: 1.0,
            color: iced::color!(0x333333),
        },
        ..Default::default()
    })
}

fn primary_style() -> iced_widget::button::StyleFn<'static, Theme> {
    Box::new(|_, status| iced_widget::button::Style {
        background: Some(
            if status == Status::Hovered {
                iced::color!(0xd4d4d4)
            } else {
                iced::color!(0xe5e5e5)
            }
            .into(),
        ),
        text_color: iced::color!(0x171717),
        border: iced::Border {
            radius: 10.0.into(),
            ..Default::default()
        },
        ..Default::default()
    })
}
