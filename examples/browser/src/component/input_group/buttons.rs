use crate::{Message, icon};
use iced::widget::component::{self, Component};
use iced::{Element, Fill, Widget};
use icedcn::widget::input_group::Align;
use icedcn::widget::{Column, button, column, input, input_group, popover, text};
use icedcn::{Slot, Theme};

const URL: &str = "https://github.com/airstrike/icedcn";

struct Buttons;
#[derive(Default)]
struct State {
    copied: bool,
    favorite: bool,
}
#[derive(Debug, Clone)]
enum Event {
    Copy,
    Favorite,
}

pub(super) fn preview<'a>() -> Element<'a, Message, Theme> {
    component::component(Buttons).boxed()
}

impl<'a> Component<'a, Message, Theme> for Buttons {
    type State = State;
    type Event = Event;
    fn update(&self, state: &mut State, event: Event, _: &iced::Renderer) -> Option<Message> {
        match event {
            Event::Copy => {
                state.copied = true;
                Some(Message::CopyText(URL.to_owned()))
            }
            Event::Favorite => {
                state.favorite = !state.favorite;
                None
            }
        }
    }
    fn view(&self, state: &State) -> impl Widget<Event, Theme> + 'a {
        let copy = if state.copied {
            icon::check()
        } else {
            icon::copy()
        };
        let info = popover(
            |p| {
                p.trigger(
                    input_group::button(icon::info())
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
        let mut favorite = input_group::button(icon::star())
            .square()
            .on_press(Event::Favorite);
        if state.favorite {
            favorite = favorite.style(button::Secondary);
        }
        let end = |content: Slot<'a, Event>| input_group::addon(content).align(Align::InlineEnd);
        let groups = [
            input_group(input(URL).value(""))
                .addon(end(input_group::button(copy)
                    .square()
                    .on_press(Event::Copy)
                    .into()))
                .into(),
            input_group(input(""))
                .addon(input_group::addon(info))
                .addon(input_group::addon(input_group::text("https://")))
                .addon(end(favorite.into()))
                .into(),
            input_group(input("Type to search..."))
                .addon(end(input_group::button("Search")
                    .style(button::Secondary)
                    .into()))
                .into(),
        ];
        Element::from(
            Column::with_children(groups)
                .spacing(24)
                .width(Fill)
                .max_width(384.),
        )
    }
}
