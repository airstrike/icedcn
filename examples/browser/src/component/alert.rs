// Custom Colors and RTL are omitted under the browser component scope.

mod action;
mod default;
mod destructive;

use crate::icon;
use iced::{Fill, Task};

use icedcn::widget::{alert, column};
use icedcn::{Theme, View};

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
}

impl AlertPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
            wide: true,
        })
    }
}

fn demo<'a, M: 'a>() -> View<'a, M> {
    let notice =
        |icon, title, description| alert().icon(icon).title(title).description(description);
    column![
        notice(icon::circle_check().size(16).line_height(1.0),
            "Payment successful",
            "Your payment of $29.99 has been processed. A receipt has been sent to your email address."),
        notice(icon::info().size(16).line_height(1.0),
            "New feature available",
            "We've added dark mode support. You can enable it in your account settings."),
    ].width(Fill.max(448)).spacing(16).into()
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
    }

    fn backdrop(&self) -> page::Backdrop {
        page::Backdrop {
            min_height: if self.wide { 288.0 } else { 0.0 },
            padding: if self.wide {
                24.into()
            } else {
                [24, 16].into()
            },
            ..page::Backdrop::default()
        }
    }

    fn preview(&self, id: &str) -> iced::Element<'_, crate::Message, Theme> {
        let view: View<'_, crate::Message> = match id {
            "demo" => demo(),
            "basic" => default::view(),
            "destructive" => destructive::view(),
            "action" => action::view(None),
            _ => unreachable!("unknown alert preview id: {id}"),
        };
        view.into()
    }
}
