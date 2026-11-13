// API Reference is rendered via the `combobox-api-reference.md` snippet at
// the bottom of BLOCKS.
//
// Intentional skips:
// - combobox-rtl: RTL is not in scope (per CLAUDE.md).

use iced::Task;

use icedxdy::Theme;
use icedxdy::widget::combobox;

use crate::Message;
use crate::icon;
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
    page::Block::Md("examples/browser/snippets/combobox-composition.md"),
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "basic",
                title: "Basic",
                description: Some("A simple combobox with a list of frameworks."),
                snippet: "examples/browser/snippets/combobox-demo.md",
            },
            page::Block::Example {
                id: "multiple",
                title: "Multiple",
                description: Some(
                    "A combobox with multiple selection using `multiple` and `ComboboxChips`.",
                ),
                snippet: "examples/browser/snippets/combobox-multiple.md",
            },
            page::Block::Example {
                id: "clear",
                title: "Clear Button",
                description: Some("Use the `showClear` prop to show a clear button."),
                snippet: "examples/browser/snippets/combobox-clear.md",
            },
            page::Block::Example {
                id: "groups",
                title: "Groups",
                description: Some("Use `ComboboxGroup` and `ComboboxSeparator` to group items."),
                snippet: "examples/browser/snippets/combobox-groups.md",
            },
            page::Block::Example {
                id: "custom",
                title: "Custom Items",
                description: Some("You can render a custom component inside `ComboboxItem`."),
                snippet: "examples/browser/snippets/combobox-custom.md",
            },
            page::Block::Example {
                id: "invalid",
                title: "Invalid",
                description: Some("Use the `aria-invalid` prop to make the combobox invalid."),
                snippet: "examples/browser/snippets/combobox-invalid.md",
            },
            page::Block::Example {
                id: "disabled",
                title: "Disabled",
                description: None,
                snippet: "examples/browser/snippets/combobox-disabled.md",
            },
            page::Block::Example {
                id: "auto-highlight",
                title: "Auto Highlight",
                description: Some(
                    "Use the `autoHighlight` prop to automatically highlight the first item on filter.",
                ),
                snippet: "examples/browser/snippets/combobox-auto-highlight.md",
            },
            page::Block::Example {
                id: "popup",
                title: "Popup",
                description: Some(
                    "You can trigger the combobox from a button or any other component by using the `render` prop. Move the `ComboboxInput` inside the `ComboboxContent`.",
                ),
                snippet: "examples/browser/snippets/combobox-popup.md",
            },
            page::Block::Example {
                id: "input-group",
                title: "Input Group",
                description: Some(
                    "You can add an addon to the combobox by using the `InputGroupAddon` component inside the `ComboboxInput`.",
                ),
                snippet: "examples/browser/snippets/combobox-input-group.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/combobox-api-reference.md"),
];

const FRAMEWORKS: [&str; 5] = ["Next.js", "SvelteKit", "Nuxt.js", "Remix", "Astro"];

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
}

impl ComboboxPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
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

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        match id {
            // shadcn `combobox-demo.tsx` / `combobox-basic.tsx`.
            "demo" | "basic" => combobox(FRAMEWORKS)
                .placeholder("Select a framework")
                .icon(icon::chevron_down)
                .into(),

            // shadcn `combobox-disabled.tsx`.
            "disabled" => combobox(FRAMEWORKS)
                .placeholder("Select a framework")
                .icon(icon::chevron_down)
                .disabled(true)
                .into(),

            "multiple" => combobox(FRAMEWORKS)
                .multiple(true)
                .auto_highlight(true)
                .default_values(["Next.js"])
                .width(320)
                .into(),
            "clear" => combobox(FRAMEWORKS)
                .placeholder("Select a framework")
                .icon(icon::chevron_down)
                .clearable(true)
                .default_value("Next.js")
                .into(),
            "groups" => combobox::options(ZONES.iter().flat_map(|&(group, zones)| {
                zones
                    .iter()
                    .map(move |&zone| combobox::item(zone, zone).group(group))
            }))
            .placeholder("Select a timezone")
            .empty("No timezones found.")
            .icon(icon::chevron_down)
            .into(),
            "custom" => combobox::options(
                COUNTRIES
                    .iter()
                    .filter(|(code, _, _)| !code.is_empty())
                    .map(|&(code, label, continent)| {
                        combobox::item(code, label).description(format!("{continent} ({code})"))
                    }),
            )
            .placeholder("Search countries...")
            .empty("No countries found.")
            .icon(icon::chevron_down)
            .into(),
            "invalid" => combobox(FRAMEWORKS)
                .placeholder("Select a framework")
                .icon(icon::chevron_down)
                .invalid(true)
                .into(),
            "auto-highlight" => combobox(FRAMEWORKS)
                .placeholder("Select a framework")
                .icon(icon::chevron_down)
                .auto_highlight(true)
                .into(),
            "popup" => combobox::options(
                COUNTRIES
                    .iter()
                    .map(|&(code, label, _)| combobox::item(code, label)),
            )
            .popup(true)
            .placeholder("Search")
            .default_value("")
            .into(),
            "input-group" => combobox::options(ZONES.iter().flat_map(|&(group, zones)| {
                zones
                    .iter()
                    .map(move |&zone| combobox::item(zone, zone).group(group))
            }))
            .placeholder("Select a timezone")
            .empty("No timezones found.")
            .icon(icon::chevron_down)
            .leading_icon(icon::globe)
            .into(),
            _ => unreachable!("unknown combobox preview id: {id}"),
        }
    }
}
