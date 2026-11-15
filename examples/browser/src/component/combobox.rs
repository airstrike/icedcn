// API Reference is rendered via the `combobox-api-reference.md` snippet at
// the bottom of BLOCKS.
//
// Intentional skips:
// - combobox-rtl: RTL is not in scope (per AGENTS.md).

use iced::Task;

use icedcn::Theme;
use icedcn::widget::combobox;

use crate::Message;
use crate::page;

pub static META: page::Meta = page::Meta {
    title: "Combobox",
    description: "Autocomplete input with a list of suggestions.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/combobox-demo.md",
    },
    page::Block::Md("examples/browser/snippets/combobox-usage.md"),
    page::Block::Group {
        title: "Composition",
        children: &[
            page::Block::Md("examples/browser/snippets/combobox-composition.md"),
            page::Block::Md("examples/browser/snippets/combobox-composition-chips.md"),
            page::Block::Md("examples/browser/snippets/combobox-composition-groups.md"),
        ],
    },
    page::Block::Md("examples/browser/snippets/combobox-guide-custom-items.md"),
    page::Block::Md("examples/browser/snippets/combobox-guide-multiple-selection.md"),
    page::Block::Example {
        id: "basic",
        title: "Basic",
        description: Some("A searchable list of Rust GUI libraries."),
        snippet: "examples/browser/snippets/combobox-demo.md",
    },
    page::Block::Example {
        id: "multiple",
        title: "Multiple",
        description: Some("Use `.multiple(true)` to display each selection as a removable chip."),
        snippet: "examples/browser/snippets/combobox-multiple.md",
    },
    page::Block::Example {
        id: "clear",
        title: "Clear Button",
        description: Some("Use `.clearable(true)` to let the user clear the current value."),
        snippet: "examples/browser/snippets/combobox-clear.md",
    },
    page::Block::Example {
        id: "groups",
        title: "Groups",
        description: Some("Assign items to named groups with `.group(...)`."),
        snippet: "examples/browser/snippets/combobox-groups.md",
    },
    page::Block::Example {
        id: "custom",
        title: "Custom Items",
        description: Some(
            "Add a second line with `.description(...)` to help distinguish similar items.",
        ),
        snippet: "examples/browser/snippets/combobox-custom.md",
    },
    page::Block::Example {
        id: "invalid",
        title: "Invalid",
        description: Some("Use `.invalid(true)` to mark an invalid selection."),
        snippet: "examples/browser/snippets/combobox-invalid.md",
    },
    page::Block::Example {
        id: "disabled",
        title: "Disabled",
        description: Some("Use `.disabled(true)` to disable the combobox."),
        snippet: "examples/browser/snippets/combobox-disabled.md",
    },
    page::Block::Example {
        id: "auto-highlight",
        title: "Auto Highlight",
        description: Some(
            "Use `.auto_highlight(true)` to highlight the first matching item as the user types.",
        ),
        snippet: "examples/browser/snippets/combobox-auto-highlight.md",
    },
    page::Block::Example {
        id: "popup",
        title: "Popup",
        description: Some("Use `.popup(true)` to open a searchable list from a button."),
        snippet: "examples/browser/snippets/combobox-popup.md",
    },
    page::Block::Example {
        id: "input-group",
        title: "Input Group",
        description: Some("Add an icon before the input with `.leading_icon(...)`."),
        snippet: "examples/browser/snippets/combobox-input-group.md",
    },
    page::Block::Md("examples/browser/snippets/combobox-api-reference.md"),
];

const FRAMEWORKS: [&str; 5] = ["iced", "egui", "Slint", "GTK", "Druid"];

