// RTL is omitted under the component policy.
mod demo;

use iced::Task;

use crate::icon;
use icedcn::Theme;
use icedcn::widget::{container, row, tabs};

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Tabs",
    description: "A set of layered sections of content, known as tab panels, that are \
                  displayed one at a time.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/tabs-demo.md",
    },
    page::Block::Md("examples/browser/snippets/tabs-installation.md"),
    page::Block::Md("examples/browser/snippets/tabs-usage.md"),
    page::Block::Md("examples/browser/snippets/tabs-composition.md"),
    page::Block::Example {
        id: "line",
        title: "Line",
        description: Some("Use `.line(true)` to underline the active tab."),
        snippet: "examples/browser/snippets/tabs-line.md",
    },
    page::Block::Example {
        id: "vertical",
        title: "Vertical",
        description: Some("Use `.orientation(tabs::Orientation::Vertical)` for vertical tabs."),
        snippet: "examples/browser/snippets/tabs-vertical.md",
    },
    page::Block::Example {
        id: "disabled",
        title: "Disabled",
        description: None,
        snippet: "examples/browser/snippets/tabs-disabled.md",
    },
    page::Block::Example {
        id: "icons",
        title: "Icons",
        description: None,
        snippet: "examples/browser/snippets/tabs-icons.md",
    },
    page::Block::Md("examples/browser/snippets/tabs-api-reference.md"),
];

/// The tabs page. The demo is uncontrolled.
pub struct TabsPage {
    state: page::State,
}

impl TabsPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

impl page::Page for TabsPage {
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

    fn backdrop(&self) -> page::Backdrop {
        page::Backdrop {
            padding: [24., 16.].into(),
            ..Default::default()
        }
    }

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        match id {
            "demo" => container(demo::view()).center_y(336).into(),

            "line" => tabs(|t| {
                t.list([
                    t.trigger("overview", "Overview"),
                    t.trigger("analytics", "Analytics"),
                    t.trigger("reports", "Reports"),
                ])
            })
            .line(true)
            .default_value("overview")
            .into(),
            "vertical" => tabs(|t| {
                t.list([
                    t.trigger("account", "Account"),
                    t.trigger("password", "Password"),
                    t.trigger("notifications", "Notifications"),
                ])
            })
            .orientation(tabs::Orientation::Vertical)
            .default_value("account")
            .into(),
            "disabled" => tabs(|t| {
                t.list([
                    t.trigger("home", "Home"),
                    t.trigger("settings", "Disabled").disabled(true),
                ])
            })
            .default_value("home")
            .into(),
            "icons" => tabs(|t| {
                t.list([
                    t.trigger(
                        "preview",
                        row![icon::app_window().size(16), "Preview"].spacing(6),
                    ),
                    t.trigger("code", row![icon::code().size(16), "Code"].spacing(6)),
                ])
            })
            .default_value("preview")
            .into(),
            _ => unreachable!("unknown tabs preview id: {id}"),
        }
    }
}
