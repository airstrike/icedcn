//! Breadcrumb component page — mirrors the Radix Nova documentation.
//! RTL is intentionally excluded by project policy.

use crate::page;
use crate::page::Message;
use iced::Widget;
use iced::widget::button::{Style, StyleFn};
use iced::widget::svg;
use iced::{Element, Fill, Task};
use icedxdy::theme::tweak::{Fg, Token, hover};
use icedxdy::widget::{breadcrumb, button, container, dropdown_menu, menu, row};
use icedxdy::{Size, Slot, Theme, Weight};

pub static META: page::Meta = page::Meta {
    title: "Breadcrumb",
    description: "Displays the path to the current resource using a hierarchy of links.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/breadcrumb-demo.md",
    },
    page::Block::Md("examples/browser/snippets/breadcrumb-installation.md"),
    page::Block::Md("examples/browser/snippets/breadcrumb-usage.md"),
    page::Block::Md("examples/browser/snippets/breadcrumb-composition.md"),
    page::Block::Example {
        id: "basic",
        title: "Basic",
        description: Some("A basic breadcrumb with a home link and a components link."),
        snippet: "examples/browser/snippets/breadcrumb-basic.md",
    },
    page::Block::Example {
        id: "separator",
        title: "Custom separator",
        description: Some(
            "Use a custom component as `children` for `<BreadcrumbSeparator />` to create a custom separator.",
        ),
        snippet: "examples/browser/snippets/breadcrumb-separator.md",
    },
    page::Block::Example {
        id: "dropdown",
        title: "Dropdown",
        description: Some(
            "You can compose `<BreadcrumbItem />` with a `<DropdownMenu />` to create a dropdown in the breadcrumb.",
        ),
        snippet: "examples/browser/snippets/breadcrumb-dropdown.md",
    },
    page::Block::Example {
        id: "ellipsis",
        title: "Collapsed",
        description: Some(
            "We provide a `<BreadcrumbEllipsis />` component to show a collapsed state when the breadcrumb is too long.",
        ),
        snippet: "examples/browser/snippets/breadcrumb-ellipsis.md",
    },
    page::Block::Example {
        id: "link",
        title: "Link component",
        description: Some(
            "To use a custom link component from your routing library, you can use the `asChild` prop on `<BreadcrumbLink />`.",
        ),
        snippet: "examples/browser/snippets/breadcrumb-link.md",
    },
    page::Block::Group {
        title: "API Reference",
        children: &[
            page::Block::Md("examples/browser/snippets/breadcrumb-api-reference.md"),
            page::Block::Md("examples/browser/snippets/breadcrumb-api-list.md"),
            page::Block::Md("examples/browser/snippets/breadcrumb-api-item.md"),
            page::Block::Md("examples/browser/snippets/breadcrumb-api-link.md"),
            page::Block::Md("examples/browser/snippets/breadcrumb-api-page.md"),
            page::Block::Md("examples/browser/snippets/breadcrumb-api-separator.md"),
            page::Block::Md("examples/browser/snippets/breadcrumb-api-ellipsis.md"),
        ],
    },
];

pub struct BreadcrumbPage {
    state: page::State,
    wide: bool,
}
impl BreadcrumbPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
            wide: true,
        })
    }
}

fn dot() -> Slot<'static, Message> {
    svg(svg::Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12.1" cy="12.1" r="1"/></svg>"#))
        .width(14)
        .height(14)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.muted.foreground.into()) })
        .boxed()
        .into()
}

fn down() -> Slot<'static, Message> {
    svg(svg::Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m6 9 6 6 6-6"/></svg>"#))
        .width(14)
        .height(14)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.muted.foreground.into()) })
        .boxed()
        .into()
}

fn destinations(_: &dropdown_menu::Handle) -> Slot<'static, Message> {
    menu::group([
        menu::item("Documentation").into(),
        menu::item("Themes").into(),
        menu::item("GitHub").into(),
    ])
}

