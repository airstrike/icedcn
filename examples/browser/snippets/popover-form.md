```rust
use iced::{Fill, FillPortion};
use icedcn::View;
use icedcn::widget::field::Orientation;
use icedcn::widget::{button, column, field, input, popover};

pub fn view<'a>(window_width: f32) -> View<'a> {
    let size = if window_width < 768. { 16. } else { 14. };
    popover(
        |p| {
            p.trigger(
                button("Open Popover")
                    .style(button::Outline)
                    .has_popup(true),
            )
        },
        move |_| {
            let dimension = |label: &'a str, value| {
                field(
                    input("")
                        .default_value(value)
                        .text_size(size)
                        .width(FillPortion(2)),
                )
                .label(label)
                .orientation(Orientation::Horizontal)
                .control_last()
                .width(Fill)
                .into()
            };
            column![
                popover::header("Dimensions", "Set the dimensions for the layer."),
                field::group([dimension("Width", "100%"), dimension("Height", "25px"),])
                    .spacing(16),
            ]
            .width(Fill)
            .spacing(10)
            .into()
        },
    )
    .align(popover::Align::Start)
    .width(256)
    .into()
}
```
