mod article;

use iced::{Fill, Pixels, Task, Widget};
use icedcn::widget::{column, container, rule, text};
use icedcn::{Theme, View, Weight};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Typography",
    description: "Styles for headings, paragraphs, lists, and inline code.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Md("examples/browser/snippets/text-introduction.md"),
    // Article hero: empty-title example so the Joke Tax article gets the
    // same card chrome every other page's hero uses. `render_example`
    // applies `min_height(PREVIEW_HEIGHT)` (not fixed height) for
    // empty-title cards, so the long article grows past 288px naturally
    // instead of clipping.
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/text-demo.md",
    },
    page::Block::Md("examples/browser/snippets/text-usage.md"),
    page::Block::Example {
        id: "h1",
        title: "h1",
        description: None,
        snippet: "examples/browser/snippets/text-h1.md",
    },
    page::Block::Example {
        id: "h2",
        title: "h2",
        description: None,
        snippet: "examples/browser/snippets/text-h2.md",
    },
    page::Block::Example {
        id: "h3",
        title: "h3",
        description: None,
        snippet: "examples/browser/snippets/text-h3.md",
    },
    page::Block::Example {
        id: "h4",
        title: "h4",
        description: None,
        snippet: "examples/browser/snippets/text-h4.md",
    },
    page::Block::Example {
        id: "p",
        title: "p",
        description: None,
        snippet: "examples/browser/snippets/text-p.md",
    },
    page::Block::Example {
        id: "blockquote",
        title: "blockquote",
        description: None,
        snippet: "examples/browser/snippets/text-blockquote.md",
    },
    page::Block::Example {
        id: "table",
        title: "table",
        description: None,
        snippet: "examples/browser/snippets/text-table.md",
    },
    page::Block::Example {
        id: "list",
        title: "list",
        description: None,
        snippet: "examples/browser/snippets/text-list.md",
    },
    page::Block::Example {
        id: "inline-code",
        title: "Inline code",
        description: None,
        snippet: "examples/browser/snippets/text-inline-code.md",
    },
    page::Block::Example {
        id: "lead",
        title: "Lead",
        description: None,
        snippet: "examples/browser/snippets/text-lead.md",
    },
    page::Block::Example {
        id: "large",
        title: "Large",
        description: None,
        snippet: "examples/browser/snippets/text-large.md",
    },
    page::Block::Example {
        id: "small",
        title: "Small",
        description: None,
        snippet: "examples/browser/snippets/text-small.md",
    },
    page::Block::Example {
        id: "muted",
        title: "Muted",
        description: None,
        snippet: "examples/browser/snippets/text-muted.md",
    },
    page::Block::Md("examples/browser/snippets/text-api-reference.md"),
];

pub struct TextPage {
    state: page::State,
}

impl TextPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

impl page::Page for TextPage {
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
            "demo" => article::view(self.page_index()),
            "h1" => h1("Taxing Laughter: The Joke Tax Chronicles")
                .width(Fill)
                .align_x(iced::Center)
                .into(),

            "h2" => h2_with_border("The People of the Kingdom").into(),

            "h3" => h3("The Joke Tax").into(),
            "h4" => text::h4("People stopped telling jokes")
                .size(20)
                .semibold()
                .line_height(Pixels(28.))
                .into(),

            "p" => paragraph(
                "The king, seeing how much happier his subjects were, realized the \
                 error of his ways and repealed the joke tax.",
            )
            .into(),

            "blockquote" => text::blockquote(
                "\"After all,\" he said, \"everyone enjoys a good joke, so it's only \
                 fair that they should pay for the privilege.\"",
            )
            .into(),

            "table" => table_card().into(),

            "list" => text::list([
                "1st level of puns: 5 gold coins",
                "2nd level of jokes: 10 gold coins",
                "3rd level of one-liners : 20 gold coins",
            ])
            .into(),

            "inline-code" => text::inline_code("widget::alert_dialog::AlertDialog")
                .font(crate::font::GEIST_MONO_NAME)
                .into(),

            "lead" => text::lead(
                "A modal dialog that interrupts the user with important content and \
                 expects a response.",
            )
            .line_height(Pixels(28.))
            .into(),

            "large" => text::large("Are you absolutely sure?")
                .line_height(Pixels(28.))
                .into(),
            "small" => container(text::label("Email address")).center_y(24).into(),
            "muted" => text::muted("Enter your email address.")
                .line_height(Pixels(20.))
                .into(),

            _ => unreachable!("unknown text preview id: {id}"),
        }
    }
}

fn h1(content: &str) -> text::Text<'_> {
    text::h1(content)
        .size(36)
        .weight(Weight::ExtraBold)
        .line_height(Pixels(40.))
        .wrapping(text::Wrapping::Balanced)
}

fn h3(content: &str) -> text::Text<'_> {
    text::h3(content)
        .size(24)
        .semibold()
        .line_height(Pixels(32.))
}

fn paragraph(content: &str) -> text::Text<'_> {
    text::p(content).line_height(Pixels(28.)).width(Fill)
}

fn h2_with_border<'a, M: 'a>(content: &'a str) -> View<'a, M> {
    column![
        text::h2(content)
            .size(30)
            .semibold()
            .line_height(Pixels(36.)),
        rule::horizontal(1).boxed()
    ]
    .width(Fill)
    .spacing(8)
    .into()
}

fn table_card<'a, M: 'a>() -> View<'a, M> {
    text::table(
        ["King's Treasury", "People's happiness"],
        [
            ["Empty", "Overflowing"],
            ["Modest", "Satisfied"],
            ["Full", "Ecstatic"],
        ],
    )
    .into()
}
