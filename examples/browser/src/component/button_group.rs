use iced::widget::component::{Component, component};
use iced::{Element, Fill, Task, Widget};

use icedxdy::widget::button::group;
use icedxdy::widget::input_group::Align;
use icedxdy::widget::{
    button, button_group, column, container, dropdown_menu, field, input, input_group, menu,
    popover, row, select, textarea, tooltip,
};
use icedxdy::{Slot, Theme};

use crate::Message;
use crate::icon;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Button Group",
    description: "A container that groups related buttons together with consistent styling.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/button-group-demo.md",
    },
    page::Block::Md("examples/browser/snippets/button-group-composition.md"),
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "orientation",
                title: "Orientation",
                description: Some("Use `.orientation(...)` to change the button group layout."),
                snippet: "examples/browser/snippets/button-group-orientation.md",
            },
            page::Block::Example {
                id: "size",
                title: "Size",
                description: Some(
                    "Control the size of buttons using `.size(...)` on individual buttons.",
                ),
                snippet: "examples/browser/snippets/button-group-size.md",
            },
            page::Block::Example {
                id: "nested",
                title: "Nested",
                description: Some("Nest button groups to create button groups with spacing."),
                snippet: "examples/browser/snippets/button-group-nested.md",
            },
            page::Block::Example {
                id: "separator",
                title: "Separator",
                description: Some(
                    "Use `group::separator()` to visually divide buttons within a \
                     group. Outline buttons do not need a separator since they \
                     have a border. For other variants, a separator is recommended \
                     to improve the visual hierarchy.",
                ),
                snippet: "examples/browser/snippets/button-group-separator.md",
            },
            page::Block::Example {
                id: "split",
                title: "Split",
                description: None,
                snippet: "examples/browser/snippets/button-group-split.md",
            },
            page::Block::Example {
                id: "input",
                title: "Input",
                description: Some("Group an input with a button."),
                snippet: "examples/browser/snippets/button-group-input.md",
            },
            page::Block::Example {
                id: "input-group",
                title: "Input Group",
                description: None,
                snippet: "examples/browser/snippets/button-group-input-group.md",
            },
            page::Block::Example {
                id: "dropdown",
                title: "Dropdown Menu",
                description: None,
                snippet: "examples/browser/snippets/button-group-dropdown.md",
            },
            page::Block::Example {
                id: "select",
                title: "Select",
                description: None,
                snippet: "examples/browser/snippets/button-group-select.md",
            },
            page::Block::Example {
                id: "popover",
                title: "Popover",
                description: None,
                snippet: "examples/browser/snippets/button-group-popover.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/button-group-api-reference.md"),
];

pub struct ButtonGroupPage {
    state: page::State,
}

impl ButtonGroupPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

impl page::Page for ButtonGroupPage {
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
        view(id, self.state.page_index).compose()
    }
}

struct MailActions;
impl<'a> Component<'a, Message, Theme> for MailActions {
    type State = Option<&'static str>;
    type Event = &'static str;
    fn update(
        &self,
        label: &mut Self::State,
        value: Self::Event,
        _: &iced::Renderer,
    ) -> Option<Message> {
        *label = Some(value);
        None
    }
    fn view(&self, state: &Self::State) -> impl Widget<Self::Event, Theme> + 'a {
        let selected = state.unwrap_or("Personal");
        Element::from(
            row![
                button(icon::arrow_left()).square().style(button::Outline),
                button_group![
                    button("Archive").style(button::Outline),
                    button("Report").style(button::Outline)
                ],
                dropdown_menu(
                    |&m| button_group![
                        button("Snooze").style(button::Outline),
                        group::decorated(
                            button(icon::ellipsis()).style(button::Outline).square(),
                            move |b| m.trigger(b)
                        )
                    ]
                    .into(),
                    move |_| column![
                        menu::item(row![icon::mail_check().size(16), "Mark as Read"].spacing(8)),
                        menu::item(row![icon::archive().size(16), "Archive"].spacing(8)),
                        menu::separator(),
                        menu::item(row![icon::clock().size(16), "Snooze"].spacing(8)),
                        menu::item(
                            row![icon::calendar_plus().size(16), "Add to Calendar"].spacing(8)
                        ),
                        menu::item(row![icon::list_filter().size(16), "Add to List"].spacing(8)),
                        menu::sub(
                            row![icon::tag(), "Label As..."].spacing(8),
                            ["Personal", "Work", "Other"].map(|label| menu::radio_item(
                                label,
                                label,
                                Some(selected),
                                |label| label
                            )
                            .into())
                        ),
                        menu::separator(),
                        menu::item(row![icon::trash_2().size(16), "Trash"].spacing(8))
                            .style(menu::Destructive)
                    ]
                    .width(Fill)
                    .into()
                )
                .align(dropdown_menu::Align::End)
                .width(160)
            ]
            .spacing(8)
            .align_y(iced::Center),
        )
    }
}
pub(super) fn demo() -> Slot<'static, Message> {
    Slot::from(component(MailActions).boxed())
}

