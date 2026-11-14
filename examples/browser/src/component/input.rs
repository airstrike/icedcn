mod forms;

use iced::Fill;
use iced::Task;

use crate::icon;
use icedcn::Theme;
use icedcn::rich_text;
use icedcn::theme;
use icedcn::theme::tweak::Alpha;
use icedcn::theme::tweak::Token;
use icedcn::widget::rich_text::span;
use icedcn::widget::{Slot, badge, button, field, input, row, space, text};
use icedcn::widget::{button_group, input_group};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Input",
    description: "A text input component for forms and user data entry with built-in styling and accessibility features.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/input-demo.md",
    },
    page::Block::Md("examples/browser/snippets/input-usage.md"),
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "basic",
                title: "Basic",
                description: Some("A simple input with a placeholder."),
                snippet: "examples/browser/snippets/input-basic.md",
            },
            page::Block::Example {
                id: "field",
                title: "Field",
                description: Some("An input with a label and description."),
                snippet: "examples/browser/snippets/input-field.md",
            },
            page::Block::Example {
                id: "fieldgroup",
                title: "Field Group",
                description: None,
                snippet: "examples/browser/snippets/input-fieldgroup.md",
            },
            page::Block::Example {
                id: "disabled",
                title: "Disabled",
                description: Some("A disabled input field."),
                snippet: "examples/browser/snippets/input-disabled.md",
            },
            page::Block::Example {
                id: "invalid",
                title: "Invalid",
                description: Some(
                    "An input with validation errors, styled with a destructive border and focus ring.",
                ),
                snippet: "examples/browser/snippets/input-invalid.md",
            },
            page::Block::Example {
                id: "file",
                title: "File",
                description: None,
                snippet: "examples/browser/snippets/input-file.md",
            },
            page::Block::Example {
                id: "inline",
                title: "Inline",
                description: Some("Use an input with a button to create a search input."),
                snippet: "examples/browser/snippets/input-inline.md",
            },
            page::Block::Example {
                id: "grid",
                title: "Grid",
                description: None,
                snippet: "examples/browser/snippets/input-grid.md",
            },
            page::Block::Example {
                id: "required",
                title: "Required",
                description: Some(
                    "An input with a required label, marked by a destructive asterisk suffix.",
                ),
                snippet: "examples/browser/snippets/input-required.md",
            },
            page::Block::Example {
                id: "badge",
                title: "Badge",
                description: Some(
                    "A field label with a trailing badge to flag the input's status.",
                ),
                snippet: "examples/browser/snippets/input-badge.md",
            },
            page::Block::Example {
                id: "input-group",
                title: "Input Group",
                description: None,
                snippet: "examples/browser/snippets/input-input-group.md",
            },
            page::Block::Example {
                id: "button-group",
                title: "Button Group",
                description: None,
                snippet: "examples/browser/snippets/input-button-group.md",
            },
            page::Block::Example {
                id: "form",
                title: "Form",
                description: None,
                snippet: "examples/browser/snippets/input-form.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/input-api-reference.md"),
];

pub struct InputPage {
    state: page::State,
    file_name: Option<String>,
}

impl InputPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
            file_name: None,
        })
    }
}

impl page::Page for InputPage {
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
        if let page::Message::FileSelected(Some(name)) = &event {
            self.file_name = Some(name.clone());
        }
        self.state.update(&event)
    }

    fn reset_volatile(&mut self) {
        self.state.reset_volatile();
    }

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        let pi = self.state.page_index;
        match id {
            "demo" => field(input("sk-...").secure(true).width(Fill))
                .label("API Key")
                .description("Your API key is encrypted and stored securely.")
                .width(320)
                .into(),

            "basic" => input("Enter text").width(320).into(),

            // Field: input with label "Username" and description.
            "field" => field(input("Enter your username").width(Fill))
                .label("Username")
                .description("Choose a unique username for your account.")
                .width(320)
                .into(),

            "disabled" => field(input("Email").width(Fill).disabled(true))
                .label(text::label("Email").style(theme::text::Foreground + Alpha(0.5)))
                .description("This field is currently disabled.")
                .width(320)
                .into(),

            "invalid" => field(input("Error").width(Fill).style(theme::text_input::Invalid))
                .label(text::label("Invalid Input").style(theme::text::Destructive))
                .description("This field contains validation errors.")
                .width(320)
                .into(),

            // Inline: input + button side by side.
            "inline" => row![
                input("Search...").width(Fill),
                button("Search").on_press(Message::Page(pi, page::Message::Noop)),
            ]
            .spacing(8)
            .width(400)
            .into(),

            // Required: rich-text label with a destructive asterisk.
            "required" => field(input("This field is required").width(Fill))
                .label(Slot::from(rich_text![
                    span("Required Field ").size(text::size::SM).medium(),
                    span("*").size(text::size::SM).color(Token::Destructive),
                ]))
                .description("This field must be filled out.")
                .width(320)
                .into(),

            "badge" => field(input("https://api.example.com/webhook").width(Fill))
                .label(Slot::from(
                    row![
                        text::label("Webhook URL"),
                        space::horizontal(),
                        badge("Beta").style(badge::Secondary),
                    ]
                    .width(Fill),
                ))
                .width(320)
                .into(),

            "file" => field(
                input_group(
                    input("")
                        .value(self.file_name.as_deref().unwrap_or("No file chosen"))
                        .width(Fill),
                )
                .addon(input_group::addon(
                    input_group::button("Choose File")
                        .on_press(Message::Page(pi, page::Message::ChooseFile)),
                )),
            )
            .label("Picture")
            .description("Select a picture to upload.")
            .width(320)
            .into(),
            "grid" => row![
                field(input("Jordan")).label("First Name").width(Fill),
                field(input("Lee")).label("Last Name").width(Fill)
            ]
            .spacing(16)
            .width(384)
            .into(),
            "input-group" => field(
                input_group(input("example.com"))
                    .addon(input_group::addon(input_group::text("https://")))
                    .addon(input_group::addon(icon::info()).align(input_group::Align::InlineEnd)),
            )
            .label("Website URL")
            .width(320)
            .into(),
            "button-group" => field(
                button_group![
                    input("Type to search...").width(Fill),
                    button("Search").style(button::Outline)
                ]
                .width(Fill),
            )
            .label("Search")
            .width(320)
            .into(),
            "fieldgroup" => forms::preview(false).into(),
            "form" => forms::preview(true).into(),
            _ => unreachable!("unknown input preview id: {id}"),
        }
    }
}
