```rust
use iced::{Element, Widget, color};
use icedcn::Theme;
use icedcn::widget::chart;
use icedcn::widget::chart::{Data, bar, bars, data};

fn visitors() -> Data {
    data(
        bars([
            bar([186, 305, 237, 73, 209, 214])
                .with_name("Desktop")
                .with_color(color!(0x2563eb)),
            bar([80, 200, 120, 190, 130, 140])
                .with_name("Mobile")
                .with_color(color!(0x60a5fa)),
        ])
        .data_labels(None),
    )
    .x_axis_labels(["Jan", "Feb", "Mar", "Apr", "May", "Jun"])
    .x_axis(|axis| axis.none())
    .y_axis(|axis| axis.none())
    .show_tooltip(false)
}

fn view<'a, Message: 'a>(visitors: &'a Data) -> Element<'a, Message, Theme> {
    chart(visitors).height(200).padding(0).boxed()
}
```
