```rust
use icedcn::View;
use icedcn::widget::{button, popover};

pub fn view<'a>() -> View<'a> {
    popover(
        |p| {
            p.trigger(
                button("Open Popover")
                    .style(button::Outline)
                    .has_popup(true),
            )
        },
        |_| popover::header("Dimensions", "Set the dimensions for the layer."),
    )
    .align(popover::Align::Start)
    .into()
}
```
