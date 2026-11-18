use iced::{Shrink, Task};

use icedcn::theme;
use icedcn::theme::tweak::Alpha;
use icedcn::widget::{field, radio_group, text};
use icedcn::{Horizontal, Slot, Theme};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Radio Group",
    description: "A set of checkable buttons, known as radio buttons, where no more than one of the buttons can be checked at a time.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/radio-group-demo.md",
    },
    page::Block::Md("examples/browser/snippets/radio-group-usage.md"),
    page::Block::Md("examples/browser/snippets/radio-group-composition.md"),
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "description",
                title: "Description",
                description: Some(
                    "Radio group items with a description using the `field` component.",
                ),
                snippet: "examples/browser/snippets/radio-group-description.md",
            },
            page::Block::Example {
                id: "choice-card",
                title: "Choice Card",
                description: Some("Use card-style labels to choose a plan."),
                snippet: "examples/browser/snippets/radio-group-choice-card.md",
            },
            page::Block::Example {
                id: "fieldset",
                title: "Fieldset",
                description: Some("Group related radio buttons with a legend and description."),
                snippet: "examples/browser/snippets/radio-group-fieldset.md",
            },
            page::Block::Example {
                id: "disabled",
                title: "Disabled",
                description: Some("Use `.disabled(true)` on an item (or the group) to disable it."),
                snippet: "examples/browser/snippets/radio-group-disabled.md",
            },
            page::Block::Example {
                id: "invalid",
                title: "Invalid",
                description: Some("Use an invalid field to indicate a validation error."),
                snippet: "examples/browser/snippets/radio-group-invalid.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/radio-group-api-reference.md"),
];

pub struct RadioGroupPage {
    state: page::State,
}

impl RadioGroupPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

impl page::Page for RadioGroupPage {
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
            padding: 40.into(),
            ..Default::default()
        }
    }

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        let view = match id {
            "demo" => density(),
            "description" => described(),
            "choice-card" => plans(),
            "fieldset" => super::field::subscription_plan(),
            "disabled" => availability(),
            "invalid" => notifications(),
            _ => unreachable!("unknown radio group preview id: {id}"),
        };
        view.into()
    }
}

fn density<'a, M: Clone + 'static>() -> Slot<'a, M> {
    radio_group(|g| {
        field::group(["Default", "Comfortable", "Compact"].map(|name| {
            field(g.item(name))
                .orientation(Horizontal)
                .label(text::label(name))
                .into()
        }))
        .spacing(8)
        .width(Shrink)
        .into()
    })
    .default_value("Comfortable")
    .into()
}

fn described<'a, M: Clone + 'static>() -> Slot<'a, M> {
    let choices = [
        ("Default", "Standard spacing for most use cases."),
        ("Comfortable", "More space between elements."),
        ("Compact", "Minimal spacing for dense layouts."),
    ];
    radio_group(move |g| {
        field::group(choices.map(|(name, detail)| {
            field(g.item(name))
                .orientation(Horizontal)
                .label(name)
                .description(detail)
                .into()
        }))
        .spacing(8)
        .width(Shrink)
        .into()
    })
    .default_value("Comfortable")
    .into()
}

fn plans<'a, M: Clone + 'static>() -> Slot<'a, M> {
    let plans = [
        ("Plus", "For individuals and small teams."),
        ("Pro", "For growing businesses."),
        ("Enterprise", "For large teams and enterprises."),
    ];
    radio_group(move |g| {
        field::group(plans.map(|(name, detail)| {
            field(g.item(name))
                .orientation(Horizontal)
                .control_last()
                .label(name)
                .description(detail)
                .card()
                .into()
        }))
        .spacing(8)
        .width(384)
        .into()
    })
    .default_value("Plus")
    .into()
}

fn availability<'a, M: Clone + 'static>() -> Slot<'a, M> {
    radio_group(|g| {
        field::group(["Disabled", "Option 2", "Option 3"].map(|name| {
            let disabled = name == "Disabled";
            let label = text::small(name)
                .style(theme::text::Foreground + Alpha(if disabled { 0.5 } else { 1. }));
            field(g.item(name))
                .orientation(Horizontal)
                .label(label)
                .disabled(disabled)
                .into()
        }))
        .spacing(8)
        .width(Shrink)
        .into()
    })
    .default_value("Option 2")
    .into()
}

fn notifications<'a, M: Clone + 'static>() -> Slot<'a, M> {
    let choices = radio_group(|g| {
        field::group(["Email only", "SMS only", "Both Email & SMS"].map(|name| {
            field(g.item(name).invalid(true))
                .orientation(Horizontal)
                .label(text::small(name).style(theme::text::Destructive))
                .into()
        }))
        .spacing(8)
        .into()
    })
    .default_value("Email only");
    field::set(
        field::label("Notification Preferences"),
        Some("Choose how you want to receive notifications."),
        choices,
    )
    .max_width(320.)
    .into()
}
