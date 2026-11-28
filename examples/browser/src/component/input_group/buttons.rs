use crate::{Message, icon};
use iced::widget::component::{self, Component};
use iced::{Element, Fill, Widget};
use icedcn::core::radius;
use icedcn::theme::text_input;
use icedcn::widget::code_block::copy_button;
use icedcn::widget::input_group::{Align, addon};
use icedcn::widget::{button, column, input, input_group, popover, text};
use icedcn::{Theme, View};

struct Buttons(f32);

pub(super) fn preview(size: f32) -> Element<'static, Message, Theme> {
    component::component(Buttons(size)).boxed()
}

impl Component<'static, Message, Theme> for Buttons {
    type State = bool;
    type Event = bool;

    fn update(&self, state: &mut bool, favorite: bool, _: &iced::Renderer) -> Option<Message> {
        *state = favorite;
        None
    }

    fn view(&self, favorite: &bool) -> impl Widget<bool, Theme> + 'static {
        buttons(self.0, *favorite, !*favorite).compose()
    }
}

fn buttons<'a, M: Clone + 'static>(size: f32, favorite: bool, toggle: M) -> View<'a, M> {
    let url = "https://github.com/airstrike/icedcn";
    let info = popover(
        |p| {
            p.trigger(
                input_group::button(icon::info().size(14))
                    .style(button::Secondary)
                    .square(),
            )
        },
        |_| {
            column![
                text::label("Your connection is not secure."),
                text::small("You should not enter any sensitive information on this site.")
            ]
            .spacing(4)
            .into()
        },
    )
    .align(popover::Align::Start)
    .width(264.);
    let mut star = icon::star().size(14);
    if favorite {
        star = star.color(iced::color!(21, 93, 252));
    }
    column![
        input_group(input(url).value("").text_size(size))
            .addon(addon(copy_button(url)).align(Align::InlineEnd)),
        input_group(input("").text_size(size))
            .style(text_input::Default + radius(9999))
            .addon(addon(info))
            .addon(addon(input_group::text("https://")))
            .addon(
                addon(input_group::button(star).square().on_press(toggle)).align(Align::InlineEnd),
            ),
        input_group(input("Type to search...").text_size(size)).addon(
            addon(input_group::button("Search").style(button::Secondary)).align(Align::InlineEnd),
        ),
    ]
    .spacing(24)
    .width(Fill)
    .max_width(384.)
    .into()
}
