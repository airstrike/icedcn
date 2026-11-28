use iced::{Fill, Task};
use icedcn::theme;

use icedcn::widget::{button, column, field, input, row, scrollable, sheet, text};
use icedcn::{Theme, View};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Sheet",
    description: "Extends the Dialog component to display content that complements the main \
                  content of the screen.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/sheet-demo.md",
    },
    page::Block::Md("examples/browser/snippets/sheet-usage.md"),
    page::Block::Md("examples/browser/snippets/sheet-composition.md"),
    page::Block::Example {
        id: "side",
        title: "Side",
        description: Some("Set the edge with `.side(..)`. Choose top, right, bottom or left."),
        snippet: "examples/browser/snippets/sheet-side.md",
    },
    page::Block::Example {
        id: "no-close-button",
        title: "No Close Button",
        description: Some("Use `.show_close_button(false)` to hide the corner close button."),
        snippet: "examples/browser/snippets/sheet-no-close-button.md",
    },
    page::Block::Md("examples/browser/snippets/sheet-api-reference.md"),
];

/// The sheet page. Every sheet is uncontrolled.
pub struct SheetPage {
    state: page::State,
    input_size: f32,
}

impl SheetPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
            input_size: 14.,
        })
    }
}

const DESCRIPTION: &str = "Make changes to your profile here. Click save when you're done.";
const PARAGRAPH: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

fn profile(input_size: f32) -> View<'static, Message> {
    sheet(
        |s| s.trigger(button("Open").style(button::Outline)),
        move |s| {
            let entry = |label, value| {
                let control = input("")
                    .default_value(value)
                    .text_size(input_size)
                    .width(Fill);
                field(control)
                    .spacing(12)
                    .label(text::label(label).height(14))
                    .width(Fill)
                    .into()
            };
            sheet::content(
                sheet::header("Edit profile", DESCRIPTION),
                field::group([
                    entry("Name", "Pedro Duarte"),
                    entry("Username", "@peduarte"),
                ])
                .spacing(24)
                .width(Fill),
                [
                    button("Save changes").into(),
                    s.close(button("Close").style(button::Outline)),
                ],
            )
        },
    )
    .into()
}

fn side<'a>(edge: sheet::Side, label: &'a str) -> View<'a, Message> {
    sheet(
        move |s| s.trigger(button(label).style(button::Outline)),
        |s| {
            let paragraphs =
                column((0..10).map(|_| text::small(PARAGRAPH).line_height(1.625).into()))
                    .spacing(8)
                    .padding([0, 0, 8, 0])
                    .width(Fill);
            sheet::content(
                sheet::header("Edit profile", DESCRIPTION),
                scrollable(paragraphs)
                    .height(Fill)
                    .width(Fill)
                    .class(theme::scrollable::Hidden),
                [
                    button("Save changes").into(),
                    s.close(button("Cancel").style(button::Outline)),
                ],
            )
        },
    )
    .side(edge)
    .max_height_ratio(0.5)
    .into()
}

impl page::Page for SheetPage {
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

    fn window_width(&mut self, width: f32) {
        self.input_size = if width < 768. { 16. } else { 14. };
    }

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        match id {
            "demo" => profile(self.input_size).into(),
            "side" => row![
                side(sheet::Side::Top,"Top"), side(sheet::Side::Right,"Right"),
                side(sheet::Side::Bottom,"Bottom"), side(sheet::Side::Left,"Left"),
            ].wrap().spacing(8).into(),
            "no-close-button" => sheet(
                |s| s.trigger(button("Open Sheet").style(button::Outline)),
                |_| sheet::header(
                    "No Close Button",
                    "This sheet doesn't have a close button in the top-right corner. Click outside to close.",
                ),
            ).show_close_button(false).into(),
            _ => unreachable!("unknown sheet preview id: {id}"),
        }
    }
}
