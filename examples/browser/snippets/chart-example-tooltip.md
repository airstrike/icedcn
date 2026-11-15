```rust
use iced::{Element, Widget, color};
use icedcn::Theme;
use icedcn::widget::chart;
use icedcn::widget::chart::{Data, bar, bars, data, hover, hyozu};

const MONTHS: [&str; 6] = ["January", "February", "March", "April", "May", "June"];

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
    .x_axis_labels(MONTHS)
    .x_axis(|axis| {
        axis.none().show_labels(true).labels(|month: f64| {
            MONTHS
                .get(month.round() as usize)
                .unwrap_or(&"")
                .chars()
                .take(3)
                .collect::<String>()
        })
    })
    .y_axis(|axis| axis.none().show_grid(true))
    .show_tooltip(true)
    .tooltip(
        hyozu::Tooltip::default()
            .tracking_line(false)
            .category_band(hyozu::Color::contrast(color!(0xf5f5f5), color!(0x262626))),
    )
}

fn view<'a, Message: 'a>(visitors: &'a Data, theme: &'a Theme) -> Element<'a, Message, Theme> {
    chart(visitors)
        .height(200)
        .padding(0)
        .style(chart::transparent)
        .hover_group(theme.clone(), |group| {
            hover::Annotation::new(Element::from(
                chart::tooltip(group.entries.iter())
                    .label(&group.label)
                    .font("Geist Mono"),
            ))
            .transparent()
            .padding(0)
        })
        .boxed()
}
```
