// Intentional skip per AGENTS.md: button-group-rtl (RTL).
mod icon;

use iced::widget::component::{Component, component};
use iced::{Element, Fill, Task, Widget};

use iced::widget::svg;
use icedcn::theme::button::{IntoStyle, state};
use icedcn::theme::text_input;
use icedcn::widget::Row;
use icedcn::widget::button::group;
use icedcn::widget::input_group::Align;
use icedcn::widget::{
    button, button_group, column, container, dropdown_menu, field, input, input_group, menu,
    popover, row, select, text, textarea, tooltip,
};
use icedcn::{Slot, Theme};

use crate::Message;
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
    page::Block::Md("examples/browser/snippets/button-group-installation.md"),
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
        description: Some("Nest `button_group!` components to create button groups with spacing."),
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
    window_width: f32,
}

impl ButtonGroupPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
            window_width: 1440.,
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

    fn window_width(&mut self, width: f32) {
        self.window_width = width;
    }

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        view(id, self.window_width).compose()
    }
}

struct MailActions {
    wide: bool,
}
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
            Row::with_children(
                self.wide
                    .then(|| {
                        Slot::from(button_group![
                            button(icon::arrow_left().boxed())
                                .square()
                                .style(button::Outline)
                        ])
                    })
                    .into_iter()
                    .chain([
                        button_group![
                            button("Archive").style(button::Outline),
                            button("Report").style(button::Outline)
                        ]
                        .into(),
                        dropdown_menu(
                            |&m| {
                                button_group![
                                    button("Snooze").style(button::Outline),
                                    m.trigger(
                                        button(icon::ellipsis().boxed())
                                            .style(button::Outline)
                                            .square()
                                            .has_popup(true)
                                    )
                                ]
                                .into()
                            },
                            move |_| {
                                column![
                                    menu::item(
                                        row![icon::mail_check().boxed(), "Mark as Read"].spacing(8)
                                    ),
                                    menu::item(row![icon::archive().boxed(), "Archive"].spacing(8)),
                                    menu::separator(),
                                    menu::item(row![icon::clock().boxed(), "Snooze"].spacing(8)),
                                    menu::item(
                                        row![icon::calendar_plus().boxed(), "Add to Calendar"]
                                            .spacing(8)
                                    ),
                                    menu::item(
                                        row![icon::list_filter().boxed(), "Add to List"].spacing(8)
                                    ),
                                    menu::sub(
                                        row![icon::tag().boxed(), "Label As..."].spacing(8),
                                        ["Personal", "Work", "Other"].map(|label| {
                                            menu::radio_item(
                                                label,
                                                label,
                                                Some(selected),
                                                |label| label,
                                            )
                                            .into()
                                        })
                                    ),
                                    menu::separator(),
                                    menu::item(row![icon::trash_2().boxed(), "Trash"].spacing(8))
                                        .style(menu::Destructive)
                                ]
                                .width(Fill)
                                .into()
                            },
                        )
                        .align(dropdown_menu::Align::End)
                        .width(160)
                        .into(),
                    ]),
            )
            .spacing(8)
            .align_y(iced::Center),
        )
    }
}
pub(super) fn demo(wide: bool) -> Slot<'static, Message> {
    Slot::from(component(MailActions { wide }).boxed())
}

struct Voice {
    narrow: bool,
}
impl<'a> Component<'a, Message, Theme> for Voice {
    type State = bool;
    type Event = bool;
    fn update(&self, state: &mut bool, enabled: bool, _: &iced::Renderer) -> Option<Message> {
        *state = enabled;
        None
    }
    fn view(&self, enabled: &bool) -> impl Widget<bool, Theme> + 'a {
        let enabled = *enabled;
        let audio = icon::audio_lines().style(move |theme: &Theme, _| svg::Style {
            color: Some(if enabled {
                voice_foreground(theme)
            } else {
                theme.neutral.foreground.into()
            }),
        });
        let toggle = input_group::button(audio.boxed())
            .square()
            .on_press(!enabled);
        let toggle = if enabled {
            toggle.style(VoiceActive)
        } else {
            toggle
        };
        Element::from(button_group![
            button_group![
                button(icon::plus().boxed())
                    .rounded()
                    .square()
                    .style(button::Outline)
            ],
            button_group![
                input_group(
                    input(if enabled {
                        "Record and send audio..."
                    } else {
                        "Send a message..."
                    })
                    .text_size(if self.narrow { 16. } else { 14. })
                    .disabled(enabled)
                )
                .style(Box::new(|theme: &Theme, status| {
                    let mut style = text_input::default(theme, status);
                    style.border.radius = 9999.0.into();
                    style
                }) as text_input::StyleFn<'static, Theme>)
                .addon(input_group::addon(tooltip(toggle, "Voice Mode")).align(Align::InlineEnd))
            ]
            .width(if self.narrow { 220. } else { 215.203 })
        ])
    }
}

