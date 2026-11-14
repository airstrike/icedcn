```rust
use iced::widget::component::{Component, component};
use iced::{Element, Fill, Pixels, Widget};
use icedcn::core::Border;
use icedcn::theme::container::Style;
use icedcn::widget::calendar::{Date, Datelike, Duration, Mode, Selection, System, today};
use icedcn::widget::{
    Column, Row, button, calendar, column, container, field, input, input_group, rule, text,
};
use icedcn::{Size, Slot, Theme, card};

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
pub fn hijri<Message: 'static>() -> Element<'static, Message, Theme> {
    bordered(
        calendar()
            .system(System::Persian)
            .cell_size(32.)
            .padding(12.)
            .default_selected(Selection::single(
                Date::from_ymd_opt(2025, 6, 12).unwrap_or_else(today),
            )),
    )
}

fn main() -> iced::Result {
    use iced::font;
    use icedcn::widget::calendar::persian;
    iced::application(
        || ((), font::load(persian::FONT).map(|_| ())),
        |_: &mut (), _: ()| {},
        |_: &()| hijri::<()>(),
    ).theme(|_: &()| Theme::light()).run()
}
```
