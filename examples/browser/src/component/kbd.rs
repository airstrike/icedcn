use iced::Task;

use crate::icon;
use icedxdy::Theme;
use icedxdy::widget::button::group;
use icedxdy::widget::input_group::Align;
use icedxdy::widget::{
    button, button_group, column, container, input, input_group, kbd, kbd_group, row, text, tooltip,
};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Kbd",
    description: "Used to display textual user input from keyboard.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/kbd-demo.md",
    },
    page::Block::Md("examples/browser/snippets/kbd-composition.md"),
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "group",
                title: "Group",
                description: Some("Use `kbd_group!` to group keyboard keys together."),
                snippet: "examples/browser/snippets/kbd-group.md",
            },
            page::Block::Example {
                id: "button",
                title: "Button",
                description: Some("Use a `kbd` inside a `button` to display a keyboard key."),
                snippet: "examples/browser/snippets/kbd-button.md",
            },
            page::Block::Example {
                id: "tooltip",
                title: "Tooltip",
                description: Some("Show a keyboard shortcut inside a tooltip."),
                snippet: "examples/browser/snippets/kbd-tooltip.md",
            },
            page::Block::Example {
                id: "input-group",
                title: "Input Group",
                description: Some("Add keyboard keys to an input group addon."),
                snippet: "examples/browser/snippets/kbd-input-group.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/kbd-api-reference.md"),
];

pub struct KbdPage {
    state: page::State,
}

impl KbdPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

impl page::Page for KbdPage {
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
            "demo" => column![
                kbd_group![kbd("⌘"), kbd("⇧"), kbd("⌥"), kbd("⌃")],
                kbd_group![kbd("Ctrl"), text::small("+"), kbd("B")],
            ]
            .spacing(16)
            .align_x(iced::Center)
            .into(),

            "group" => row![
                text::muted("Use"),
                kbd_group![kbd("Ctrl + B"), kbd("Ctrl + K")],
                text::muted("to open the command palette"),
            ]
            .spacing(4)
            .align_y(iced::Center)
            .into(),

            "button" => button!["Accept", kbd("⏎")].style(button::Outline).into(),

            "tooltip" => button_group![
                group::decorated(button("Save").style(button::Outline), |button| tooltip(
                    button,
                    row![text::xs("Save Changes"), kbd("S").style(kbd::Tooltip)].spacing(4)
                )
                .into()),
                group::decorated(button("Print").style(button::Outline), |button| tooltip(
                    button,
                    row![
                        text::xs("Print Document"),
                        kbd_group![
                            kbd("Ctrl").style(kbd::Tooltip),
                            kbd("P").style(kbd::Tooltip)
                        ]
                    ]
                    .spacing(4)
                )
                .into())
            ]
            .into(),

            "input-group" => container(
                input_group(input("Search..."))
                    .addon(input_group::addon(icon::search()))
                    .addon(
                        input_group::addon(kbd_group![kbd("⌘"), kbd("K")]).align(Align::InlineEnd),
                    ),
            )
            .max_width(320)
            .width(iced::Fill)
            .into(),

            _ => unreachable!("unknown kbd preview id: {id}"),
        }
    }
}
