//! Theme customization controls in the Create rail.

use iced::{Fill, Length};
use iced_widget::button::Status;
use icedcn::Theme;
use icedcn::core::Border;
use icedcn::theme::container::Style;
use icedcn::widget::{Size, Slot, button, column, container, native_select, row, scrollable, text};

use crate::{Message, Mode, State, customize};

pub fn view(state: &State, compact: bool) -> iced::Element<'_, Message, Theme> {
    let appearance = row![
        button("Light")
            .style(button::Outline)
            .size(Size::Sm)
            .width(Fill)
            .on_press(Message::SetMode(Mode::Light)),
        button("Dark")
            .style(button::Outline)
            .size(Size::Sm)
            .width(Fill)
            .on_press(Message::SetMode(Mode::Dark)),
    ]
    .spacing(8);
    let fields = column![
        text::small("Customize").semibold(),
        text::muted(if state.mode == Mode::Dark {
            "Dark appearance"
        } else {
            "Light appearance"
        }),
        appearance,
        picker(
            "Base color",
            state.settings.base,
            customize::BASES,
            Message::SetBase
        ),
        picker(
            "Theme",
            state.settings.accent,
            customize::ACCENTS,
            Message::SetAccent
        ),
        picker(
            "Chart color",
            state.settings.chart,
            customize::ACCENTS,
            Message::SetChart
        ),
        picker(
            "Menu accent",
            state.settings.menu,
            customize::MENUS,
            Message::SetMenu
        ),
        text::xs("Nova style · Lucide icons · Default radius"),
        text::muted(format!("Font: {}", state.settings.font.family)),
        text::xs("Choose a font from the browser toolbar."),
    ]
    .spacing(12)
    .width(Fill);
    let actions = column![
        row![
            button("Reset")
                .style(button::Outline)
                .size(Size::Sm)
                .width(Fill)
                .on_press(Message::Reset),
            button("Next preset")
                .style(button::Outline)
                .size(Size::Sm)
                .width(Fill)
                .on_press(Message::NextPreset),
        ]
        .spacing(8),
        button("Get Code")
            .size(Size::Sm)
            .width(Fill)
            .style(rail_primary_button_style())
            .on_press(Message::CodeOpen(true)),
    ]
    .spacing(8)
    .width(Fill);
    container(column![scrollable(fields).height(Fill).width(Fill), actions,].spacing(16))
        .padding(14)
        .width(if compact { Fill } else { Length::Fixed(224.0) })
        .height(Fill)
        .clip(true)
        .style(rail_style)
        .into()
}

fn picker(
    label: &'static str,
    selected: &'static str,
    choices: &'static [&'static str],
    on_change: fn(String) -> Message,
) -> Slot<'static, Message> {
    column![
        text::xs(label),
        native_select(
            choices
                .iter()
                .map(|value| native_select::item(*value, *value))
        )
        .value(selected)
        .width(Fill)
        .on_value_change(on_change),
    ]
    .spacing(4)
    .width(Fill)
    .into()
}

fn rail_style(theme: &Theme) -> Style {
    Style {
        text_color: Some(theme.sidebar.foreground),
        background: Some(theme.sidebar.background.into()),
        border: Border {
            radius: 18.0.into(),
            width: 1.0,
            color: theme.sidebar.border,
        },
        ..Style::default()
    }
}

fn rail_primary_button_style() -> iced_widget::button::StyleFn<'static, Theme> {
    Box::new(|theme, status| {
        let background = match status {
            Status::Disabled => theme.sidebar.foreground.alpha(0.55),
            Status::Pressed => theme.sidebar.foreground.alpha(0.86),
            Status::Hovered | Status::Active => theme.sidebar.foreground,
        };

        iced_widget::button::Style {
            background: Some(iced::Background::Color(background.into())),
            text_color: theme.sidebar.background.into(),
            border: iced::Border {
                radius: 8.0.into(),
                ..iced::Border::default()
            },
            ..Default::default()
        }
    })
}