fn trigger_style() -> StyleFn<'static, Theme> {
    Box::new(|theme, _| Style {
        text_color: theme.muted.foreground.into(),
        ..Default::default()
    })
}

fn basic() -> Slot<'static, Message> {
    breadcrumb([
        breadcrumb::link("Home", Message::Noop),
        breadcrumb::link("Components", Message::Noop),
        breadcrumb::page("Breadcrumb"),
    ])
    .into()
}

fn link() -> Slot<'static, Message> {
    breadcrumb([
        breadcrumb::link("Home", Message::Noop),
        breadcrumb::link("Components", Message::Noop),
        breadcrumb::page("Breadcrumb"),
    ])
    .into()
}

fn separator() -> Slot<'static, Message> {
    breadcrumb([
        breadcrumb::link("Home", Message::Noop),
        breadcrumb::link("Components", Message::Noop),
        breadcrumb::page("Breadcrumb"),
    ])
    .separator(dot)
    .into()
}

fn ellipsis() -> Slot<'static, Message> {
    breadcrumb([
        breadcrumb::link("Home", Message::Noop),
        breadcrumb::ellipsis(None),
        breadcrumb::link("Components", Message::Noop),
        breadcrumb::page("Breadcrumb"),
    ])
    .into()
}

fn demo() -> Slot<'static, Message> {
    breadcrumb([
        breadcrumb::link("Home", Message::Noop),
        breadcrumb::item(
            dropdown_menu(
                |m| {
                    m.trigger(
                        button(breadcrumb::ellipsis(None))
                            .size(Size::Sm)
                            .square()
                            .style(
                                button::Ghost
                                    + Fg(Token::MutedForeground)
                                    + hover(Fg(Token::Foreground)),
                            )
                            .translate(false),
                    )
                },
                destinations,
            )
            .align(dropdown_menu::Align::Start),
        ),
        breadcrumb::link("Components", Message::Noop),
        breadcrumb::page("Breadcrumb"),
    ])
    .into()
}

fn dropdown() -> Slot<'static, Message> {
    breadcrumb([
        breadcrumb::link("Home", Message::Noop),
        breadcrumb::item(
            dropdown_menu(
                |m| {
                    m.trigger(
                        button(row!["Components", down()].spacing(4))
                            .height(20)
                            .padding(0)
                            .weight(Weight::Normal)
                            .translate(false)
                            .focus_ring(2, |theme| theme.neutral.foreground)
                            .style(trigger_style()),
                    )
                },
                destinations,
            )
            .align(dropdown_menu::Align::Start),
        ),
        breadcrumb::page("Breadcrumb"),
    ])
    .separator(dot)
    .into()
}

fn view(id: &str) -> Slot<'static, Message> {
    match id {
        "demo" => demo(),
        "basic" => basic(),
        "separator" => separator(),
        "dropdown" => dropdown(),
        "ellipsis" => ellipsis(),
        "link" => link(),
        _ => unreachable!("unknown breadcrumb example: {id}"),
    }
}

impl page::Page for BreadcrumbPage {
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

    fn backdrop(&self) -> page::Backdrop {
        page::Backdrop {
            padding: iced::Padding::ZERO,
            ..page::Backdrop::default()
        }
    }

    fn window_width(&mut self, width: f32) {
        self.wide = width >= 768.;
    }

    fn docs_preview(&self, id: &str, outer: &Theme) -> Element<'_, crate::Message, Theme> {
        let padding = if matches!(id, "demo" | "ellipsis") {
            8
        } else if !self.wide {
            // The native shell contributes 32px; upstream contributes 24px.
            // Together with this inset, both leave 64px before the content.
            32
        } else {
            40
        };
        container(self.themed_preview(id, outer))
            .center_x(Fill)
            .padding(padding)
            .into()
    }

    fn preview(&self, id: &str) -> iced::Element<'_, crate::Message, Theme> {
        let pi = self.state.page_index;
        view(id)
            .map(move |message| crate::Message::Page(pi, message))
            .into()
    }
}
