## Usage

```rust
use iced::widget::component::{Component, component};
use iced::{Element, Fill, Pixels, Widget};
use icedxdy::core::Border;
use icedxdy::theme::container::Style;
use icedxdy::widget::calendar::{Date, Datelike, Duration, Mode, Selection, System, today};
use icedxdy::widget::{
    Column, Row, button, calendar, column, container, field, input, input_group, rule, text,
};
use icedxdy::{Size, Slot, Theme, card};

fn bordered<'a, Message: 'a>(content: impl Into<Slot<'a, Message>>) -> Element<'a, Message, Theme> {
    container(content)
        .padding(1)
        .style(|theme: &Theme| Style {
            background: Some(theme.neutral.base.into()),
            border: Border {
                width: 1.,
                radius: 10.0.into(),
                color: theme.neutral.border,
            },
            ..Style::default()
        })
        .into()
}
pub fn example<Message: 'static>() -> Element<'static, Message, Theme> {
    bordered(
        calendar()
            .default_selected(Selection::single(today()))
            .caption_dropdown(true),
    )
}
```

Use `.default_selected(...)` when the date only affects the calendar. Use `.selected(value).on_select(Message::Selected)` when the application needs the selection; store the new optional value in `update`. See [React DayPicker](https://react-day-picker.js.org) for the source component.
