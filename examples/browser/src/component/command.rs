use crate::{Message, icon, page};
use iced::Task;
use icedcn::widget::{button, command, dialog};
use icedcn::{Slot, Theme};

pub static META: page::Meta = page::Meta {
    title: "Command",
    description: "Command menu for search and quick actions.",
};
pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/command-demo.md",
    },
    page::Block::Md("examples/browser/snippets/command-about.md"),
    page::Block::Md("examples/browser/snippets/command-composition.md"),
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "basic",
                title: "Basic",
                description: Some("A simple command menu in a dialog."),
                snippet: "examples/browser/snippets/command-basic.md",
            },
            page::Block::Example {
                id: "shortcuts",
                title: "Shortcuts",
                description: None,
                snippet: "examples/browser/snippets/command-shortcuts.md",
            },
            page::Block::Example {
                id: "groups",
                title: "Groups",
                description: Some("A command menu with groups, icons and separators."),
                snippet: "examples/browser/snippets/command-groups.md",
            },
            page::Block::Example {
                id: "scrollable",
                title: "Scrollable",
                description: Some("Scrollable command menu with multiple items."),
                snippet: "examples/browser/snippets/command-scrollable.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/command-api-reference.md"),
];
pub struct CommandPage {
    state: page::State,
}

impl CommandPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

fn preview_demo() -> Slot<'static, Message> {
    command([
        command::item("Calendar")
            .group("Suggestions")
            .icon(icon::calendar),
        command::item("Search Emoji")
            .group("Suggestions")
            .icon(icon::smile),
        command::item("Calculator")
            .group("Suggestions")
            .icon(icon::calculator)
            .disabled(true),
        command::item("Profile")
            .group("Settings")
            .icon(icon::user)
            .shortcut("⌘P"),
        command::item("Billing")
            .group("Settings")
            .icon(icon::credit_card)
            .shortcut("⌘B"),
        command::item("Settings")
            .group("Settings")
            .icon(icon::settings)
            .shortcut("⌘S"),
    ])
    .width(iced::Fill.max(384.0))
    .into()
}
fn preview_basic() -> Slot<'static, Message> {
    dialog(
        |d| d.trigger(button("Open Menu").style(button::Outline)),
        |_| {
            command([
                command::item("Calendar").group("Suggestions"),
                command::item("Search Emoji").group("Suggestions"),
                command::item("Calculator").group("Suggestions"),
            ])
            .into()
        },
    )
    .show_close_button(false)
    .max_width(512.0)
    .into()
}
fn preview_shortcuts() -> Slot<'static, Message> {
    dialog(
        |d| d.trigger(button("Open Menu").style(button::Outline)),
        |_| {
            command([
                command::item("Profile")
                    .group("Settings")
                    .icon(icon::user)
                    .shortcut("⌘P"),
                command::item("Billing")
                    .group("Settings")
                    .icon(icon::credit_card)
                    .shortcut("⌘B"),
                command::item("Settings")
                    .group("Settings")
                    .icon(icon::settings)
                    .shortcut("⌘S"),
            ])
            .into()
        },
    )
    .show_close_button(false)
    .max_width(512.0)
    .into()
}
fn preview_groups() -> Slot<'static, Message> {
    dialog(
        |d| d.trigger(button("Open Menu").style(button::Outline)),
        |_| {
            command([
                command::item("Calendar")
                    .group("Suggestions")
                    .icon(icon::calendar),
                command::item("Search Emoji")
                    .group("Suggestions")
                    .icon(icon::smile),
                command::item("Calculator")
                    .group("Suggestions")
                    .icon(icon::calculator),
                command::item("Profile")
                    .group("Settings")
                    .icon(icon::user)
                    .shortcut("⌘P"),
                command::item("Billing")
                    .group("Settings")
                    .icon(icon::credit_card)
                    .shortcut("⌘B"),
                command::item("Settings")
                    .group("Settings")
                    .icon(icon::settings)
                    .shortcut("⌘S"),
            ])
            .into()
        },
    )
    .show_close_button(false)
    .max_width(512.0)
    .into()
}
fn preview_scrollable() -> Slot<'static, Message> {
    dialog(
        |d| d.trigger(button("Open Menu").style(button::Outline)),
        |_| {
            command([
                command::item("Home")
                    .group("Navigation")
                    .icon(icon::house)
                    .shortcut("⌘H"),
                command::item("Inbox")
                    .group("Navigation")
                    .icon(icon::inbox)
                    .shortcut("⌘I"),
                command::item("Documents")
                    .group("Navigation")
                    .icon(icon::file_text)
                    .shortcut("⌘D"),
                command::item("Folders")
                    .group("Navigation")
                    .icon(icon::folder)
                    .shortcut("⌘F"),
                command::item("New File")
                    .group("Actions")
                    .icon(icon::plus)
                    .shortcut("⌘N"),
                command::item("New Folder")
                    .group("Actions")
                    .icon(icon::folder_plus)
                    .shortcut("⇧⌘N"),
                command::item("Copy")
                    .group("Actions")
                    .icon(icon::copy)
                    .shortcut("⌘C"),
                command::item("Cut")
                    .group("Actions")
                    .icon(icon::scissors)
                    .shortcut("⌘X"),
                command::item("Paste")
                    .group("Actions")
                    .icon(icon::clipboard_paste)
                    .shortcut("⌘V"),
                command::item("Delete")
                    .group("Actions")
                    .icon(icon::trash)
                    .shortcut("⌫"),
                command::item("Grid View")
                    .group("View")
                    .icon(icon::layout_grid),
                command::item("List View").group("View").icon(icon::list),
                command::item("Zoom In")
                    .group("View")
                    .icon(icon::zoom_in)
                    .shortcut("⌘+"),
                command::item("Zoom Out")
                    .group("View")
                    .icon(icon::zoom_out)
                    .shortcut("⌘-"),
                command::item("Profile")
                    .group("Account")
                    .icon(icon::user)
                    .shortcut("⌘P"),
                command::item("Billing")
                    .group("Account")
                    .icon(icon::credit_card)
                    .shortcut("⌘B"),
                command::item("Settings")
                    .group("Account")
                    .icon(icon::settings)
                    .shortcut("⌘S"),
                command::item("Notifications")
                    .group("Account")
                    .icon(icon::bell),
                command::item("Help & Support")
                    .group("Account")
                    .icon(icon::circle_question_mark),
                command::item("Calculator")
                    .group("Tools")
                    .icon(icon::calculator),
                command::item("Calendar")
                    .group("Tools")
                    .icon(icon::calendar),
                command::item("Image Editor")
                    .group("Tools")
                    .icon(icon::image),
                command::item("Code Editor").group("Tools").icon(icon::code),
            ])
            .into()
        },
    )
    .show_close_button(false)
    .max_width(512.0)
    .into()
}
impl page::Page for CommandPage {
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
            "demo" => preview_demo().compose(),
            "basic" => preview_basic().compose(),
            "shortcuts" => preview_shortcuts().compose(),
            "groups" => preview_groups().compose(),
            "scrollable" => preview_scrollable().compose(),
            _ => unreachable!("unknown command preview: {id}"),
        }
    }
}
