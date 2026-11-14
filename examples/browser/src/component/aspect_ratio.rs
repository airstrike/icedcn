use iced::widget::image;
use iced::{ContentFit, Fill, Task, Widget};
use icedxdy::widget::{aspect_ratio, container};
use icedxdy::{Theme, radius};
use std::sync::LazyLock;

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Aspect Ratio",
    description: "Displays content within a desired ratio.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/aspect-ratio-demo.md",
    },
    page::Block::Md("examples/browser/snippets/aspect-ratio-installation.md"),
    page::Block::Md("examples/browser/snippets/aspect-ratio-usage.md"),
    page::Block::Example {
        id: "square",
        title: "Square",
        description: Some(
            "Pass `1.0` for a square crop.",
        ),
        snippet: "examples/browser/snippets/aspect-ratio-square.md",
    },
    page::Block::Example {
        id: "portrait",
        title: "Portrait",
        description: Some(
            "Pass `9.0 / 16.0` for a portrait crop.",
        ),
        snippet: "examples/browser/snippets/aspect-ratio-portrait.md",
    },
    page::Block::Group {
        title: "API Reference",
        children: &[page::Block::Md(
            "examples/browser/snippets/aspect-ratio-api-reference.md",
        )],
    },
];

pub struct AspectRatioPage {
    state: page::State,
}

impl AspectRatioPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

/// Both galleries crop the same landscape photo.
pub(super) fn cover() -> image::Handle {
    static PHOTO: LazyLock<image::Handle> = LazyLock::new(|| {
        image::Handle::from_bytes(include_bytes!("../../assets/landscape.jpg").as_slice())
    });
    PHOTO.clone()
}

// Darken source pixels before rounding the image corners.
// A separate shade leaves bright image pixels visible around its curved edge.
fn dark_cover() -> image::Handle {
    static PHOTO: LazyLock<image::Handle> = LazyLock::new(|| {
        image::Handle::from_bytes(include_bytes!("../../assets/landscape-dark.png").as_slice())
    });
    PHOTO.clone()
}

impl page::Page for AspectRatioPage {
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

    fn backdrop(&self) -> page::Backdrop {
        page::Backdrop {
            padding: [0, 40].into(),
            ..page::Backdrop::default()
        }
    }

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        self.themed_preview(id, &Theme::default())
    }

    fn themed_preview(&self, id: &str, theme: &Theme) -> iced::Element<'_, Message, Theme> {
        let photo = if theme.neutral.is_dark {
            dark_cover()
        } else {
            cover()
        };
        let photo = image(photo)
            .width(Fill)
            .height(Fill)
            .content_fit(ContentFit::Cover)
            .border_radius(radius::LG);

        // The image stays centered even when it exceeds the space between the
        // frame's vertical padding (the hero is 216px in a 288px frame).
        let (ratio, max_width, frame_height) = match id {
            "demo" => (16.0 / 9.0, 384, 288),
            "square" => (1.0, 192, 288),
            "portrait" => (9.0 / 16.0, 160, 384),
            _ => unreachable!("unknown aspect_ratio preview id: {id}"),
        };
        container(aspect_ratio(photo.boxed(), ratio).max_width(max_width))
            .center_x(Fill)
            .center_y(frame_height)
            .into()
    }
}
