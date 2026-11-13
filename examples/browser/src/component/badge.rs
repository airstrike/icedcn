// rawr
use iced::Task;

use icedxdy::widget::{badge, row, spinner};
use icedxdy::{Slot, Theme};

use crate::icon;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Badge",
    description: "Displays a badge or a component that looks like a badge.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/badge-demo.md",
    },
    page::Block::Md("examples/browser/snippets/badge-usage.md"),
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "variants",
                title: "Variants",
                description: Some("Use the variant prop to change the variant of the badge."),
                snippet: "examples/browser/snippets/badge-variants.md",
            },
            page::Block::Example {
                id: "icon",
                title: "With Icon",
                description: Some("You can render an icon inside the badge."),
                snippet: "examples/browser/snippets/badge-icon.md",
            },
            page::Block::Example {
                id: "spinner",
                title: "With Spinner",
                description: Some("You can render a spinner inside the badge."),
                snippet: "examples/browser/snippets/badge-spinner.md",
            },
            page::Block::Example {
                id: "link",
                title: "Link",
                description: Some("Use the on_press builder to render a clickable badge."),
                snippet: "examples/browser/snippets/badge-link.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/badge-api-reference.md"),
];

pub struct BadgePage {
    state: page::State,
}

impl BadgePage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

fn view(id: &str) -> Slot<'static, page::Message> {
    match id {
        // Demo (hero): 4 variants in a row, verbatim shadcn `badge-demo.tsx`.
        "demo" => row![
            badge("Badge"),
            badge("Secondary").style(badge::Secondary),
            badge("Destructive").style(badge::Destructive),
            badge("Outline").style(badge::Outline),
        ]
        .spacing(8)
        .wrap()
        .into(),
        // Variants (full set incl. ghost)
        "variants" => row![
            badge("Default"),
            badge("Secondary").style(badge::Secondary),
            badge("Destructive").style(badge::Destructive),
            badge("Outline").style(badge::Outline),
            badge("Ghost").style(badge::Ghost),
        ]
        .spacing(8)
        .wrap()
        .into(),
        "icon" => row![
            badge![icon::badge_check().size(12), "Verified"]
                .icon_start()
                .style(badge::Secondary),
            badge!["Bookmark", icon::bookmark().size(12)]
                .icon_end()
                .style(badge::Outline),
        ]
        .spacing(8)
        .wrap()
        .into(),
        "spinner" => row![
            badge![spinner().size(12.0), "Deleting"]
                .icon_start()
                .style(badge::Destructive),
            badge!["Generating", spinner().size(12.0)]
                .icon_end()
                .style(badge::Secondary),
        ]
        .spacing(8)
        .wrap()
        .into(),
        "link" => row![
            badge!["Open Link", icon::arrow_up_right().size(12)]
                .icon_end()
                .on_press(page::Message::LinkClicked("#link".into())),
        ]
        .spacing(8)
        .wrap()
        .into(),
        _ => unreachable!("unknown badge preview id: {id}"),
    }
}

impl page::Page for BadgePage {
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

    fn preview(&self, id: &str) -> iced::Element<'_, crate::Message, Theme> {
        let pi = self.state.page_index;
        view(id)
            .map(move |message| crate::Message::Page(pi, message))
            .into()
    }
}
