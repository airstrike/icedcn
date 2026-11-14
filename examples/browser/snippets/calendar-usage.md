## Usage

```rust
use iced::Element;
use icedcn::Theme;
use icedcn::widget::calendar;
use icedcn::widget::calendar::Selection;

#[derive(Clone, Debug)]
pub enum Message { Selected(Option<Selection>) }

pub fn view(value: Option<Selection>) -> Element<'static, Message, Theme> {
    calendar().selected(value).on_select(Message::Selected).into()
}
```

Store the new selection in your application’s `update`. For a calendar that manages its own selection, use `.default_selected(...)`.