/// Keep the Ghost variant's geometry and keyboard decoration in both themes.
struct VoiceActive;
impl<'a> IntoStyle<'a> for VoiceActive {
    fn into_style(self) -> state::StyleFn<'a> {
        let ghost = button::Ghost.into_style();
        Box::new(move |theme, status| {
            let mut style = ghost(theme, status);
            style.paint.background = Some(
                if theme.neutral.is_dark {
                    iced::Color::from_rgb8(154, 52, 18)
                } else {
                    iced::Color::from_rgb8(255, 237, 213)
                }
                .into(),
            );
            style.paint.text_color = voice_foreground(theme);
            style
        })
    }
}

fn voice_foreground(theme: &Theme) -> iced::Color {
    if theme.neutral.is_dark {
        iced::Color::from_rgb8(255, 237, 213)
    } else {
        iced::Color::from_rgb8(194, 65, 12)
    }
}

pub(super) fn view(id: &str, window_width: f32) -> Slot<'static, Message> {
    use icedcn::Size::{Lg, Sm};
    let narrow = window_width < 768.;

    match id {
        "demo" => Slot::from(
            component(MailActions {
                wide: window_width >= 640.,
            })
            .boxed(),
        ),
        "orientation" => button_group![
            button(icon::plus().boxed()).square().style(button::Outline),
            button(icon::minus().boxed())
                .square()
                .style(button::Outline),
        ]
        .orientation(group::Vertical)
        .into(),

        "size" => column([
            button_group![
                button("Small").style(button::Outline).size(Sm),
                button("Button").style(button::Outline).size(Sm),
                button("Group").style(button::Outline).size(Sm),
                button(icon::plus().boxed())
                    .square()
                    .style(button::Outline)
                    .size(Sm),
            ]
            .into(),
            button_group![
                button("Default").style(button::Outline),
                button("Button").style(button::Outline),
                button("Group").style(button::Outline),
                button(icon::plus().boxed()).square().style(button::Outline),
            ]
            .into(),
            button_group![
                button("Large").style(button::Outline).size(Lg),
                button("Button").style(button::Outline).size(Lg),
                button("Group").style(button::Outline).size(Lg),
                button(icon::plus().boxed())
                    .square()
                    .style(button::Outline)
                    .size(Lg),
            ]
            .into(),
        ])
        .spacing(32)
        .align_x(iced::Left)
        .into(),

        "nested" => button_group![
            button_group![button(icon::plus().boxed()).square().style(button::Outline)],
            button_group![
                input_group(input("Send a message...").text_size(if narrow { 16. } else { 14. }))
                    .addon(
                        input_group::addon(tooltip(
                            container(icon::audio_lines().boxed()).padding([0, 0]),
                            "Voice Mode"
                        ))
                        .align(Align::InlineEnd)
                    )
            ]
            .width(if narrow { 220. } else { 212. })
        ]
        .into(),

        "separator" => button_group![
            button("Copy").style(button::Secondary).size(Sm),
            group::separator(),
            button("Paste").style(button::Secondary).size(Sm),
        ]
        .into(),

        "split" => button_group![
            button("Button").style(button::Secondary),
            group::separator(),
            button(icon::tabler_plus().boxed())
                .square()
                .style(button::Secondary)
        ]
        .into(),

        "input" => button_group![
            input("Search...")
                .text_size(if narrow { 16. } else { 14. })
                .width(if narrow { 216. } else { 192. }),
            button(icon::search().boxed()).style(button::Outline)
        ]
        .into(),

        "input-group" => Slot::from(component(Voice { narrow }).boxed()),

        "dropdown" => dropdown_menu(
            |&m| {
                button_group![
                    button("Follow").style(button::Outline),
                    m.trigger(
                        button(icon::chevron_down().boxed())
                            .style(button::Outline)
                            .padding(iced::Padding::ZERO.left(8).right(10))
                            .has_popup(true)
                    )
                ]
                .into()
            },
            |_| {
                column![
                    menu::item(row![icon::volume_off().boxed(), "Mute Conversation"].spacing(8)),
                    menu::item(row![icon::check().boxed(), "Mark as Read"].spacing(8)),
                    menu::item(
                        row![icon::triangle_alert().boxed(), "Report Conversation"].spacing(8)
                    ),
                    menu::item(row![icon::user_round_x().boxed(), "Block User"].spacing(8)),
                    menu::item(row![icon::share().boxed(), "Share Conversation"].spacing(8)),
                    menu::item(row![icon::copy().boxed(), "Copy Conversation"].spacing(8)),
                    menu::separator(),
                    menu::item(row![icon::trash().boxed(), "Delete Conversation"].spacing(8))
                        .style(menu::Destructive)
                ]
                .width(Fill)
                .into()
            },
        )
        .align(dropdown_menu::Align::End)
        .width(176)
        .into(),

        "select" => button_group![
            button_group![
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
                .width(50.406)
                .min_width(96.)
                .icon_content(icon::chevron_down),
                input("10.00")
                    .text_size(if narrow { 16. } else { 14. })
                    .width(Fill)
            ]
            .width(if narrow { 220. } else { 241.406 }),
            button_group![
                button(icon::arrow_right().boxed())
                    .square()
                    .style(button::Outline)
            ]
        ]
        .into(),

        "popover" => popover(
            |&p| {
                button_group![
                    button![icon::bot().boxed(), "Copilot"].style(button::Outline),
                    p.trigger(
                        button(icon::chevron_down().boxed())
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
