use crate::icon;
use crate::{Message, page};
use iced::Task;
use iced::widget::responsive;
use iced::{Fill, Widget};
use icedxdy::Theme;
use icedxdy::widget::carousel::Carousel;
use icedxdy::widget::{aspect_ratio, carousel, container, text};
use icedxdy::{Orientation, Slot, card};
use std::time::Duration;

pub static META: page::Meta = page::Meta {
    title: "Carousel",
    description: "A swipeable carousel with keyboard controls and optional autoplay.",
};
pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/carousel-demo.md",
    },
    page::Block::Md("examples/browser/snippets/carousel-composition.md"),
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "size",
                title: "Sizes",
                description: Some("Set the number of visible items with `.visible(n)`."),
                snippet: "examples/browser/snippets/carousel-size.md",
            },
            page::Block::Example {
                id: "spacing",
                title: "Spacing",
                description: Some("Set the gap between slides with `.spacing(px)`."),
                snippet: "examples/browser/snippets/carousel-spacing.md",
            },
            page::Block::Example {
                id: "orientation",
                title: "Orientation",
                description: Some(
                    "Use the orientation prop to set the orientation of the carousel.",
                ),
                snippet: "examples/browser/snippets/carousel-orientation.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/carousel-options.md"),
    page::Block::Example {
        id: "api",
        title: "API",
        description: Some("Use the selected snap index to display the current slide."),
        snippet: "examples/browser/snippets/carousel-api.md",
    },
    page::Block::Md("examples/browser/snippets/carousel-events.md"),
    page::Block::Example {
        id: "plugin",
        title: "Plugins",
        description: Some(
            "Advance every two seconds. Hover to pause. Manual navigation stops autoplay.",
        ),
        snippet: "examples/browser/snippets/carousel-plugin.md",
    },
    page::Block::Md("examples/browser/snippets/carousel-api-reference.md"),
];
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
pub struct CarouselPage {
    state: page::State,
}

impl CarouselPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

impl page::Page for CarouselPage {
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
            "demo" => frame(gallery(36.0, 1, 16.0, false), 408.0).into(),
            "size" => sized(16.0, 30.0).into(),
            "spacing" => sized(4.0, 24.0).into(),
            "orientation" => frame(gallery(30.0, 2, 4.0, true), 320.0).into(),
            "api" => frame(gallery(36.0, 1, 16.0, false).caption(true), 408.0).into(),
            "plugin" => frame(
                gallery(36.0, 1, 16.0, false).autoplay(Duration::from_secs(2)),
                408.0,
            )
            .into(),
            _ => unreachable!("unknown carousel preview id: {id}"),
        }
    }
}
