use iced::Task;

use icedcn::Horizontal;
use icedcn::Theme;
use icedcn::theme::alpha;
use icedcn::theme::text::{Destructive, Foreground};
use icedcn::widget::{container, field, switch};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Switch",
    description: "A control that allows the user to toggle between checked and not checked.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/switch-demo.md",
    },
    page::Block::Md("examples/browser/snippets/switch-usage.md"),
    page::Block::Example {
        id: "description",
        title: "Description",
        description: Some("Pair a switch with a label and supporting description."),
        snippet: "examples/browser/snippets/switch-description.md",
    },
    page::Block::Example {
        id: "choice-card",
        title: "Choice Card",
        description: Some("Wrap a horizontal field in a clickable choice card."),
        snippet: "examples/browser/snippets/switch-choice-card.md",
    },
    page::Block::Example {
        id: "disabled",
        title: "Disabled",
        description: Some("A disabled switch cannot be toggled."),
        snippet: "examples/browser/snippets/switch-disabled.md",
    },
    page::Block::Example {
        id: "invalid",
        title: "Invalid",
        description: Some(
            "Apply the invalid style to the switch and its label to identify a validation error.",
        ),
        snippet: "examples/browser/snippets/switch-invalid.md",
    },
    page::Block::Example {
        id: "sizes",
        title: "Size",
        description: Some("Use `.size(...)` to change the switch size."),
        snippet: "examples/browser/snippets/switch-sizes.md",
    },
    page::Block::Md("examples/browser/snippets/switch-api-reference.md"),
];

pub struct SwitchPage {
    state: page::State,
}

impl SwitchPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

impl page::Page for SwitchPage {
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
            padding: [24, 32].into(),
            ..Default::default()
        }
    }

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        let inner: iced::Element<'_, Message, Theme> = match id {
            "demo" => field(switch())
                .orientation(Horizontal)
                .label("Airplane Mode")
                .into(),

            "description" => field(switch())
                .orientation(Horizontal)
                .control_last()
                .label("Share across devices")
                .description(
                    "Focus is shared across devices, and turns off when you leave the app.",
                )
                .width(384)
                .into(),

            "disabled" => field(switch())
                .orientation(Horizontal)
                .label(field::label("Disabled").style(Foreground + alpha(0.5)))
                .disabled(true)
                .into(),

            "invalid" => field(switch().style(switch::Invalid))
                .orientation(Horizontal)
                .control_last()
                .label(field::label("Accept terms and conditions").style(Destructive))
                .description("You must accept the terms and conditions to continue.")
                .width(384)
                .into(),

            "choice-card" => field::group([
                field(switch())
                    .orientation(Horizontal)
                    .control_last()
                    .label(field::label("Share across devices").line_height(20. / 14.))
                    .description(
                        "Focus is shared across devices, and turns off when you leave the app.",
                    )
                    .card()
                    .into(),
                field(switch().default_checked(true))
                    .orientation(Horizontal)
                    .control_last()
                    .label(field::label("Enable notifications").line_height(20. / 14.))
                    .description("Receive notifications when focus mode is enabled or disabled.")
                    .card()
                    .into(),
            ])
            .width(384)
            .into(),

            "sizes" => field::group([
                field(switch().size(switch::Size::Sm))
                    .orientation(Horizontal)
                    .label("Small")
                    .into(),
                field(switch())
                    .orientation(Horizontal)
                    .label("Default")
                    .into(),
            ])
            .width(160)
            .into(),

            _ => unreachable!("unknown switch preview id: {id}"),
        };

        container(inner).center_x(iced::Fill).into()
    }
}
