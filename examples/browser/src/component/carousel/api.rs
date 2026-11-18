use iced::widget::component::{Component, component};
use iced::{Element, Fill, Widget};
use icedcn::widget::{carousel, column, container, text};
use icedcn::{Slot, Theme};

use super::slide;

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

pub(super) fn view<'a, Message: Clone + 'a>(window_width: f32) -> Slot<'a, Message> {
    component(Example {
        width: if window_width < 640.0 { 160.0 } else { 320.0 },
    })
    .boxed()
    .into()
}
