// Intentional skip per AGENTS.md: checkbox-rtl (RTL).
mod selection;

use iced::{Fill, Pixels, Shrink, Task};

use icedcn::Theme;
use icedcn::theme;
use icedcn::theme::alpha;
use icedcn::widget::{View, checkbox, column, container, field, text};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Checkbox",
    description: "A control that allows the user to toggle between checked and not checked.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/checkbox-demo.md",
    },
    page::Block::Md("examples/browser/snippets/checkbox-usage.md"),
    page::Block::Example {
        id: "checked-state",
        title: "Checked State",
        description: Some(
            "Set the initial value with `.default_checked(...)`. To keep the value in your application, use `.checked(...)` and `.on_checked_change(...)`.",
        ),
        snippet: "examples/browser/snippets/checkbox-checked-state.md",
    },
    page::Block::Example {
        id: "invalid-state",
        title: "Invalid State",
        description: Some(
            "Apply `.style(theme::checkbox::Invalid)` to the checkbox and use a destructive text style for the label.",
        ),
        snippet: "examples/browser/snippets/checkbox-invalid-state.md",
    },
    page::Block::Example {
        id: "basic",
        title: "Basic",
        description: Some(
            "Give the checkbox an associated label with `field(checkbox()).label(...)`.",
        ),
        snippet: "examples/browser/snippets/checkbox-basic.md",
    },
    page::Block::Example {
        id: "description",
        title: "Description",
        description: Some("Add helper text below the label with `.description(...)`."),
        snippet: "examples/browser/snippets/checkbox-description.md",
    },
    page::Block::Example {
        id: "disabled",
        title: "Disabled",
        description: Some(
            "Set `.disabled(true)` on the field to disable the control and label together.",
        ),
        snippet: "examples/browser/snippets/checkbox-disabled.md",
    },
    page::Block::Example {
        id: "group",
        title: "Group",
        description: Some("Use multiple fields to create a checkbox list."),
        snippet: "examples/browser/snippets/checkbox-group.md",
    },
    page::Block::Example {
        id: "table",
        title: "Table",
        description: None,
        snippet: "examples/browser/snippets/checkbox-table.md",
    },
    page::Block::Md("examples/browser/snippets/checkbox-api-reference.md"),
];

pub struct CheckboxPage {
    state: page::State,
    wide: bool,
}

impl CheckboxPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
            wide: true,
        })
    }
}

impl page::Page for CheckboxPage {
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
        self.wide = width >= 768.0;
    }

    fn backdrop(&self) -> page::Backdrop {
        page::Backdrop {
            padding: 0.into(),
            ..page::Backdrop::default()
        }
    }

    fn docs_preview(&self, id: &str, theme: &Theme) -> iced::Element<'_, Message, Theme> {
        let padding = if id == "table" {
            if self.wide { 32 } else { 16 }
        } else {
            40
        };
        container(self.themed_preview(id, theme))
            .center_x(Fill)
            .center_y(if id == "demo" { 320 } else { 288 })
            .padding([0, padding])
            .into()
    }

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        match id {
            "demo" => column![
                field(checkbox())
                    .orientation(field::Horizontal)
                    .label(View::from(
                        container(text::label("Accept terms and conditions")).height(14)
                    )),
                field(container(checkbox().default_checked(true)).padding([1, 0, 0, 0]))
                    .orientation(field::Horizontal)
                    .label(text::label("Accept terms and conditions").line_height(Pixels(19.25)))
                    .description(
                        text::muted("By clicking this checkbox, you agree to the terms.")
                            .line_height(Pixels(21.0))
                    ),
                field(checkbox().disabled(true))
                    .orientation(field::Horizontal)
                    .disabled(true)
                    .label(
                        text::label("Enable notifications")
                            .line_height(Pixels(19.25))
                            .style(theme::text::Foreground + alpha(0.5))
                    ),
                field(container(checkbox()).padding([1, 0, 0, 0]))
                    .orientation(field::Horizontal)
                    .label(text::label("Enable notifications").line_height(Pixels(20.0)))
                    .description(
                        text::muted("You can enable or disable notifications at any time.")
                            .line_height(Pixels(21.0))
                    )
                    .card(),
            ]
            .spacing(20)
            .width(Fill)
            .max_width(384.0)
            .into(),
            "checked-state" => field(checkbox().default_checked(true))
                .orientation(field::Horizontal)
                .label(text::label("Accept terms and conditions").line_height(Pixels(19.25)))
                .width(224)
                .into(),
            "invalid-state" => field(checkbox().style(theme::checkbox::Invalid))
                .orientation(field::Horizontal)
                .label(
                    text::label("Accept terms and conditions")
                        .line_height(Pixels(19.25))
                        .style(theme::text::Destructive),
                )
                .width(224)
                .into(),
            "basic" => field(checkbox())
                .orientation(field::Horizontal)
                .label(text::label("Accept terms and conditions").line_height(Pixels(19.25)))
                .width(224)
                .into(),
            "description" => {
                field(container(checkbox().default_checked(true)).padding([1, 0, 0, 0]))
                    .orientation(field::Horizontal)
                    .label(text::label("Accept terms and conditions").line_height(Pixels(19.25)))
                    .description(
                        text::muted(
                            "By clicking this checkbox, you agree to the terms and conditions.",
                        )
                        .line_height(Pixels(21.0)),
                    )
                    .width(288)
                    .into()
            }
            "disabled" => field(checkbox().disabled(true))
                .orientation(field::Horizontal)
                .disabled(true)
                .label(
                    text::label("Enable notifications")
                        .line_height(Pixels(19.25))
                        .style(theme::text::Foreground + alpha(0.5)),
                )
                .width(224)
                .into(),
            "group" => {
                let choices = [
                    "Hard disks",
                    "External disks",
                    "CDs, DVDs, and iPods",
                    "Connected servers",
                ];
                field::set(
                    column![
                        text::label("Show these items on the desktop:").line_height(Pixels(20.0)),
                        text::muted("Select the items you want to show on the desktop.")
                            .line_height(Pixels(21.0)),
                    ]
                    .spacing(2),
                    None,
                    field::group(choices.into_iter().enumerate().map(|(index, label)| {
                        field(checkbox().default_checked(index < 2))
                            .orientation(field::Horizontal)
                            .label(text::small(label).line_height(Pixels(19.25)))
                            .into()
                    }))
                    .spacing(12),
                )
                .width(Shrink)
                .into()
            }
            "table" => selection::preview(),
            _ => unreachable!("unknown checkbox preview id: {id}"),
        }
    }
}
