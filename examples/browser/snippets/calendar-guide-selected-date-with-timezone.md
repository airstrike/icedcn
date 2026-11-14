## Selected Date (With TimeZone)

Calendar selections are civil dates without a time of day. `today()` uses the user's local timezone. Convert a stored instant to the intended timezone before supplying its civil date to `.selected(...)` or `.default_selected(...)`.

```rust
use iced::Element;
use icedcn::Theme;
use icedcn::widget::calendar::{Date, Selection, today};
use icedcn::widget::calendar;

pub fn calendar_for_local_date<Message: 'static>(local_date: Date) -> Element<'static, Message, Theme> {
    calendar().default_selected(Selection::single(local_date)).into()
}

pub fn example<Message: 'static>() -> Element<'static, Message, Theme> {
    calendar_for_local_date(today())
}
```

If selecting the 20th highlights the 19th, check that conversion at the application boundary. Calendar does not convert a civil date through UTC.
