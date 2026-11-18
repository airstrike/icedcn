mod forms;

use iced::Fill;
use iced::Task;

use crate::icon;
use icedcn::Theme;
use icedcn::theme;
use icedcn::theme::tweak::Alpha;
use icedcn::widget::input_group;
use icedcn::widget::{Slot, badge, button, container, field, input, row, space, text};

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
    page::Block::Example {
        id: "basic",
        title: "Basic",
        description: None,
        snippet: "examples/browser/snippets/input-basic.md",
    },
    page::Block::Example {
        id: "field",
        title: "Field",
        description: Some(
            "Give the input a clickable label with `.label(...)` and add help text with `.description(...)`.",
        ),
        snippet: "examples/browser/snippets/input-field.md",
    },
    page::Block::Example {
        id: "fieldgroup",
        title: "Field Group",
        description: Some("Group related fields with `field::group`."),
        snippet: "examples/browser/snippets/input-fieldgroup.md",
    },
    page::Block::Example {
        id: "disabled",
        title: "Disabled",
        description: Some("Set `.disabled(true)` on the field to disable its input."),
        snippet: "examples/browser/snippets/input-disabled.md",
    },
    page::Block::Example {
        id: "invalid",
        title: "Invalid",
        description: Some(
            "Apply `theme::text_input::Invalid` to the input and the destructive text style to its label.",
        ),
        snippet: "examples/browser/snippets/input-invalid.md",
    },
    page::Block::Example {
        id: "file",
        title: "File",
        description: Some(
            "Open a file picker from the Choose File action and display the selected filename.",
        ),
        snippet: "examples/browser/snippets/input-file.md",
    },
    page::Block::Example {
        id: "inline",
        title: "Inline",
        description: Some("Place an input and a search button in the same row."),
        snippet: "examples/browser/snippets/input-inline.md",
    },
    page::Block::Example {
        id: "grid",
        title: "Grid",
        description: Some("Place two equal-width fields side by side."),
        snippet: "examples/browser/snippets/input-grid.md",
    },
    page::Block::Example {
        id: "required",
        title: "Required",
        description: Some(
            "Mark a required field with an asterisk and validate its value on submission.",
        ),
        snippet: "examples/browser/snippets/input-required.md",
    },
    page::Block::Example {
        id: "badge",
        title: "Badge",
        description: Some("Add a badge to the label to highlight the field’s status."),
        snippet: "examples/browser/snippets/input-badge.md",
    },
    page::Block::Example {
        id: "input-group",
        title: "Input Group",
        description: Some(
            "Use [Input Group](icedcn://component/input-group) to place text and icons inside the input border.",
        ),
        snippet: "examples/browser/snippets/input-input-group.md",
    },
    page::Block::Example {
        id: "button-group",
        title: "Button Group",
        description: Some(
            "Use [Button Group](icedcn://component/button-group) to join an input and its action.",
        ),
        snippet: "examples/browser/snippets/input-button-group.md",
    },
    page::Block::Example {
        id: "form",
        title: "Form",
        description: Some(
            "Combine text inputs, a country selector, and submit validation in one form.",
        ),
        snippet: "examples/browser/snippets/input-form.md",
    },
    page::Block::Md("examples/browser/snippets/input-api-reference.md"),
];

pub struct InputPage {
    state: page::State,
    file_name: Option<String>,
    text_size: f32,
}

impl InputPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
            file_name: None,
            text_size: 14.,
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

    fn window_width(&mut self, width: f32) {
        self.text_size = if width < 768. { 16. } else { 14. };
    }

    fn backdrop(&self) -> page::Backdrop {
        page::Backdrop {
            padding: 0.into(),
            ..Default::default()
        }
    }

    fn docs_preview(&self, id: &str, theme: &Theme) -> iced::Element<'_, Message, Theme> {
        let height = if id == "form" { 512 } else { 288 };
        let padding = if id == "grid" { 24 } else { 40 };
        container(self.themed_preview(id, theme))
            .center_x(Fill)
            .center_y(height)
            .padding([0, padding])
            .into()
    }

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        let input = |placeholder| input(placeholder).text_size(self.text_size);
        let pi = self.state.page_index;
        match id {
            "demo" => field(input("sk-...").secure(true).width(Fill))
                .label("API Key")
                .description("Your API key is encrypted and stored securely.")
                .width(320)
                .into(),

            "basic" => input("Enter text").width(320).into(),

            "field" => field(input("Enter your username").width(Fill))
                .label("Username")
                .description("Choose a unique username for your account.")
                .width(320)
                .into(),

            "disabled" => field(input("Email").width(Fill))
                .label(field::label("Email").style(theme::text::Foreground + Alpha(0.5)))
                .description("This field is currently disabled.")
                .disabled(true)
                .width(320)
                .into(),

            "invalid" => field(input("Error").width(Fill).style(theme::text_input::Invalid))
                .label(field::label("Invalid Input").style(theme::text::Destructive))
                .description("This field contains validation errors.")
                .width(320)
                .into(),

            "inline" => row![input("Search...").width(Fill), button("Search"),]
                .spacing(8)
                .width(320)
                .into(),

            "required" => field(input("This field is required").width(Fill))
                .label(Slot::from(
                    row![
                        field::label("Required Field"),
                        field::label("*").style(theme::text::Destructive),
                    ]
                    .spacing(8),
                ))
                .description("This field must be filled out.")
                .width(320)
                .into(),

            "badge" => field(input("https://api.example.com/webhook").width(Fill))
                .label(Slot::from(
                    row![
                        field::label("Webhook URL"),
                        space::horizontal(),
                        badge("Beta").style(badge::Secondary),
                    ]
                    .align_y(iced::Center)
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
                    input_group::button(text::small("Choose File").medium())
                        .padding([0, 3])
                        .on_press(Message::Page(pi, page::Message::ChooseFile)),
                )),
            )
            .label("Picture")
            .description("Select a picture to upload.")
            .width(320)
            .into(),
            "grid" => row![
                field(input("Jordan").width(Fill))
                    .label("First Name")
                    .width(Fill),
                field(input("Lee").width(Fill))
                    .label("Last Name")
                    .width(Fill)
            ]
            .spacing(20)
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
                button::group![
                    input("Type to search...").width(Fill),
                    button("Search").style(button::Outline)
                ]
                .width(Fill),
            )
            .label("Search")
            .width(320)
            .into(),
            "fieldgroup" => forms::group(self.text_size),
            "form" => forms::form(self.text_size),
            _ => unreachable!("unknown input preview id: {id}"),
        }
    }
}
