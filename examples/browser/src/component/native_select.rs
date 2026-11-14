use iced::Task;

use icedxdy::Theme;
use icedxdy::widget::native_select;

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Native Select",
    description: "A styled iced pick list for choosing one option.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/native-select-demo.md",
    },
    page::Block::Md("examples/browser/snippets/native-select-composition.md"),
    page::Block::Md("examples/browser/snippets/native-select-guide-native-select-vs-select.md"),
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "groups",
                title: "Groups",
                description: Some(
                    "Use `native_select::group(label, items)` to organize options into categories.",
                ),
                snippet: "examples/browser/snippets/native-select-groups.md",
            },
            page::Block::Example {
                id: "disabled",
                title: "Disabled",
                description: Some("Use `.disabled(true)` to disable the select."),
                snippet: "examples/browser/snippets/native-select-disabled.md",
            },
            page::Block::Example {
                id: "invalid",
                title: "Invalid",
                description: Some("Use `.style(native_select::Invalid)` to show an error state."),
                snippet: "examples/browser/snippets/native-select-invalid.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/native-select-api-reference.md"),
];

pub struct NativeSelectPage {
    state: page::State,
}

impl NativeSelectPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

/// A prompt option followed by three fruits.
fn fruits(prompt: &str) -> [native_select::Item; 4] {
    [
        native_select::item("", prompt),
        native_select::item("apple", "Apple"),
        native_select::item("banana", "Banana"),
        native_select::item("blueberry", "Blueberry"),
    ]
}

impl page::Page for NativeSelectPage {
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
            // shadcn `native-select-demo.tsx`.
            "demo" => native_select([
                native_select::item("", "Select status"),
                native_select::item("todo", "Todo"),
                native_select::item("in-progress", "In Progress"),
                native_select::item("done", "Done"),
                native_select::item("cancelled", "Cancelled"),
            ])
            .into(),

            "groups" => native_select([
                native_select::item("", "Select department"),
                native_select::group(
                    "Engineering",
                    [
                        native_select::item("frontend", "Frontend"),
                        native_select::item("backend", "Backend"),
                        native_select::item("devops", "DevOps"),
                    ],
                ),
                native_select::group(
                    "Sales",
                    [
                        native_select::item("sales-rep", "Sales Rep"),
                        native_select::item("account-manager", "Account Manager"),
                        native_select::item("sales-director", "Sales Director"),
                    ],
                ),
                native_select::group(
                    "Operations",
                    [
                        native_select::item("support", "Customer Support"),
                        native_select::item("product-manager", "Product Manager"),
                        native_select::item("ops-manager", "Operations Manager"),
                    ],
                ),
            ])
            .into(),

            // shadcn `native-select-disabled.tsx`.
            "disabled" => native_select(fruits("Disabled")).disabled(true).into(),

            // shadcn `native-select-invalid.tsx`.
            "invalid" => native_select(fruits("Error state"))
                .style(native_select::Invalid)
                .into(),

            _ => unreachable!("unknown native select preview id: {id}"),
        }
    }
}
