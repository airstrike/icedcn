//! Carousel navigation, slide layout and autoplay examples.
mod api;

use std::time::Duration;

use iced::widget::responsive;
use iced::{Alignment, Element, Fill, Shrink, Task, Widget};
use icedcn::widget::{carousel, container, text};
use icedcn::{Orientation, Slot, Theme, card};

use crate::{Message, page};

pub static META: page::Meta = page::Meta {
    title: "Carousel",
    description: "A swipeable row or column of slides with animated navigation.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/carousel-demo.md",
    },
    page::Block::Md("examples/browser/snippets/carousel-about.md"),
    page::Block::Md("examples/browser/snippets/carousel-usage.md"),
    page::Block::Md("examples/browser/snippets/carousel-composition.md"),
    page::Block::Example {
        id: "size",
        title: "Sizes",
        description: Some("Use `.visible(...)` to set how many slides share the viewport."),
        snippet: "examples/browser/snippets/carousel-size.md",
    },
    page::Block::Md("examples/browser/snippets/carousel-guide-size.md"),
    page::Block::Example {
        id: "spacing",
        title: "Spacing",
        description: Some("Set the gap between slides in pixels with `.spacing(...)`."),
        snippet: "examples/browser/snippets/carousel-spacing.md",
    },
    page::Block::Md("examples/browser/snippets/carousel-guide-spacing.md"),
    page::Block::Example {
        id: "orientation",
        title: "Orientation",
        description: Some("Set `.orientation(Orientation::Vertical)` to stack slides vertically."),
        snippet: "examples/browser/snippets/carousel-orientation.md",
    },
    page::Block::Md("examples/browser/snippets/carousel-options.md"),
    page::Block::Example {
        id: "api",
        title: "API",
        description: Some(
            "Store the selected slide in your component state and update it when the carousel sends a change message.",
        ),
        snippet: "examples/browser/snippets/carousel-api.md",
    },
    page::Block::Md("examples/browser/snippets/carousel-events.md"),
    page::Block::Md("examples/browser/snippets/carousel-plugins.md"),
    page::Block::Example {
        id: "plugin",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/carousel-plugin.md",
    },
    page::Block::Md("examples/browser/snippets/carousel-api-reference.md"),
];

pub struct CarouselPage {
    state: page::State,
    width: f32,
}

impl CarouselPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
            width: 1440.0,
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
    fn window_width(&mut self, width: f32) {
        self.width = width;
    }
    fn reset_volatile(&mut self) {
        self.state.reset_volatile();
    }
    fn backdrop(&self) -> page::Backdrop {
        page::Backdrop {
            padding: 0.into(),
            ..page::Backdrop::default()
        }
    }
    fn docs_preview(&self, id: &str, theme: &Theme) -> Element<'_, Message, Theme> {
        let height = match id {
            "orientation" => 512,
            "demo" if self.width < 640.0 => 320,
            "demo" | "api" | "plugin" if self.width >= 640.0 => 512,
            _ => 288,
        };
        container(self.themed_preview(id, theme))
            .center_x(Fill)
            .center_y(height)
            .padding([0, if id == "orientation" { 40 } else { 0 }])
            .into()
    }
    fn preview(&self, id: &str) -> Element<'_, Message, Theme> {
        match id {
            "demo" => demo(self.width).compose(),
            "size" => size(self.width).compose(),
            "spacing" => spacing(self.width).compose(),
            "orientation" => orientation().compose(),
            "api" => api::view(self.width).compose(),
            "plugin" => plugin(self.width).compose(),
            _ => unreachable!("unknown carousel preview id: {id}"),
        }
    }
}

fn slide<Message: 'static>(
    index: usize,
    font_size: f32,
    line_height: f32,
    vertical: bool,
    margin: f32,
) -> Slot<'static, Message> {
    let content: Slot<'static, Message> = if vertical {
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
        responsive(move |bounds| -> Element<'static, Message, Theme> {
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

pub fn demo<Message: Clone + 'static>(window_width: f32) -> Slot<'static, Message> {
    carousel(5, |index| slide(index, 36.0, 40.0, false, 4.0))
        .width(if window_width < 640.0 { 192.0 } else { 320.0 })
        .into()
}

pub fn size<Message: Clone + 'static>(window_width: f32) -> Slot<'static, Message> {
    carousel(5, |index| slide(index, 30.0, 36.0, false, 4.0))
        .visible(if window_width < 1024.0 { 2 } else { 3 })
        .align(Alignment::Start)
        .width(if window_width < 640.0 {
            192.0
        } else if window_width < 768.0 {
            320.0
        } else {
            384.0
        })
        .into()
}

pub fn spacing<Message: Clone + 'static>(window_width: f32) -> Slot<'static, Message> {
    carousel(5, |index| slide(index, 24.0, 32.0, false, 4.0))
        .visible(if window_width < 1024.0 { 2 } else { 3 })
        .spacing(4.0)
        .width(if window_width < 640.0 {
            192.0
        } else if window_width < 768.0 {
            320.0
        } else {
            384.0
        })
        .into()
}

pub fn orientation<Message: Clone + 'static>() -> Slot<'static, Message> {
    carousel(5, |index| slide(index, 30.0, 36.0, true, 4.0))
        .orientation(Orientation::Vertical)
        .align(Alignment::Start)
        .visible(2)
        .spacing(4.0)
        .width(320)
        .height(266.0)
        .into()
}

pub fn plugin<Message: Clone + 'static>(window_width: f32) -> Slot<'static, Message> {
    carousel(5, |index| slide(index, 36.0, 40.0, false, 4.0))
        .width(if window_width < 640.0 { 160.0 } else { 320.0 })
        .autoplay(Duration::from_secs(2))
        .stop_on_hover(true)
        .into()
}
