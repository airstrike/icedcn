// Custom Colors and RTL are excluded by the browser component scope.

use iced::alignment;
use iced::widget::{self, svg};
use iced::{Task, Widget};

use icedxdy::widget::{badge, row, spinner};
use icedxdy::{Slot, Theme};

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
    page::Block::Md("examples/browser/snippets/badge-installation.md"),
    page::Block::Md("examples/browser/snippets/badge-usage.md"),
    page::Block::Example {
        id: "variants",
        title: "Variants",
        description: Some("Use `.style(...)` to change the variant of the badge."),
        snippet: "examples/browser/snippets/badge-variants.md",
    },
    page::Block::Example {
        id: "icon",
        title: "With Icon",
        description: Some(
            "You can render an icon inside the badge. Use `.icon_start()` to render the icon on the left and `.icon_end()` to render the icon on the right.",
        ),
        snippet: "examples/browser/snippets/badge-icon.md",
    },
    page::Block::Example {
        id: "spinner",
        title: "With Spinner",
        description: Some(
            "You can render a spinner inside the badge. Remember to add `.icon_start()` or `.icon_end()` to the badge on the spinner's side.",
        ),
        snippet: "examples/browser/snippets/badge-spinner.md",
    },
    page::Block::Example {
        id: "link",
        title: "Link",
        description: Some(
            "Use `.on_press(...)` to render an interactive badge. Handle the message in your application's `update` to navigate to the link.",
        ),
        snippet: "examples/browser/snippets/badge-link.md",
    },
    page::Block::Group {
        title: "API Reference",
        children: &[page::Block::Md(
            "examples/browser/snippets/badge-api-reference.md",
        )],
    },
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
        "demo" => {
            widget::row::<iced::Element<'_, _, Theme>>([
                badge("Badge").into(),
                badge("Secondary").style(badge::Secondary).into(),
                badge("Destructive").style(badge::Destructive).into(),
                badge("Outline").style(badge::Outline).into(),
            ])
            .spacing(8)
            .wrap()
            .align_x(alignment::Horizontal::Center)
            .boxed()
            .into()
        },
        "variants" => {
            row![
                badge("Default"),
                badge("Secondary").style(badge::Secondary),
                badge("Destructive").style(badge::Destructive),
                badge("Outline").style(badge::Outline),
                badge("Ghost").style(badge::Ghost),
            ]
            .spacing(8)
            .wrap()
            .into()
        },
        "icon" => {
            row![
                badge![svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"/><path d="m9 12 2 2 4-4"/></svg>"#.as_bytes()))
                        .width(12)
                        .height(12)
                        .style(|theme: &Theme, _| svg::Style { color: Some(theme.secondary.foreground.into()) })
                        .boxed(), "Verified"]
                    .icon_start()
                    .style(badge::Secondary),
                badge!["Bookmark", svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M17 3a2 2 0 0 1 2 2v15a1 1 0 0 1-1.496.868l-4.512-2.578a2 2 0 0 0-1.984 0l-4.512 2.578A1 1 0 0 1 5 20V5a2 2 0 0 1 2-2z"/></svg>"#.as_bytes()))
                        .width(12)
                        .height(12)
                        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
                        .boxed()]
                    .icon_end()
                    .style(badge::Outline),
            ]
            .spacing(8)
            .wrap()
            .into()
        },
        "spinner" => {
            row![
                badge![spinner().size(12.0), "Deleting"]
                    .icon_start()
                    .style(badge::Destructive),
                badge!["Generating", spinner().size(12.0)]
                    .icon_end()
                    .style(badge::Secondary),
            ]
            .spacing(8)
            .wrap()
            .into()
        },
        "link" => {
            badge!["Open Link", svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M7 7h10v10M7 17 17 7"/></svg>"#.as_bytes()))
                        .width(12)
                        .height(12)
                        .style(|theme: &Theme, _| svg::Style { color: Some(theme.primary.foreground.into()) })
                        .boxed()]
                .icon_end()
                .on_press(page::Message::LinkClicked("#link".into()))
                .into()
        },
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
