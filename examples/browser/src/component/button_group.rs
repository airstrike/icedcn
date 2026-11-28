// Intentional skip per AGENTS.md: button-group-rtl (RTL).

use iced::widget::component::{Component, component};
use iced::{Fill, Shrink, Task, Widget};

use iced::widget::{Text, responsive};
use icedcn::theme::text_input;
use icedcn::theme::tweak::{Bg, Fg, Token, hover};
use icedcn::widget::Row;
use icedcn::widget::button::group;
use icedcn::widget::input_group::Align;
use icedcn::widget::{
    button, column, container, dropdown_menu, field, input, input_group, menu, popover, row,
    select, text, textarea, tooltip,
};
use icedcn::{Theme, View};

use crate::page;
use crate::{Message, icon};

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
    page::Block::Md("examples/browser/snippets/button-group-usage.md"),
    page::Block::Md("examples/browser/snippets/button-group-composition.md"),
    page::Block::Md("examples/browser/snippets/button-group-guide-accessibility.md"),
    page::Block::Md("examples/browser/snippets/button-group-guide-buttongroup-vs-togglegroup.md"),
    page::Block::Example {
        id: "orientation",
        title: "Orientation",
        description: Some("Set `.orientation(...)` to change the button group layout."),
        snippet: "examples/browser/snippets/button-group-orientation.md",
    },
    page::Block::Example {
        id: "size",
        title: "Size",
        description: Some("Control the size of buttons using `.size(...)` on individual buttons."),
        snippet: "examples/browser/snippets/button-group-size.md",
    },
    page::Block::Example {
        id: "nested",
        title: "Nested",
        description: Some("Nest `button::group!` components to create button groups with spacing."),
        snippet: "examples/browser/snippets/button-group-nested.md",
    },
    page::Block::Example {
        id: "separator",
        title: "Separator",
        description: Some(
            "The `group::separator()` component visually divides buttons within a group. Outline buttons do not need a separator since they have a border. For other variants, a separator is recommended to improve the visual hierarchy.",
        ),
        snippet: "examples/browser/snippets/button-group-separator.md",
    },
    page::Block::Example {
        id: "split",
        title: "Split",
        description: Some(
            "Create a split button group by adding two buttons separated by a `group::separator()`.",
        ),
        snippet: "examples/browser/snippets/button-group-split.md",
    },
    page::Block::Example {
        id: "input",
        title: "Input",
        description: Some("Wrap an `input` component with buttons."),
        snippet: "examples/browser/snippets/button-group-input.md",
    },
    page::Block::Example {
        id: "input-group",
        title: "Input Group",
        description: Some("Wrap an `input_group` component to create complex input layouts."),
        snippet: "examples/browser/snippets/button-group-input-group.md",
    },
    page::Block::Example {
        id: "dropdown",
        title: "Dropdown Menu",
        description: Some("Create a split button group with a `dropdown_menu` component."),
        snippet: "examples/browser/snippets/button-group-dropdown.md",
    },
    page::Block::Example {
        id: "select",
        title: "Select",
        description: Some("Pair with a `select` component."),
        snippet: "examples/browser/snippets/button-group-select.md",
    },
    page::Block::Example {
        id: "popover",
        title: "Popover",
        description: Some("Use with a `popover` component."),
        snippet: "examples/browser/snippets/button-group-popover.md",
    },
    page::Block::Group {
        title: "API Reference",
        children: &[
            page::Block::Md("examples/browser/snippets/button-group-api-reference.md"),
            page::Block::Md("examples/browser/snippets/button-group-api-separator.md"),
            page::Block::Md("examples/browser/snippets/button-group-api-text.md"),
        ],
    },
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
        view(id).compose()
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
        responsive(move |area| {
            View::from(container(mail_actions(area.width >= 320., selected)).center_x(Fill))
                .compose()
        })
        .height(Shrink)
    }
}

fn mail_actions<'a>(wide: bool, selected: &'static str) -> View<'a, &'static str> {
    let mut actions = Row::new().spacing(8).align_y(iced::Center);
    if wide {
        actions = actions.push(
            button::group![
                button(icon::arrow_left().size(16).line_height(1.0))
                    .square()
                    .style(button::Outline)
            ]
            .into(),
        );
    }
    let more = dropdown_menu(
        |m| {
            button::group![
                button("Snooze").style(button::Outline),
                m.trigger(
                    button(icon::ellipsis().size(16).line_height(1.0))
                        .square()
                        .style(button::Outline)
                        .has_popup(true)
                ),
            ]
            .into()
        },
        move |_| {
            let caption = |glyph: Text<'a, Theme>, label| {
                menu::caption(glyph.size(16).line_height(1.0), label)
            };
            let item = |glyph, label| menu::item(caption(glyph, label));
            let labels = ["Personal", "Work", "Other"]
                .map(|label| menu::radio_item(label, label, Some(selected), |value| value).into());
            menu::group![
                item(icon::mail_check(), "Mark as Read"),
                item(icon::archive(), "Archive"),
                menu::separator(),
                item(icon::clock(), "Snooze"),
                item(icon::calendar_plus(), "Add to Calendar"),
                item(icon::list_filter(), "Add to List"),
                menu::sub(caption(icon::tag(), "Label As..."), labels),
                menu::separator(),
                item(icon::trash_2(), "Trash").style(menu::Destructive),
            ]
        },
    )
    .align(dropdown_menu::Align::End)
    .width(160);
    actions
        .push(
            button::group![
                button("Archive").style(button::Outline),
                button("Report").style(button::Outline)
            ]
            .into(),
        )
        .push(more.into())
        .into()
}
pub(super) fn demo<'a>() -> View<'a, Message> {
    component(MailActions).boxed().into()
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
        voice_input(*enabled).compose()
    }
}

