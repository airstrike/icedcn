use iced::widget::component::{Component, component};
use iced::{Element, Fill, Task, Widget};

use icedxdy::widget::{column, container, row, slider, space, text};
use icedxdy::{Theme, Vertical};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Slider",
    description: "An input where the user selects a value from within a given range.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/slider-demo.md",
    },
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "range",
                title: "Range",
                description: Some("Use two thumbs to select a range of values."),
                snippet: "examples/browser/snippets/slider-range.md",
            },
            page::Block::Example {
                id: "multiple",
                title: "Multiple Thumbs",
                description: Some("Use multiple thumbs on the same track."),
                snippet: "examples/browser/snippets/slider-multiple.md",
            },
            page::Block::Example {
                id: "vertical",
                title: "Vertical",
                description: Some(
                    "Set the orientation to vertical to lay out the track from bottom to top.",
                ),
                snippet: "examples/browser/snippets/slider-vertical.md",
            },
            page::Block::Example {
                id: "controlled",
                title: "Controlled",
                description: None,
                snippet: "examples/browser/snippets/slider-controlled.md",
            },
            page::Block::Example {
                id: "disabled",
                title: "Disabled",
                description: Some("Use `.disabled(true)` to disable the slider."),
                snippet: "examples/browser/snippets/slider-disabled.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/slider-api-reference.md"),
];

pub struct SliderPage {
    state: page::State,
}

impl SliderPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

struct Temperature;
impl<'a> Component<'a, Message, Theme> for Temperature {
    type State = Option<[f32; 2]>;
    type Event = [f32; 2];

    fn update(
        &self,
        state: &mut Self::State,
        values: Self::Event,
        _: &iced::Renderer,
    ) -> Option<Message> {
        *state = Some(values);
        None
    }
    fn view(&self, state: &Self::State) -> impl Widget<Self::Event, Theme> + 'a {
        let values = state.unwrap_or([0.3, 0.7]);
        Element::from(
            column![
                row![
                    text::label("Temperature"),
                    space::horizontal(),
                    text::muted(format!("{}, {}", values[0], values[1]))
                ]
                .width(Fill),
                slider::range(0.0..=1.0)
                    .value(values)
                    .step(0.1)
                    .on_value_change(|values| values),
            ]
            .spacing(12)
            .max_width(320.0)
            .width(Fill),
        )
    }
}

impl page::Page for SliderPage {
    fn page_index(&self) -> usize {
        self.state.page_index
    }
    fn meta(&self) -> &'static page::Meta {
        &META
    }
    fn blocks(&self) -> &'static [page::Block] {
        BLOCKS
    }
    fn state(&self) -> &page::State {
        &self.state
    }
    fn scrollable_id(&self) -> iced::widget::Id {
        self.state.scrollable_id.clone()
    }

    fn init(&self) -> Task<Message> {
        self.state.init()
    }

    fn update(&mut self, event: page::Message) -> Task<Message> {
        self.state.update(&event)
    }

    fn reset_volatile(&mut self) {
        self.state.reset_volatile();
    }

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        match id {
            // shadcn `slider-demo.tsx`: `mx-auto w-full max-w-xs`.
            "demo" => container(slider(0.0..=100.0).default_value(75.0).step(1.0))
                .width(Fill)
                .max_width(320.0)
                .into(),

            // shadcn `slider-disabled.tsx`.
            "disabled" => container(
                slider(0.0..=100.0)
                    .default_value(50.0)
                    .step(1.0)
                    .disabled(true),
            )
            .width(Fill)
            .max_width(320.0)
            .into(),

            "range" => container(
                slider::range(0.0..=100.0)
                    .default_value([25.0, 50.0])
                    .step(5),
            )
            .width(Fill)
            .max_width(320.0)
            .into(),

            "multiple" => container(slider::multiple(0.0..=100.0, [10.0, 20.0, 70.0]).step(10))
                .width(Fill)
                .max_width(320.0)
                .into(),

            "vertical" => row![
                slider(0.0..=100.0)
                    .default_value(50.0)
                    .orientation(Vertical)
                    .height(160),
                slider(0.0..=100.0)
                    .default_value(25.0)
                    .orientation(Vertical)
                    .height(160),
            ]
            .spacing(24)
            .into(),

            "controlled" => component(Temperature).boxed(),

            _ => unreachable!("unknown slider preview id: {id}"),
        }
    }
}
