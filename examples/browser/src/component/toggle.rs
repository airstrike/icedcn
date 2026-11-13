// Intentional skips: toggle-rtl (RTL). shadcn's demo fills its bookmark
// icon while on; lucide's font glyphs are outlines only.

use iced::widget::component::{Component, component};
use iced::widget::svg;
use iced::{Element, Task, Widget};
use std::sync::OnceLock;

use icedxdy::widget::{row, toggle};
use icedxdy::{Size, Theme};

use crate::page;
use crate::{Message, icon};

pub static META: page::Meta = page::Meta {
    title: "Toggle",
    description: "A two-state button that can be either on or off.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/toggle-demo.md",
    },
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "outline",
                title: "Outline",
                description: Some("Use `.style(toggle::Outline)` for an outline style."),
                snippet: "examples/browser/snippets/toggle-outline.md",
            },
            page::Block::Example {
                id: "text",
                title: "With Text",
                description: None,
                snippet: "examples/browser/snippets/toggle-text.md",
            },
            page::Block::Example {
                id: "sizes",
                title: "Size",
                description: Some("Use `.size(..)` to change the size of the toggle."),
                snippet: "examples/browser/snippets/toggle-sizes.md",
            },
            page::Block::Example {
                id: "disabled",
                title: "Disabled",
                description: None,
                snippet: "examples/browser/snippets/toggle-disabled.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/toggle-api-reference.md"),
];

struct Bookmark;
impl<'a> Component<'a, Message, Theme> for Bookmark {
    type State = bool;
    type Event = bool;
    fn update(&self, state: &mut bool, next: bool, _: &iced::Renderer) -> Option<Message> {
        *state = next;
        None
    }
    fn view(&self, pressed: &bool) -> impl Widget<bool, Theme> + 'a {
        static FILLED: OnceLock<svg::Handle> = OnceLock::new();
        let glyph: icedxdy::Slot<'a, bool> = if *pressed {
            let handle=FILLED.get_or_init(||svg::Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M19 21 12 17 5 21V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2z" fill="black" stroke="black" stroke-linejoin="round" stroke-width="2"/></svg>"#));
            svg(handle.clone())
                .width(14)
                .height(14)
                .style(|t: &Theme, _| svg::Style {
                    color: Some(t.neutral.foreground.into()),
                })
                .boxed()
                .into()
        } else {
            icon::bookmark().size(14).into()
        };
        Element::from(
            toggle(row![glyph, "Bookmark"].spacing(6))
                .size(Size::Sm)
                .style(toggle::Outline)
                .pressed(*pressed)
                .on_pressed_change(|v| v),
        )
    }
}
pub struct TogglePage {
    state: page::State,
}

impl TogglePage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

impl page::Page for TogglePage {
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
            // shadcn `toggle-demo.tsx`.
            "demo" => component(Bookmark).boxed(),

            // shadcn `toggle-outline.tsx`.
            "outline" => row![
                toggle![italic, "Italic"].style(toggle::Outline),
                toggle![bold, "Bold"].style(toggle::Outline),
            ]
            .spacing(8)
            .align_y(iced::Center)
            .into(),

            // shadcn `toggle-text.tsx`.
            "text" => toggle![italic, "Italic"].into(),

            // shadcn `toggle-sizes.tsx`.
            "sizes" => row![
                toggle("Small").size(Size::Sm).style(toggle::Outline),
                toggle("Default").style(toggle::Outline),
                toggle("Large").size(Size::Lg).style(toggle::Outline),
            ]
            .spacing(8)
            .align_y(iced::Center)
            .into(),

            // shadcn `toggle-disabled.tsx`.
            "disabled" => row![
                toggle("Disabled").disabled(true),
                toggle("Disabled").style(toggle::Outline).disabled(true),
            ]
            .spacing(8)
            .align_y(iced::Center)
            .into(),

            _ => unreachable!("unknown toggle preview id: {id}"),
        }
    }
}
