## Usage

```rust
use icedxdy::widget::button;
```

```rust
button("Button").style(button::Outline).on_press(Message::Pressed)
```

When pressed, this button publishes `Message::Pressed`. Handle that message in `update`. Use `.size(Size::Sm)` for a smaller button or `button![icon::git_branch(), "New Branch"]` for an icon beside the label. Use `.disabled(true)` to disable the button.
