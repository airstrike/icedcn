## Usage

A provider connects the trigger to the panel and stores its open state. This example sends the selected destination to the application.

```rust
use icedcn::View;
use icedcn::widget::{row, sidebar};

pub fn view<'a>() -> View<'a, &'static str> {
    sidebar::provider(|s| {
        let links =
            ["Dashboard", "Settings"].map(|name| sidebar::menu_button(name).on_press(name).into());
        row![
            s.sidebar(sidebar::fixed(sidebar::menu(links))),
            sidebar::main(sidebar::trigger()),
        ]
        .width(iced::Fill)
        .height(iced::Fill)
        .into()
    })
    .into()
}
```
