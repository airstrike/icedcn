// Custom Colors and RTL are omitted under the browser component scope.

mod action;
mod default;
mod destructive;

use iced::widget::svg;
use iced::{Fill, Task, Widget};

use icedcn::widget::{alert, column, text};
use icedcn::{Slot, Theme};

use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Alert",
    description: "Displays a callout for user attention.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/alert-demo.md",
    },
    page::Block::Md("examples/browser/snippets/alert-installation.md"),
    page::Block::Md("examples/browser/snippets/alert-usage.md"),
    page::Block::Md("examples/browser/snippets/alert-composition.md"),
    page::Block::Example {
        id: "basic",
        title: "Basic",
        description: Some("A basic alert with an icon, title and description."),
        snippet: "examples/browser/snippets/alert-basic.md",
    },
    page::Block::Example {
        id: "destructive",
        title: "Destructive",
        description: Some("Use `.style(alert::Destructive)` to create a destructive alert."),
        snippet: "examples/browser/snippets/alert-destructive.md",
    },
    page::Block::Example {
        id: "action",
        title: "Action",
        description: Some(
            "Use `.action(...)` to add a button or other action element to the alert.",
        ),
        snippet: "examples/browser/snippets/alert-action.md",
    },
    page::Block::Group {
        title: "API Reference",
        children: &[
            page::Block::Md("examples/browser/snippets/alert-api-reference.md"),
            page::Block::Md("examples/browser/snippets/alert-api-title.md"),
            page::Block::Md("examples/browser/snippets/alert-api-description.md"),
            page::Block::Md("examples/browser/snippets/alert-api-action.md"),
        ],
    },
];

pub struct AlertPage {
    state: page::State,
    wide: bool,
    description_wrapping: text::Wrapping,
}

impl AlertPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
            wide: true,
            description_wrapping: text::Wrapping::Word,
        })
    }
}

fn demo<'a>(wrapping: text::Wrapping) -> Slot<'a, crate::Message> {
    const CHECK: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="m9 12 2 2 4-4"/></svg>"#;
    const INFO: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 16v-4M12 8h.01"/></svg>"#;
    column![
        alert()
            .description_wrapping(wrapping)
            .icon(
                svg(svg::Handle::from_memory(CHECK.as_bytes()))
                    .width(16)
                    .height(16)
                    .style(|theme: &Theme, _| svg::Style {
                        color: Some(theme.card.foreground.into()),
                    })
                    .boxed(),
            )
            .title("Payment successful")
            .description(
                "Your payment of $29.99 has been processed. A receipt has been sent to your email address.",
            )
            .max_width(448),
        alert()
            .description_wrapping(wrapping)
            .icon(
                svg(svg::Handle::from_memory(INFO.as_bytes()))
                    .width(16)
                    .height(16)
                    .style(|theme: &Theme, _| svg::Style {
                        color: Some(theme.card.foreground.into()),
                    })
                    .boxed(),
            )
            .title("New feature available")
            .description(
                "We've added dark mode support. You can enable it in your account settings.",
            )
            .max_width(448),
    ]
    .spacing(16)
    .width(Fill.max(448))
    .into()
}

impl page::Page for AlertPage {
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

    fn init(&self) -> Task<crate::Message> {
        self.state.init()
    }

    fn update(&mut self, event: page::Message) -> Task<crate::Message> {
        self.state.update(&event)
    }

    fn reset_volatile(&mut self) {
        self.state.reset_volatile();
    }

    fn window_width(&mut self, width: f32) {
        self.wide = width >= 640.0;
        self.description_wrapping = if width < 768.0 {
            text::Wrapping::Balanced
        } else {
            text::Wrapping::Word
        };
    }

    fn backdrop(&self) -> page::Backdrop {
        page::Backdrop {
            min_height: if self.wide { 288.0 } else { 0.0 },
            ..page::Backdrop::default()
        }
    }

    fn preview(&self, id: &str) -> iced::Element<'_, crate::Message, Theme> {
        let view: Slot<'_, crate::Message> = match id {
            "demo" => demo(self.description_wrapping),
            "basic" => default::view(self.description_wrapping),
            "destructive" => destructive::view(self.description_wrapping),
            "action" => action::view(self.description_wrapping),
            _ => unreachable!("unknown alert preview id: {id}"),
        };
        view.into()
    }
}
