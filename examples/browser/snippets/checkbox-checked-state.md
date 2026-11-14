```rust
use iced::Pixels;
use icedxdy::widget::{checkbox, field, text};

field(checkbox().default_checked(true))
    .orientation(field::Horizontal)
    .label(text::label("Accept terms and conditions").line_height(Pixels(19.25)))
    .width(224)
```

The preview uses `.default_checked(true)` for local state. For controlled state, store the value in your model and apply the complete value from `.on_checked_change` in `update`:

```rust
use iced::Element;
use icedxdy::Theme;
use icedxdy::widget::checkbox;

#[derive(Default)]
struct Example {
    checked: bool,
}

#[derive(Debug, Clone)]
enum Change {
    Checked(bool),
}

impl Example {
    fn update(&mut self, change: Change) {
        let Change::Checked(checked) = change;
        self.checked = checked;
    }

    fn view(&self) -> Element<'_, Change, Theme> {
        checkbox()
            .checked(self.checked)
            .on_checked_change(Change::Checked)
            .into()
    }
}
```
