```rust
use crate::icon;
use iced::Fill;
use icedcn::Size;
use icedcn::Slot;
use icedcn::widget::input_group::{Align, addon};
use icedcn::widget::{button, container, input_group, row, space, textarea};

fn editor<M: Clone + 'static>(size: f32) -> Slot<'static, M> {
    container(
        input_group(
            textarea("console.log('Hello, world!');")
                .text_size(size)
                .min_height(200.),
        )
        .addon(
            addon(
                row![
                    icon::file_code().size(16).line_height(1.0),
                    input_group::text("script.js").font(iced::Font::MONOSPACE),
                    space::horizontal(),
                    input_group::button(icon::refresh_ccw().size(16).line_height(1.0)).square(),
                    input_group::button(icon::copy().size(16).line_height(1.0)).square()
                ]
                .spacing(4)
                .width(Fill),
            )
            .align(Align::BlockStart)
            .divider(),
        )
        .addon(
            addon(
                row![
                    input_group::text("Line 1, Column 1"),
                    space::horizontal(),
                    input_group::button(
                        row!["Run", icon::corner_down_left().size(16).line_height(1.0)].spacing(4)
                    )
                    .size(Size::Sm)
                    .height(32)
                    .style(button::Default)
                ]
                .width(Fill),
            )
            .align(Align::BlockEnd)
            .divider(),
        ),
    )
    .width(Fill)
    .max_width(448.)
    .into()
}
```
