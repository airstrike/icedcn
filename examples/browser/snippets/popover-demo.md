```rust
use iced::{Fill, FillPortion};
use icedcn::View;
use icedcn::widget::field::Orientation;
use icedcn::widget::{button, column, field, input, popover, text};

pub fn view<'a>(window_width: f32) -> View<'a> {
    let size = if window_width < 768. { 16. } else { 14. };
    popover(
        |p| {
            p.trigger(
                button("Open popover")
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
                .label(text::label(label))
                .orientation(Orientation::Horizontal)
                .control_last()
                .width(Fill)
                .into()
            };
            column![
                column![
                    text::label("Dimensions").height(14),
                    text::muted("Set the dimensions for the layer."),
                ]
                .spacing(8),
                field::group([
                    dimension("Width", "100%"),
                    dimension("Max. width", "300px"),
                    dimension("Height", "25px"),
                    dimension("Max. height", "none"),
                ])
                .spacing(8),
            ]
            .width(Fill)
            .spacing(16)
            .into()
        },
    )
    .width(320)
    .into()
}
```
