## Usage

Compose an alert with the parts you need. It fits the available width and wraps its text automatically.

```rust
use crate::icon;
use icedcn::View;
use icedcn::widget::{alert, button};

fn heads_up<'a, M: Clone + 'a>(enable: M) -> View<'a, M> {
    alert()
        .icon(icon::info().size(16).line_height(1.0))
        .title("Heads up!")
        .description("You can add components and dependencies to your app using the cli.")
        .action(button("Enable").style(button::Outline).on_press(enable))
        .max_width(448)
        .into()
}
```

The icon, title, description and action are optional. Alert inherits its foreground color from the selected style. Use `.style(alert::Destructive)` for an error.
