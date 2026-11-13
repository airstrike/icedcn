```rust
fn slide(i: usize, size: f32, vertical: bool) -> Slot<'static, Message> {
    let label = container(text::p((i + 1).to_string()).size(size).semibold()).center(Fill);
    let face: Slot<'static, Message> = if vertical {
        card(label.height(120)).width(Fill).boxed().into()
    } else {
        aspect_ratio(card(label).width(Fill).height(Fill).boxed(), 1.0).into()
    };
    container(face).padding(4).width(Fill).into()
}
fn gallery(size: f32, visible: usize, gap: f32, vertical: bool) -> Carousel<'static, Message> {
    let rotation = if vertical {
        Orientation::Vertical
    } else {
        Orientation::Horizontal
    };
    carousel(5, move |i| slide(i, size, vertical))
        .visible(visible)
        .spacing(gap)
        .orientation(rotation)
        .controls(
            move || {
                if vertical {
                    icon::arrow_up()
                } else {
                    icon::arrow_left()
                }
            },
            move || {
                if vertical {
                    icon::arrow_down()
                } else {
                    icon::arrow_right()
                }
            },
        )
}
fn frame(content: impl Into<Slot<'static, Message>>, width: f32) -> Slot<'static, Message> {
    container(content).width(Fill).max_width(width).into()
}

fn sized(gap: f32, size: f32) -> Slot<'static, Message> {
    responsive(move |bounds| -> iced::Element<'static, Message, Theme> {
        let count = if bounds.width >= 450.0 { 3 } else { 2 };
        frame(gallery(size, count, gap, false), 472.0).compose()
    })
    .height(188)
    .boxed()
    .into()
}

sized(16.0,30.0)
```
