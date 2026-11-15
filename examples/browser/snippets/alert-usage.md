## Usage

Pass the width of your application window and a message for the action. The following examples take the resulting wrapping mode directly.

```rust
use crate::icon;
use icedcn::Slot;
use icedcn::widget::{alert, button, text};

fn heads_up<M: Clone + 'static>(window_width: f32, enable: M) -> Slot<'static, M> {
    let wrapping =
        if window_width < 768. { text::Wrapping::Balanced } else { text::Wrapping::Word };
    alert()
        .icon(icon::info().size(16).line_height(1.0))
        .title("Heads up!")
        .description("You can add components and dependencies to your app using the cli.")
        .description_wrapping(wrapping)
        .action(button("Enable").style(button::Outline).on_press(enable))
        .max_width(448)
        .into()
}
```

The icon, title, description and action are optional. Alert inherits its foreground color from the selected style. Use `.style(alert::Destructive)` for an error.
