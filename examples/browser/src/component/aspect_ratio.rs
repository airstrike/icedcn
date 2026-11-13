// API Reference is rendered via the `aspect-ratio-api-reference.md` snippet
// at the bottom of BLOCKS. The live docs API Reference covers `ratio`
// (`f32`, required) plus styling props that our snippet captures via the
// icedxdy-shaped builder API (`.width(...)`, `.max_width(...)`, ratio
// fallback semantics).
//
// Intentional skips:
// - aspect-ratio-rtl — RTL is not in scope (per CLAUDE.md).

use iced::widget::{image, stack};
use iced::{ContentFit, Fill, Task, Widget};
use icedxdy::core::{Border, Color};
use icedxdy::theme::container::Style;
use icedxdy::widget::{aspect_ratio, container};
use icedxdy::{Theme, radius};
use std::sync::OnceLock;

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
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "square",
                title: "Square",
                description: Some(
                    "A square aspect ratio component using a `1 / 1` ratio. \
                     This is useful for displaying images in a square format.",
                ),
                snippet: "examples/browser/snippets/aspect-ratio-square.md",
            },
            page::Block::Example {
                id: "portrait",
                title: "Portrait",
                description: Some(
                    "A portrait aspect ratio component using a `9 / 16` ratio. \
                     This is useful for displaying images in a portrait format.",
                ),
                snippet: "examples/browser/snippets/aspect-ratio-portrait.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/aspect-ratio-api-reference.md"),
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

/// Nova uses a grayscale crop of the shadcn1 avatar gradient and a darker
/// image in dark mode. Both Aspect Ratio and Card use the same cover.
pub(crate) fn cover<'a>(card: bool) -> iced::Element<'a, Message, Theme> {
    static COVER: OnceLock<image::Handle> = OnceLock::new();
    let handle = COVER.get_or_init(|| {
        image::Handle::from_bytes(include_bytes!("../../assets/shadcn1.jpg").as_slice())
    });
    let radius = if card { 0.0 } else { radius::LG };
    let image = image(handle.clone())
        .width(Fill)
        .height(Fill)
        .content_fit(ContentFit::Cover)
        .border_radius(radius);
    let tint = container("")
        .width(Fill)
        .height(Fill)
        .style(move |t: &Theme| {
            let amount = match (card, t.neutral.is_dark) {
                (true, true) => 0.74,
                (true, false) => 0.61,
                (false, true) => 0.8,
                (false, false) => 0.0,
            };
            Style {
                background: Some(Color::BLACK.alpha(amount).into()),
                border: Border {
                    radius: radius.into(),
                    ..Default::default()
                },
                ..Style::default()
            }
        });
    stack![image, iced::Element::from(tint)].boxed()
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

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        match id {
            // Demo (16:9). shadcn `aspect-ratio-demo.tsx`: max-w-sm
            // (`24rem` = 384px) cap on the AspectRatio's parent. We apply
            // it directly via `aspect_ratio.max_width(...)` — see the
            // widget docs for why a wrapping `container(...).max_width(...)`
            // is the wrong shape (Shrink-width parent triggers
            // width-compression and collapses the child).
            "demo" => aspect_ratio(cover(false), 16.0 / 9.0)
                .max_width(384.0)
                .into(),

            // Portrait (9:16). shadcn `aspect-ratio-portrait.tsx`:
            // max-w-[10rem] = 160px.
            "portrait" => aspect_ratio(cover(false), 9.0 / 16.0)
                .max_width(160.0)
                .into(),

            // Square (1:1). shadcn `aspect-ratio-square.tsx`:
            // max-w-[12rem] = 192px.
            "square" => aspect_ratio(cover(false), 1.0).max_width(192.0).into(),

            _ => unreachable!("unknown aspect_ratio preview id: {id}"),
        }
    }
}