fn voice_input<'a>(enabled: bool) -> View<'a, bool> {
    let toggle = input_group::button(icon::audio_lines().size(16).line_height(1.0))
        .square()
        .on_press(!enabled);
    let toggle = if enabled {
        let background = Token::themed(iced::color!(0xffedd5), iced::color!(0x9a3412));
        let foreground = Token::themed(iced::color!(0xc2410c), iced::color!(0xffedd5));
        toggle.style(button::Ghost + Bg(background) + Fg(foreground))
    } else {
        toggle.style(button::Ghost + Fg(Token::MutedForeground) + hover(Fg(Token::Foreground)))
    };
    let placeholder = if enabled {
        "Record and send audio..."
    } else {
        "Send a message..."
    };
    let editor = input_group(input(placeholder).disabled(enabled))
        .style(text_input::Default + icedcn::radius(9999.))
        .addon(input_group::addon(tooltip(toggle, "Voice Mode")).align(Align::InlineEnd));
    button::group![
        button::group![
            button(icon::plus().size(16).line_height(1.0))
                .rounded()
                .square()
                .style(button::Outline)
        ],
        button::group![editor].width(Fill),
    ]
    .width(Fill.max(256.))
    .into()
}

pub(super) fn view<'a>(id: &str) -> View<'a, Message> {
    use icedcn::Size::{Default, Lg, Sm};

    match id {
        "demo" => demo(),
        "orientation" => button::group![
            button(icon::plus().size(16).line_height(1.0))
                .square()
                .style(button::Outline),
            button(icon::minus().size(16).line_height(1.0))
                .square()
                .style(button::Outline),
        ]
        .orientation(group::Vertical)
        .into(),

        "size" => column([(Sm, "Small"), (Default, "Default"), (Lg, "Large")].map(
            |(size, first)| {
                let action = |label| button(label).size(size).style(button::Outline);
                button::group![
                    action(first),
                    action("Button"),
                    action("Group"),
                    button(icon::plus().size(16).line_height(1.0))
                        .size(size)
                        .square()
                        .style(button::Outline)
                ]
                .into()
            },
        ))
        .spacing(32)
        .align_x(iced::Left)
        .into(),

        "nested" => button::group![
            button::group![
                button(icon::plus().size(16).line_height(1.0))
                    .square()
                    .style(button::Outline)
            ],
            button::group![
                input_group(input("Send a message...")).addon(
                    input_group::addon(tooltip(
                        icon::audio_lines().size(16).line_height(1.0),
                        "Voice Mode"
                    ))
                    .align(Align::InlineEnd)
                )
            ]
            .width(Fill)
        ]
        .width(Fill.max(256.))
        .into(),

        "separator" => button::group![
            button("Copy").style(button::Secondary).size(Sm),
            group::separator(),
            button("Paste").style(button::Secondary).size(Sm),
        ]
        .into(),

        "split" => button::group![
            button("Button").style(button::Secondary),
            group::separator(),
            button(icon::plus().size(16).line_height(1.0))
                .square()
                .style(button::Secondary)
        ]
        .into(),

        "input" => button::group![
            input("Search...").width(Fill),
            button(icon::search().size(16).line_height(1.0)).style(button::Outline)
        ]
        .width(Fill.max(240.))
        .into(),

        "input-group" => View::from(component(Voice).boxed()),

        "dropdown" => dropdown_menu(
            |&m| {
                button::group![
                    button("Follow").style(button::Outline),
                    m.trigger(
                        button(icon::chevron_down().size(16).line_height(1.0))
                            .style(button::Outline)
                            .padding(iced::Padding::ZERO.left(8).right(10))
                            .has_popup(true)
                    )
                ]
                .into()
            },
            |_| {
                let item = |glyph: Text<'a, Theme>, label| {
                    menu::item(menu::caption(glyph.size(16).line_height(1.0), label))
                };
                menu::group![
                    item(icon::volume_off(), "Mute Conversation"),
                    item(icon::check(), "Mark as Read"),
                    item(icon::triangle_alert(), "Report Conversation"),
                    item(icon::user_round_x(), "Block User"),
                    item(icon::share(), "Share Conversation"),
                    item(icon::copy(), "Copy Conversation"),
                    menu::separator(),
                    item(icon::trash(), "Delete Conversation").style(menu::Destructive),
                ]
            },
        )
        .align(dropdown_menu::Align::End)
        .width(176)
        .into(),

        "select" => button::group![
            button::group![
                select(
                    [("$", "US Dollar"), ("€", "Euro"), ("£", "British Pound")]
                        .map(|(value, label)| select::item(value, label))
                )
                .default_value("$")
                .format_value(|value, _| value.to_owned())
                .font(iced::Font::MONOSPACE)
                .format_item(|value, label| row![
                    text::small(value.to_owned()),
                    text::muted(label.to_owned())
                ]
                .spacing(4))
                .width(50)
                .min_width(96.),
                input("10.00").width(Fill)
            ]
            .width(Fill),
            button::group![
                button(icon::arrow_right().size(16).line_height(1.0))
                    .square()
                    .style(button::Outline)
            ]
        ]
        .width(Fill.max(280.))
        .into(),

        "popover" => popover(
            |&p| {
                button::group![
                    button![icon::bot().size(16).line_height(1.0), "Copilot"]
                        .style(button::Outline),
                    p.trigger(
                        button(icon::chevron_down().size(16).line_height(1.0))
                            .square()
                            .style(button::Outline)
                            .has_popup(true)
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
