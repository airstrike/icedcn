use crate::{Message, page};
use iced::Task;
use iced::widget::image;
use iced::{ContentFit, Widget};
use icedcn::Theme;
use icedcn::View;
use icedcn::core::Border;
use icedcn::theme;
use icedcn::theme::container::Style;
use icedcn::widget::scrollable::Direction;
use icedcn::widget::{Row, column, container, row, scroll_area, separator, text};
use std::sync::OnceLock;

pub static META: page::Meta = page::Meta {
    title: "Scroll Area",
    description: "Augments native scroll functionality for custom, cross-browser styling.",
};
pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/scroll-area-demo.md",
    },
    page::Block::Md("examples/browser/snippets/scroll-area-composition.md"),
    page::Block::Group {
        title: "Examples",
        children: &[page::Block::Example {
            id: "horizontal-demo",
            title: "Horizontal",
            description: Some("Use a horizontal scrollbar for horizontal scrolling."),
            snippet: "examples/browser/snippets/scroll-area-horizontal-demo.md",
        }],
    },
    page::Block::Md("examples/browser/snippets/scroll-area-api-reference.md"),
];
fn frame<'a>(content: impl Into<View<'a, Message>>) -> View<'a, Message> {
    container(content)
        .clip(true)
        .padding(1)
        .style(|theme: &Theme| Style {
            border: Border {
                color: theme.neutral.border,
                width: 1.0,
                radius: 8.into(),
            },
            ..Style::default()
        })
        .into()
}

pub struct ScrollAreaPage {
    state: page::State,
}

impl ScrollAreaPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

impl page::Page for ScrollAreaPage {
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
            "demo" => {
                let rows = (1..=50).rev().map(|n| {
                    column![text::small(format!("v1.2.0-beta.{n}")), separator()]
                        .spacing(8)
                        .into()
                });
                let contents = column![text::label("Tags"), column(rows).spacing(8)]
                    .spacing(16)
                    .padding(16);
                frame(scroll_area(contents).width(190).height(286))
            }
            .into(),
            "horizontal-demo" => {
                let works = [
                    (
                        "Ornella Binni",
                        include_bytes!("../../assets/artwork/ornella-binni.jpg").as_slice(),
                    ),
                    (
                        "Tom Byrom",
                        include_bytes!("../../assets/artwork/tom-byrom.jpg").as_slice(),
                    ),
                    (
                        "Vladimir Malyavko",
                        include_bytes!("../../assets/artwork/vladimir-malyavko.jpg").as_slice(),
                    ),
                ];
                static IMAGES: [OnceLock<image::Handle>; 3] = [const { OnceLock::new() }; 3];
                let figures = Row::with_children(works.into_iter().zip(&IMAGES).map(
                    |((artist, bytes), cache)| {
                        let handle = cache.get_or_init(|| image::Handle::from_bytes(bytes));
                        column![
                            image(handle.clone())
                                .width(150)
                                .height(200)
                                .content_fit(ContentFit::Cover)
                                .border_radius(6)
                                .boxed(),
                            row![
                                text::xs("Photo by").style(theme::text::Muted),
                                text::xs(artist).semibold()
                            ]
                            .spacing(3)
                        ]
                        .spacing(8)
                        .into()
                    },
                ))
                .spacing(16);
                frame(
                    scroll_area(container(figures).padding(16))
                        .direction(Direction::Horizontal(scroll_area::bar()))
                        .width(382),
                )
            }
            .into(),
            _ => unreachable!("unknown scroll-area preview id: {id}"),
        }
    }
}
