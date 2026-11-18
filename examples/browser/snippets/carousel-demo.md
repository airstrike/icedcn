```rust
use iced::widget::responsive;
use iced::{Element, Fill, Shrink, Widget};
use icedcn::widget::{carousel, container, text};
use icedcn::{Slot, Theme, card};

fn slide<'a, Message: 'a>(
    index: usize,
    font_size: f32,
    line_height: f32,
    vertical: bool,
    margin: f32,
) -> Slot<'a, Message> {
    let content: Slot<'a, Message> = if vertical {
        container(
            text::p((index + 1).to_string())
                .size(font_size)
                .semibold()
                .line_height(iced::Pixels(line_height))
                .height(line_height),
        )
        .center_x(Fill)
        .padding(24)
        .into()
    } else {
        responsive(move |bounds| -> Element<'a, Message, Theme> {
            container(
                text::p((index + 1).to_string())
                    .size(font_size)
                    .semibold()
                    .line_height(iced::Pixels(line_height))
                    .height(line_height),
            )
            .center_x(Fill)
            .center_y(bounds.width.max(line_height + 48.0))
            .padding(24)
            .into()
        })
        .height(Shrink)
        .boxed()
        .into()
    };
    container(
        card(container(content).padding([16, 0]).width(Fill))
            .gap(0)
            .padding(0)
            .width(Fill),
    )
    .padding(margin)
    .width(Fill)
    .into()
}

pub fn view<'a, Message: Clone + 'static>(window_width: f32) -> Slot<'a, Message> {
    carousel(5, |index| slide(index, 36.0, 40.0, false, 4.0))
        .width(if window_width < 640.0 { 192.0 } else { 320.0 })
        .into()
}
```
