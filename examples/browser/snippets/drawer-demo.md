```rust
use crate::icon;
use iced::widget::Space;
use iced::{Alignment, Fill};
use icedcn::theme::container::Transparent;
use icedcn::theme::Bg;
use icedcn::widget::{button, column, container, drawer, row, text};

let goal = state.0.unwrap_or(350);
let chart = [
    400, 300, 200, 300, 200, 278, 189, 239, 300, 200, 278, 189, 349,
];
drawer(
    |d| d.trigger(button("Open Drawer").style(button::Outline)),
    move |d| {
        let bars = row::Row::with_children(chart.into_iter().map(|v| {
            container(Space::new())
                .width(Fill)
                .height(v as f32 * 0.3)
                .style(Transparent + Bg::Primary)
                .into()
        }))
        .spacing(5)
        .height(120)
        .align_y(Alignment::End);
        let controls = row![
            button(icon::minus())
                .size(icedcn::Size::Sm)
                .square()
                .rounded()
                .style(button::Outline)
                .disabled(goal <= 200)
                .on_press(goal - 10),
            container(
                column![
                    text(goal.to_string()).size(72).bold(),
                    text::muted("CALORIES/DAY").size(11)
                ]
                .align_x(Alignment::Center)
            )
            .center_x(Fill),
            button(icon::plus())
                .size(icedcn::Size::Sm)
                .square()
                .rounded()
                .style(button::Outline)
                .disabled(goal >= 400)
                .on_press(goal + 10),
        ]
        .spacing(8)
        .align_y(Alignment::Center);
        container(
            column![
                drawer::header("Move Goal", "Set your daily activity goal."),
                container(column![controls, bars].spacing(12)).padding([0, 16]),
                drawer::footer([
                    d.close(button("Submit").width(Fill)),
                    d.close(button("Cancel").style(button::Outline).width(Fill))
                ]),
            ]
            .width(Fill.max(384.)),
        )
        .center_x(Fill)
        .into()
    },
)
```