const COUNTRIES: &[(&str, &str, &str)] = &[
    ("", "Select country", ""),
    ("ar", "Argentina", "South America"),
    ("au", "Australia", "Oceania"),
    ("br", "Brazil", "South America"),
    ("ca", "Canada", "North America"),
    ("cn", "China", "Asia"),
    ("co", "Colombia", "South America"),
    ("eg", "Egypt", "Africa"),
    ("fr", "France", "Europe"),
    ("de", "Germany", "Europe"),
    ("it", "Italy", "Europe"),
    ("jp", "Japan", "Asia"),
    ("ke", "Kenya", "Africa"),
    ("mx", "Mexico", "North America"),
    ("nz", "New Zealand", "Oceania"),
    ("ng", "Nigeria", "Africa"),
    ("za", "South Africa", "Africa"),
    ("kr", "South Korea", "Asia"),
    ("gb", "United Kingdom", "Europe"),
    ("us", "United States", "North America"),
];

const ZONES: &[(&str, &[&str])] = &[
    (
        "Americas",
        &[
            "(GMT-5) New York",
            "(GMT-8) Los Angeles",
            "(GMT-6) Chicago",
            "(GMT-5) Toronto",
            "(GMT-8) Vancouver",
            "(GMT-3) São Paulo",
        ],
    ),
    (
        "Europe",
        &[
            "(GMT+0) London",
            "(GMT+1) Paris",
            "(GMT+1) Berlin",
            "(GMT+1) Rome",
            "(GMT+1) Madrid",
            "(GMT+1) Amsterdam",
        ],
    ),
    (
        "Asia/Pacific",
        &[
            "(GMT+9) Tokyo",
            "(GMT+8) Shanghai",
            "(GMT+8) Singapore",
            "(GMT+4) Dubai",
            "(GMT+11) Sydney",
            "(GMT+9) Seoul",
        ],
    ),
];

/// The combobox page. Every combobox is uncontrolled.
pub struct ComboboxPage {
    state: page::State,
    text_size: f32,
}

impl ComboboxPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
            text_size: 14.,
        })
    }
}

impl page::Page for ComboboxPage {
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
        self.text_size = if width < 768. { 16. } else { 14. };
    }

    fn backdrop(&self) -> page::Backdrop {
        page::Backdrop {
            padding: 40.into(),
            ..Default::default()
        }
    }

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        let field = match id {
            "demo" | "basic" => combobox(FRAMEWORKS).placeholder("Select a framework"),

            "disabled" => combobox(FRAMEWORKS)
                .placeholder("Select a framework")
                .disabled(true),

            "multiple" => combobox(FRAMEWORKS)
                .auto_highlight(true)
                .default_values(["iced"])
                .width(320),
            "clear" => combobox(FRAMEWORKS)
                .placeholder("Select a framework")
                .clearable(true)
                .default_value("iced"),
            "groups" => combobox::options(ZONES.iter().flat_map(|&(group, zones)| {
                zones
                    .iter()
                    .map(move |&zone| combobox::item(zone, zone).group(group))
            }))
            .placeholder("Select a timezone")
            .empty("No timezones found."),
            "custom" => combobox::options(
                COUNTRIES
                    .iter()
                    .filter(|(code, _, _)| !code.is_empty())
                    .map(|&(code, label, continent)| {
                        combobox::item(code, label).description(format!("{continent} ({code})"))
                    }),
            )
            .placeholder("Search countries...")
            .empty("No countries found."),
            "invalid" => combobox(FRAMEWORKS)
                .placeholder("Select a framework")
                .invalid(true),
            "auto-highlight" => combobox(FRAMEWORKS)
                .placeholder("Select a framework")
                .auto_highlight(true),
            "popup" => combobox::options(
                COUNTRIES
                    .iter()
                    .map(|&(code, label, _)| combobox::item(code, label)),
            )
            .popup(true)
            .placeholder("Search")
            .default_value(""),
            "input-group" => combobox::options(ZONES.iter().flat_map(|&(group, zones)| {
                zones
                    .iter()
                    .map(move |&zone| combobox::item(zone, zone).group(group))
            }))
            .placeholder("Select a timezone")
            .empty("No timezones found.")
            .leading_icon(combobox::globe)
            .separators(false)
            .content_width(240)
            .align_offset(-4.),
            _ => unreachable!("unknown combobox preview id: {id}"),
        };
        field.text_size(self.text_size).into()
    }
}