struct Voice;
impl<'a> Component<'a, Message, Theme> for Voice {
    type State = bool;
    type Event = bool;
    fn update(&self, state: &mut bool, enabled: bool, _: &iced::Renderer) -> Option<Message> {
        *state = enabled;
        None
    }
    fn view(&self, enabled: &bool) -> impl Widget<bool, Theme> + 'a {
        let toggle = input_group::button(icon::audio_lines())
            .square()
            .on_press(!*enabled);
        let toggle = if *enabled {
            toggle.style(button::Secondary)
        } else {
            toggle
        };
        Element::from(
            row![
                button(icon::plus())
                    .rounded()
                    .square()
                    .style(button::Outline),
                input_group(
                    input(if *enabled {
                        "Record and send audio..."
                    } else {
                        "Send a message..."
                    })
                    .disabled(*enabled)
                )
                .addon(input_group::addon(tooltip(toggle, "Voice Mode")).align(Align::InlineEnd))
            ]
            .spacing(8)
            .align_y(iced::Center),
        )
    }
}

fn view(id: &str, pi: usize) -> Slot<'static, Message> {
    use icedxdy::Size::{Lg, Sm};

    // Stub on_press target — every button in the page wires `.on_press(noop())`
    // so they aren't styled as disabled. Cloning is fine because `Message::Page`
    // is a thin enum variant.
    let noop = || Message::Page(pi, page::Message::Noop);

    match id {
        "demo" => demo(),
        // Orientation. shadcn `button-group-orientation.tsx`: vertical
        // group of two icon buttons (plus, minus).
        "orientation" => button_group![
            button(icon::plus())
                .square()
                .style(button::Outline)
                .on_press(noop()),
            button(icon::minus())
                .square()
                .style(button::Outline)
                .on_press(noop()),
        ]
        .orientation(group::Vertical)
        .into(),

        // Size. shadcn `button-group-size.tsx`: three groups stacked
        // vertically, each with three text buttons + one icon button.
        // Sizes: sm / default / lg respectively. icon-sm/icon/icon-lg map
        // to a `square()` icon button at the matching text size, since our
        // Size enum drives both the height and the icon-cell width.
        //
        // Each row is wrapped in a center-x container so the size-varying
        // clusters align on a center line rather than a left edge — gives
        // the column a balanced, calibrated look in the preview card.
        "size" => column([
            container(button_group![
                button("Small")
                    .style(button::Outline)
                    .size(Sm)
                    .on_press(noop()),
                button("Button")
                    .style(button::Outline)
                    .size(Sm)
                    .on_press(noop()),
                button("Group")
                    .style(button::Outline)
                    .size(Sm)
                    .on_press(noop()),
                button(icon::plus())
                    .square()
                    .style(button::Outline)
                    .size(Sm)
                    .on_press(noop()),
            ])
            .center_x(Fill)
            .into(),
            container(button_group![
                button("Default").style(button::Outline).on_press(noop()),
                button("Button").style(button::Outline).on_press(noop()),
                button("Group").style(button::Outline).on_press(noop()),
                button(icon::plus())
                    .square()
                    .style(button::Outline)
                    .on_press(noop()),
            ])
            .center_x(Fill)
            .into(),
            container(button_group![
                button("Large")
                    .style(button::Outline)
                    .size(Lg)
                    .on_press(noop()),
                button("Button")
                    .style(button::Outline)
                    .size(Lg)
                    .on_press(noop()),
                button("Group")
                    .style(button::Outline)
                    .size(Lg)
                    .on_press(noop()),
                button(icon::plus())
                    .square()
                    .style(button::Outline)
                    .size(Lg)
                    .on_press(noop()),
            ])
            .center_x(Fill)
            .into(),
        ])
        .spacing(32)
        .into(),

        "nested" => row![
            button(icon::plus()).square().style(button::Outline),
            input_group(input("Send a message...")).addon(
                input_group::addon(tooltip(icon::audio_lines(), "Voice Mode"))
                    .align(Align::InlineEnd)
            )
        ]
        .spacing(8)
        .align_y(iced::Center)
        .into(),

        // Separator. shadcn `button-group-separator.tsx`: two secondary
        // buttons with a `ButtonGroupSeparator` between them.
        "separator" => button_group![
            button("Copy")
                .style(button::Secondary)
                .size(Sm)
                .on_press(noop()),
            group::separator(),
            button("Paste")
                .style(button::Secondary)
                .size(Sm)
                .on_press(noop()),
        ]
        .into(),

        "split" => button_group![
            button("Button").style(button::Secondary),
            group::separator(),
            button(icon::plus()).square().style(button::Secondary)
        ]
        .into(),

        "input" => button_group![
            input("Search..."),
            button(icon::search()).style(button::Outline)
        ]
        .into(),

        "input-group" => Slot::from(component(Voice).boxed()),

        "dropdown" => dropdown_menu(
            |&m| {
                button_group![
                    button("Follow").style(button::Outline),
                    group::decorated(
                        button(icon::chevron_down()).style(button::Outline),
                        move |b| m.trigger(b)
                    )
                ]
                .into()
            },
            |_| {
                column![
                    menu::item(row![icon::volume_off().size(16), "Mute Conversation"].spacing(8)),
                    menu::item(row![icon::check().size(16), "Mark as Read"].spacing(8)),
                    menu::item(
                        row![icon::triangle_alert().size(16), "Report Conversation"].spacing(8)
                    ),
                    menu::item(row![icon::user_round_x().size(16), "Block User"].spacing(8)),
                    menu::item(row![icon::share().size(16), "Share Conversation"].spacing(8)),
                    menu::item(row![icon::copy().size(16), "Copy Conversation"].spacing(8)),
                    menu::separator(),
                    menu::item(row![icon::trash_2().size(16), "Delete Conversation"].spacing(8))
                        .style(menu::Destructive)
                ]
                .width(Fill)
                .into()
            },
        )
        .align(dropdown_menu::Align::End)
        .width(176)
        .into(),

        "select" => row![
            button_group![
                group::Item::Content(
                    select(
                        [
                            ("$", "$ US Dollar"),
                            ("€", "€ Euro"),
                            ("£", "£ British Pound")
                        ]
                        .map(|(v, label)| select::item(v, label))
                    )
                    .default_value("$")
                    .format_value(|value, _| value.to_owned())
                    .width(52)
                    .icon(icon::chevron_down)
                    .into()
                ),
                input("10.00").width(120)
            ],
            button(icon::arrow_right()).square().style(button::Outline)
        ]
        .spacing(8)
        .into(),

        "popover" => popover(
            |&p| {
                button_group![
                    button![icon::bot(), "Copilot"].style(button::Outline),
                    group::decorated(
                        button(icon::chevron_down()).square().style(button::Outline),
                        move |b| p.trigger(b)
                    )
                ]
                .into()
            },
            |_| {
                column![
                    popover::header(
                        "Start a new task with Copilot",
                        "Describe your task in natural language."
                    ),
                    field(textarea("I need to..."))
                        .description("Copilot will open a pull request for review.")
                ]
                .spacing(12)
                .width(Fill)
                .into()
            },
        )
        .align(popover::Align::End)
        .into(),

        _ => unreachable!("unknown button_group preview id: {id}"),
    }
}
