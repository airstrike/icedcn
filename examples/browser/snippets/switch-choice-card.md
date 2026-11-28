```rust
use icedcn::widget::{field, switch};
use icedcn::{Horizontal, View};

fn preferences<'a, M: Clone + 'a>() -> View<'a, M> {
    field::group([
        field(switch())
            .orientation(Horizontal)
            .control_last()
            .label(field::label("Share across devices").line_height(20. / 14.))
            .description("Focus is shared across devices, and turns off when you leave the app.")
            .card()
            .into(),
        field(switch().default_checked(true))
            .orientation(Horizontal)
            .control_last()
            .label(field::label("Enable notifications").line_height(20. / 14.))
            .description("Receive notifications when focus mode is enabled or disabled.")
            .card()
            .into(),
    ])
    .width(384)
    .into()
}
```
