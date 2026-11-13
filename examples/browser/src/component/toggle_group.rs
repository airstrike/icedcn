use iced::widget::component::{Component, component};
use iced::{Element, Task, Widget};

use icedxdy::widget::{column, field, row, text, toggle_group};
use icedxdy::{Size, Theme, Vertical};
use icedxdy::{Slot, Weight};

use crate::page;
use crate::{Message, icon};

pub static META: page::Meta = page::Meta {
    title: "Toggle Group",
    description: "A set of two-state buttons that can be toggled on or off.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/toggle-group-demo.md",
    },
    page::Block::Md("examples/browser/snippets/toggle-group-composition.md"),
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "outline",
                title: "Outline",
                description: Some("Use `.style(toggle_group::Outline)` for an outline style."),
                snippet: "examples/browser/snippets/toggle-group-outline.md",
            },
            page::Block::Example {
                id: "sizes",
                title: "Size",
                description: Some("Use `.size(..)` to change the size of the toggle group."),
                snippet: "examples/browser/snippets/toggle-group-sizes.md",
            },
            page::Block::Example {
                id: "spacing",
                title: "Spacing",
                description: Some("Use `.spacing(..)` to add space between the items."),
                snippet: "examples/browser/snippets/toggle-group-spacing.md",
            },
            page::Block::Example {
                id: "vertical",
                title: "Vertical",
                description: Some("Use `.orientation(Vertical)` for a vertical toggle group."),
                snippet: "examples/browser/snippets/toggle-group-vertical.md",
            },
            page::Block::Example {
                id: "disabled",
                title: "Disabled",
                description: None,
                snippet: "examples/browser/snippets/toggle-group-disabled.md",
            },
            page::Block::Example {
                id: "font-weight-selector",
                title: "Custom",
                description: Some(
                    "Compose a custom toggle group to create a font weight selector.",
                ),
                snippet: "examples/browser/snippets/toggle-group-font-weight-selector.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/toggle-group-api-reference.md"),
];

pub struct ToggleGroupPage {
    state: page::State,
}

impl ToggleGroupPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

/// Bold, italic and underline items.
fn formatting<'a>() -> [toggle_group::Item<'a, Message>; 3] {
    [
        toggle_group::item("bold", icon::bold()),
        toggle_group::item("italic", icon::italic()),
        toggle_group::item("underline", icon::underline()),
    ]
}

/// Top, bottom, left and right items.
fn sides<'a>() -> [toggle_group::Item<'a, Message>; 4] {
    ["Top", "Bottom", "Left", "Right"].map(|side| toggle_group::item(side.to_lowercase(), side))
}

struct FontWeight;
impl<'a> Component<'a, Message, Theme> for FontWeight {
    type State = Option<String>;
    type Event = Vec<String>;

    fn update(
        &self,
        state: &mut Self::State,
        selection: Self::Event,
        _: &iced::Renderer,
    ) -> Option<Message> {
        *state = Some(selection.into_iter().next().unwrap_or_default());
        None
    }

    fn view(&self, state: &Self::State) -> impl Widget<Self::Event, Theme> + 'a {
        let selected = state.as_deref().unwrap_or("normal");
        let group = toggle_group(|| {
            [
                ("Light", Weight::Light),
                ("Normal", Weight::Normal),
                ("Medium", Weight::Medium),
                ("Bold", Weight::Bold),
            ]
            .map(|(name, weight)| {
                toggle_group::item(
                    name.to_lowercase(),
                    column![
                        text::Text::new("Aa")
                            .size(24.0)
                            .line_height(1.0)
                            .weight(weight),
                        text::muted(name).size(12.0)
                    ]
                    .align_x(iced::Center),
                )
                .square(64)
            })
        })
        .value([selected])
        .on_value_change(|values| values)
        .style(toggle_group::Outline)
        .spacing(8)
        .size(Size::Lg);
        let description = row![
            text::muted("Use"),
            text::inline_code(format!("font-{selected}")),
            text::muted("to set the font weight.")
        ]
        .spacing(4);
        Element::from(
            field(group)
                .label("Font Weight")
                .description(Slot::from(description))
                .width(iced::Shrink),
        )
    }
}

impl page::Page for ToggleGroupPage {
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
            // shadcn `toggle-group-demo.tsx`.
            "demo" => toggle_group(formatting)
                .mode(toggle_group::Mode::Multiple)
                .style(toggle_group::Outline)
                .into(),

            // shadcn `toggle-group-outline.tsx`.
            "outline" => toggle_group(|| {
                [
                    toggle_group::item("all", "All"),
                    toggle_group::item("missed", "Missed"),
                ]
            })
            .default_value(["all"])
            .style(toggle_group::Outline)
            .into(),

            // shadcn `toggle-group-sizes.tsx`.
            "sizes" => column![
                toggle_group(sides)
                    .default_value(["top"])
                    .size(Size::Sm)
                    .style(toggle_group::Outline),
                toggle_group(sides)
                    .default_value(["top"])
                    .style(toggle_group::Outline),
            ]
            .spacing(16)
            .into(),

            // shadcn `toggle-group-spacing.tsx`: `spacing={2}` is 8px.
            "spacing" => toggle_group(sides)
                .default_value(["top"])
                .size(Size::Sm)
                .spacing(8)
                .style(toggle_group::Outline)
                .into(),

            // shadcn `toggle-group-vertical.tsx`: `spacing={1}` is 4px.
            "vertical" => toggle_group(formatting)
                .mode(toggle_group::Mode::Multiple)
                .default_value(["bold", "italic"])
                .orientation(Vertical)
                .spacing(4)
                .into(),

            // shadcn `toggle-group-disabled.tsx`.
            "disabled" => toggle_group(formatting)
                .mode(toggle_group::Mode::Multiple)
                .disabled(true)
                .into(),

            "font-weight-selector" => component(FontWeight).boxed(),

            _ => unreachable!("unknown toggle group preview id: {id}"),
        }
    }
}
