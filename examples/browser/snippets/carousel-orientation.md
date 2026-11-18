```rust
use iced::Alignment;
use iced::widget::responsive;
use iced::{Element, Fill, Shrink, Widget};
use icedcn::Orientation;
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

pub fn view<'a, Message: Clone + 'static>() -> Slot<'a, Message> {
    carousel(5, |index| slide(index, 30.0, 36.0, true, 4.0))
        .orientation(Orientation::Vertical)
        .align(Alignment::Start)
        .visible(2)
        .spacing(4.0)
        .width(320)
        .height(266.0)
        .into()
}
```
