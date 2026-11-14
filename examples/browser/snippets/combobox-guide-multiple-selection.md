## Multiple Selection

Use `.multiple(true)` with chips for multi-select behavior. In a controlled combobox, `.values(...)` supplies the selection and `.on_values_change(...)` publishes the complete new selection. Store it in `update`:

```rust
use iced::Element;
use icedcn::Theme;
use icedcn::widget::combobox;

#[derive(Default)]
struct State {
    selected: Vec<String>,
}

#[derive(Debug, Clone)]
enum Message {
    Selection(Vec<String>),
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::Selection(values) => self.selected = values,
        }
    }

    fn view(&self) -> Element<'_, Message, Theme> {
        combobox(["iced", "egui", "Slint", "GTK", "Druid"])
            .values(self.selected.clone())
            .on_values_change(Message::Selection)
            .placeholder("Add framework")
            .width(320)
            .into()
    }
}
```
