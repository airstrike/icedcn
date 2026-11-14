use crate::{Message, page};
use iced::Task;
use icedxdy::Theme;
mod items;
mod preview;
use preview::preview;

pub static META: page::Meta = page::Meta {
    title: "Questionnaire",
    description: "A multi-step questionnaire with single-choice, multiple-choice, freeform, and skippable questions.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/questionnaire-demo.md",
    },
    page::Block::Md("examples/browser/snippets/questionnaire-usage.md"),
    page::Block::Md("examples/browser/snippets/questionnaire-composition.md"),
    page::Block::Example {
        id: "multiple",
        title: "Multiple Selection",
        description: Some(
            "Use `.multiple(true)` on an item that accepts more than one fixed answer.",
        ),
        snippet: "examples/browser/snippets/questionnaire-multiple.md",
    },
    page::Block::Example {
        id: "freeform",
        title: "Freeform Answer",
        description: Some(
            "Use `.input(label, placeholder)` to offer a freeform answer alongside the fixed choices.",
        ),
        snippet: "examples/browser/snippets/questionnaire-freeform.md",
    },
    page::Block::Example {
        id: "skip",
        title: "Explicit Skip",
        description: Some("Use `.allow_skip(true)` to let the user skip an optional item."),
        snippet: "examples/browser/snippets/questionnaire-skip.md",
    },
    page::Block::Example {
        id: "shortcuts",
        title: "Shortcuts",
        description: Some("Use `.shortcuts(...)` to assign letter or number keys to the choices."),
        snippet: "examples/browser/snippets/questionnaire-shortcuts.md",
    },
    page::Block::Example {
        id: "validation",
        title: "Custom Validation",
        description: Some(
            "Use `.validate(...)` to check the answers. Return `Some(Invalid { item, message })` to take the user back to that item.",
        ),
        snippet: "examples/browser/snippets/questionnaire-validation.md",
    },
    page::Block::Example {
        id: "controlled",
        title: "Controlled",
        description: Some(
            "Control the active item from host state, such as returning to an invalid step.",
        ),
        snippet: "examples/browser/snippets/questionnaire-controlled.md",
    },
    page::Block::Example {
        id: "resume",
        title: "Resume",
        description: Some(
            "Restore a saved active item and default answers, then reset changes back to that saved state.",
        ),
        snippet: "examples/browser/snippets/questionnaire-resume.md",
    },
    page::Block::Example {
        id: "conditional",
        title: "Conditional Items",
        description: Some("Disable items that do not apply to the user's earlier answers."),
        snippet: "examples/browser/snippets/questionnaire-conditional.md",
    },
    page::Block::Example {
        id: "navigation-state",
        title: "Navigation State",
        description: Some(
            "Read item status to opt into disabled navigation and custom action styling.",
        ),
        snippet: "examples/browser/snippets/questionnaire-navigation-state.md",
    },
    page::Block::Example {
        id: "progress",
        title: "Custom Progress",
        description: Some(
            "Pass a closure to `.progress(...)` to build an indicator from the current question number and total.",
        ),
        snippet: "examples/browser/snippets/questionnaire-progress.md",
    },
    page::Block::Example {
        id: "animated",
        title: "Animated Items",
        description: Some(
            "Animate the active item while keeping progress and navigation stationary.",
        ),
        snippet: "examples/browser/snippets/questionnaire-animated.md",
    },
    page::Block::Example {
        id: "card",
        title: "Card",
        description: Some(
            "Compose Questionnaire with Card slots while keeping the question title and description semantic.",
        ),
        snippet: "examples/browser/snippets/questionnaire-card.md",
    },
    page::Block::Example {
        id: "dialog",
        title: "Dialog",
        description: Some(
            "Compose Questionnaire inside a Dialog while keeping cancellation and dismissal host-owned.",
        ),
        snippet: "examples/browser/snippets/questionnaire-dialog.md",
    },
    page::Block::Md("examples/browser/snippets/questionnaire-accessibility.md"),
    page::Block::Md("examples/browser/snippets/questionnaire-api-reference.md"),
];
pub struct QuestionnairePage {
    state: page::State,
}

impl QuestionnairePage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

impl page::Page for QuestionnairePage {
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
            "demo" => preview("demo", self.page_index()),
            "multiple" => preview("multiple", self.page_index()),
            "freeform" => preview("freeform", self.page_index()),
            "skip" => preview("skip", self.page_index()),
            "shortcuts" => preview("shortcuts", self.page_index()),
            "validation" => preview("validation", self.page_index()),
            "controlled" => preview("controlled", self.page_index()),
            "resume" => preview("resume", self.page_index()),
            "conditional" => preview("conditional", self.page_index()),
            "navigation-state" => preview("navigation-state", self.page_index()),
            "progress" => preview("progress", self.page_index()),
            "animated" => preview("animated", self.page_index()),
            "card" => preview("card", self.page_index()),
            "dialog" => preview("dialog", self.page_index()),
            _ => unreachable!("unknown preview: {id}"),
        }
    }
}
