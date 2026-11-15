```rust
use iced::{Color, Element, Fill, Widget, color};
use icedcn::theme::tweak::Bg;
use icedcn::widget::chart::{Data, bar, bars, data, hover};
use icedcn::widget::{chart, column, container, row, space, text};
use icedcn::{Theme, radius, theme};

type Message = ();

const LABELS: [&str; 6] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun"];
const DESKTOP: [f32; 6] = [186.0, 305.0, 237.0, 73.0, 209.0, 214.0];
const MOBILE: [f32; 6] = [80.0, 200.0, 120.0, 190.0, 130.0, 140.0];
const BLUE: Color = color!(0x2563eb);
const LIGHT_BLUE: Color = color!(0x60a5fa);

fn monthly() -> Data {
    data(
        bars([
            bar(DESKTOP).with_name("Desktop").with_color(BLUE),
            bar(MOBILE).with_name("Mobile").with_color(LIGHT_BLUE),
        ])
        .data_labels(None),
    )
    .x_axis_labels(LABELS)
    .x_axis(|axis| axis.none())
    .y_axis(|axis| axis.none())
    .show_tooltip(false)
}

fn tooltip<'a>(entry: &hover::Entry<'_>) -> hover::Annotation<'a, Message, Theme> {
    let index = match entry {
        hover::Entry::Cartesian { point_idx, .. } => *point_idx,
        _ => 0,
    };
    let index = index.min(LABELS.len() - 1);
    let item = |name, value: f32, color: Color| {
        row![
            container(space::horizontal()).width(8).height(8).class(
                theme::container::Transparent + Bg::from(icedcn::Color::from(color)) + radius(2)
            ),
            text::muted(name).size(12).width(Fill),
            text::small(value.to_string()).medium(),
        ]
        .spacing(8)
        .align_y(iced::Center)
    };
    hover::Annotation::new(Element::from(
        container(
            column![
                text::small(LABELS[index]).medium(),
                item("Desktop", DESKTOP[index], BLUE),
                item("Mobile", MOBILE[index], LIGHT_BLUE),
            ]
            .spacing(6),
        )
        .width(160)
        .padding(10)
        .class(theme::container::Card + radius(8)),
    ))
    .padding(0)
}

fn visitors() -> Data {
    monthly()
        .y_axis(|axis| axis.show_grid(true))
        .x_axis(|axis| axis.show_labels(true))
        .show_tooltip(true)
}

fn view<'a>(visitors: &'a Data) -> Element<'a, Message, Theme> {
    chart(visitors)
        .height(200)
        .padding(0)
        .hover(tooltip)
        .boxed()
}
```
