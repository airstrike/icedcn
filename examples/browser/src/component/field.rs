mod forms;
mod recipes;

use crate::{Message, page};
use iced::widget::component::{Component, component};
use iced::{Element, Fill, Task, Widget};
use icedcn::widget::{container, field, slider};
use icedcn::{Theme, View};

pub static META: page::Meta = page::Meta {
    title: "Field",
    description: "Combine labels, controls, and help text to compose accessible form fields and grouped inputs.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/field-demo.md",
    },
    page::Block::Md("examples/browser/snippets/field-usage.md"),
    page::Block::Group {
        title: "Composition",
        children: &[
            page::Block::Md("examples/browser/snippets/field-composition.md"),
            page::Block::Md("examples/browser/snippets/field-composition-group.md"),
            page::Block::Md("examples/browser/snippets/field-composition-set.md"),
        ],
    },
    page::Block::Md("examples/browser/snippets/field-guide-anatomy.md"),
    page::Block::Md("examples/browser/snippets/field-guide-form.md"),
    page::Block::Example {
        id: "input",
        title: "Input",
        description: None,
        snippet: "examples/browser/snippets/field-input.md",
    },
    page::Block::Example {
        id: "textarea",
        title: "Textarea",
        description: None,
        snippet: "examples/browser/snippets/field-textarea.md",
    },
    page::Block::Example {
        id: "select",
        title: "Select",
        description: None,
        snippet: "examples/browser/snippets/field-select.md",
    },
    page::Block::Example {
        id: "slider",
        title: "Slider",
        description: None,
        snippet: "examples/browser/snippets/field-slider.md",
    },
    page::Block::Example {
        id: "fieldset",
        title: "Fieldset",
        description: None,
        snippet: "examples/browser/snippets/field-fieldset.md",
    },
    page::Block::Example {
        id: "checkbox",
        title: "Checkbox",
        description: None,
        snippet: "examples/browser/snippets/field-checkbox.md",
    },
    page::Block::Example {
        id: "radio",
        title: "Radio",
        description: None,
        snippet: "examples/browser/snippets/field-radio.md",
    },
    page::Block::Example {
        id: "switch",
        title: "Switch",
        description: None,
        snippet: "examples/browser/snippets/field-switch.md",
    },
    page::Block::Example {
        id: "choice-card",
        title: "Choice Card",
        description: Some("Make the whole field clickable with .card()."),
        snippet: "examples/browser/snippets/field-choice-card.md",
    },
    page::Block::Example {
        id: "group",
        title: "Field Group",
        description: None,
        snippet: "examples/browser/snippets/field-group.md",
    },
    page::Block::Example {
        id: "responsive",
        title: "Responsive Layout",
        description: None,
        snippet: "examples/browser/snippets/field-responsive.md",
    },
    page::Block::Md("examples/browser/snippets/field-guide-validation-and-errors.md"),
    page::Block::Md("examples/browser/snippets/field-guide-accessibility.md"),
    page::Block::Group {
        title: "API Reference",
        children: &[
            page::Block::Md("examples/browser/snippets/field-api-reference.md"),
            page::Block::Md("examples/browser/snippets/field-api-reference-legend.md"),
            page::Block::Md("examples/browser/snippets/field-api-reference-group.md"),
            page::Block::Md("examples/browser/snippets/field-api-reference-field.md"),
            page::Block::Md("examples/browser/snippets/field-api-reference-content.md"),
            page::Block::Md("examples/browser/snippets/field-api-reference-label.md"),
            page::Block::Md("examples/browser/snippets/field-api-reference-title.md"),
            page::Block::Md("examples/browser/snippets/field-api-reference-description.md"),
            page::Block::Md("examples/browser/snippets/field-api-reference-separator.md"),
            page::Block::Md("examples/browser/snippets/field-api-reference-error.md"),
        ],
    },
];

pub struct FieldPage {
    state: page::State,
    compact: bool,
}

impl FieldPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
            compact: false,
        })
    }
}

impl page::Page for FieldPage {
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
        self.compact = width < 768.;
    }

    fn backdrop(&self) -> page::Backdrop {
        page::Backdrop {
            padding: 0.into(),
            ..Default::default()
        }
    }

    fn docs_preview(&self, id: &str, outer: &Theme) -> Element<'_, Message, Theme> {
        let height = match id {
            "demo" => {
                if self.compact {
                    800
                } else {
                    850
                }
            }
            "responsive" => {
                if self.compact {
                    650
                } else {
                    500
                }
            }
            "checkbox" => 512,
            "group" => 384,
            _ => 288,
        };
        let inset = if self.compact && matches!(id, "demo" | "responsive") {
            16
        } else {
            32
        };
        container(self.themed_preview(id, outer))
            .center_x(Fill)
            .center_y(height)
            .padding([0, inset])
            .into()
    }

    fn preview(&self, id: &str) -> Element<'_, Message, Theme> {
        let size = if self.compact { 16. } else { 14. };
        match id {
            "demo" => component(forms::Form::Payment(size)).boxed(),
            "input" => recipes::credentials(size).into(),
            "textarea" => recipes::feedback().into(),
            "select" => recipes::department().into(),
            "slider" => component(Budget).boxed(),
            "fieldset" => recipes::address(size).into(),
            "checkbox" => recipes::devices().into(),
            "radio" => subscription_plan().into(),
            "switch" => recipes::authentication().into(),
            "choice-card" => recipes::environment().into(),
            "group" => {
                recipes::notifications(Message::Page(self.page_index(), page::Message::ScrollTo(0)))
                    .into()
            }
            "responsive" => component(forms::Form::Profile(size)).boxed(),
            _ => unreachable!("unknown field preview id: {id}"),
        }
    }
}

pub(super) fn subscription_plan<'a>() -> View<'a, Message> {
    recipes::plans()
}

pub(super) fn checkout<'a>() -> Element<'a, Message, Theme> {
    component(forms::Form::Payment(14.)).boxed()
}

struct Budget;

impl<'a> Component<'a, Message, Theme> for Budget {
    type State = Option<[f32; 2]>;
    type Event = [f32; 2];

    fn update(
        &self,
        state: &mut Self::State,
        next: Self::Event,
        _: &iced::Renderer,
    ) -> Option<Message> {
        *state = Some(next);
        None
    }

    fn view(&self, state: &Self::State) -> impl Widget<Self::Event, Theme> + 'a {
        let [low, high] = state.unwrap_or([200., 800.]);
        Element::from(
            field(
                slider::range(0.0..=1000.0)
                    .step(10.)
                    .value([low, high])
                    .on_value_change(|values| values),
            )
            .label("Price Range")
            .description(format!("Set your budget range (${low:.0} - {high:.0})."))
            .description_above()
            .width(320),
        )
    }
}
