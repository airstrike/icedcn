```rust
use crate::icon;
use iced::Element;
use icedcn::widget::chart::{self, tooltip};
use icedcn::widget::{column, container, row, text};
use icedcn::{Slot, Theme, theme};

type Message = ();

fn view<'a>(window_width: f32) -> Element<'a, Message, Theme> {
    let [first, second, third, fourth, _] = theme::chart::COLORS;
    let panel = |content: Slot<'a, Message>| {
        container(content)
            .width(224)
            .height(137)
            .center_x(224)
            .center_y(137)
    };
    let label = panel(
        row![
            text("Label").size(14).medium(),
            icon::arrow_right(),
            chart::tooltip([("Desktop", 186, first), ("Mobile", 80, second)])
                .label("Page Views")
                .font("Geist Mono")
                .width(128),
        ]
        .spacing(8)
        .align_y(iced::Center)
        .into(),
    );
    let name = panel(
        column![
            row![text("Name").size(14).medium(), icon::arrow_down()].spacing(8),
            chart::tooltip([("Chrome", 1286, third), ("Firefox", 1000, fourth)])
                .label("Browser")
                .hide_label(true)
                .indicator(tooltip::Dashed)
                .font("Geist Mono")
                .width(128),
        ]
        .spacing(8)
        .into(),
    );
    let line = panel(
        chart::tooltip([("Desktop", 12486, third)])
            .label("Page Views")
            .indicator(tooltip::Line)
            .font("Geist Mono")
            .width(144)
            .into(),
    );
    let indicator = panel(
        column![
            chart::tooltip([("Chrome", 1286, first)])
                .label("Browser")
                .hide_label(true)
                .font("Geist Mono")
                .width(128),
            row![icon::arrow_up(), text("Indicator").size(14).medium()].spacing(8),
        ]
        .spacing(8)
        .into(),
    );
    if window_width < 768.0 {
        column![label, name, indicator].into()
    } else {
        column![row![label, name], row![line, indicator]].into()
    }
}
```
