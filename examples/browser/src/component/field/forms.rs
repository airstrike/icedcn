use super::recipes;
use iced::widget::component::Component;
use iced::{Element, Fill, Length, Widget};
use icedcn::Theme;
use icedcn::theme::text_input::Invalid;
use icedcn::widget::{column, field, input, text};
use std::collections::BTreeMap;

/// Each preview owns its edits. A submission only checks the required fields.
pub(super) enum Form {
    Payment(f32),
    Profile(f32),
}

#[derive(Default)]
pub(super) struct State {
    values: BTreeMap<&'static str, String>,
    attempted: bool,
    accepted: bool,
}

#[derive(Debug, Clone)]
pub(super) enum Event {
    Edit(&'static str, String),
    Submit,
}

impl<'a, M: 'a> Component<'a, M, Theme> for Form {
    type State = State;
    type Event = Event;

    fn update(&self, state: &mut State, event: Event, _: &iced::Renderer) -> Option<M> {
        match event {
            Event::Edit(name, value) => {
                state.values.insert(name, value);
                state.accepted = false;
            }
            Event::Submit => {
                state.attempted = true;
                let required: &[&str] = match self {
                    Self::Payment(_) => &["Name on Card", "Card Number", "CVV"],
                    Self::Profile(_) => &["Name"],
                };
                state.accepted = required.iter().all(|name| {
                    state
                        .values
                        .get(name)
                        .is_some_and(|value| !value.trim().is_empty())
                });
            }
        }
        None
    }

    fn view(&self, state: &State) -> impl Widget<Event, Theme> + 'a {
        let size = match *self {
            Self::Payment(size) | Self::Profile(size) => size,
        };
        let control_width = match self {
            Self::Payment(_) => Fill,
            Self::Profile(_) => Length::Fixed(192.),
        };
        let required = |name: &'static str, hint: &'static str| {
            let value = state.values.get(name).cloned().unwrap_or_default();
            let invalid = state.attempted && value.trim().is_empty();
            let control = input(hint)
                .value(value)
                .text_size(size)
                .width(control_width)
                .on_input(move |value| Event::Edit(name, value))
                .on_submit(Event::Submit);
            if invalid {
                field(control.style(Invalid))
                    .label(name)
                    .error(format!("{name} is required."))
            } else {
                field(control).label(name)
            }
        };
        let form = match self {
            Self::Payment(_) => recipes::payment(required, Event::Submit),
            Self::Profile(_) => recipes::profile(required("Name", "Evil Rabbit"), Event::Submit),
        };
        let max_width = match self {
            Self::Payment(_) => 448.,
            Self::Profile(_) => 512.,
        };
        let mut content = column![form].width(Fill).max_width(max_width).spacing(8);
        if state.accepted {
            content = content.push(text::small("Form submitted.").into());
        }
        Element::from(content)
    }
}
