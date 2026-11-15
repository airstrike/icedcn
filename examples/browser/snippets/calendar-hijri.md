```rust
use iced::Element;
use icedcn::Theme;
use icedcn::widget::calendar;
use icedcn::widget::calendar::{Date, Selection, System};

pub fn hijri<Message: 'static>() -> Element<'static, Message, Theme> {
    calendar()
        .bordered(true)
        .system(System::Persian)
        .cell_size(32.)
        .padding(12.)
        .default_selected(Selection::single(
            Date::from_ymd_opt(2025, 6, 12).expect("valid Solar Hijri example date"),
        ))
        .into()
}

fn main() -> iced::Result {
    use icedcn::widget::calendar::persian;
    iced::application(|| (), |_: &mut (), _: ()| {}, |_: &()| hijri::<()>())
        .fonts([persian::FONT])
        .theme(|_: &()| Theme::light())
        .run()
}
```
