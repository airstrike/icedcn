```rust
use iced::widget::component::{Component, component};
use iced::widget::responsive;
use iced::{Element, Fill, Shrink, Widget};
use icedcn::widget::column;
use icedcn::widget::{carousel, container, text};
use icedcn::{View, Theme, card};

fn slide<'a, Message: 'a>(
    index: usize,
    font_size: f32,
    line_height: f32,
    vertical: bool,
    margin: f32,
) -> View<'a, Message> {
    let content: View<'a, Message> = if vertical {
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

struct Example {
    width: f32,
}

impl<'a, Message: Clone + 'a> Component<'a, Message, Theme> for Example {
    type State = usize;
    type Event = usize;

    fn update(&self, current: &mut usize, selected: usize, _: &iced::Renderer) -> Option<Message> {
        *current = selected;
        None
    }

    fn view(&self, current: &usize) -> impl Widget<usize, Theme> + 'a {
        let gallery = carousel(5, |index| slide(index, 36.0, 40.0, false, 1.0))
            .width(self.width)
            .value(*current)
            .on_value_change(std::convert::identity);
        let count = gallery.snap_count();
        Element::from(
            column![
                gallery,
                container(
                    text::muted(format!("Slide {} of {count}", current + 1))
                        .size(14.0)
                        .line_height(iced::Pixels(20.0))
                        .height(20.0)
                )
                .center_x(Fill)
                .padding([8, 0]),
            ]
            .spacing(0),
        )
    }
}

pub fn view<'a, Message: Clone + 'a>(window_width: f32) -> View<'a, Message> {
    component(Example {
        width: if window_width < 640.0 { 160.0 } else { 320.0 },
    })
    .boxed()
    .into()
}
```
