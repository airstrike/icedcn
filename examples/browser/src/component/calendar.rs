// Intentional skip per AGENTS.md: calendar-rtl and the RTL-only changelog.
pub mod recipes;
use crate::{Message, page};
use iced::{Element, Fill, Task, font};
use icedcn::Theme;
use icedcn::widget::calendar::persian;
use icedcn::widget::container;

pub static META: page::Meta = page::Meta {
    title: "Calendar",
    description: "A calendar component that allows users to select a date or a range of dates.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/calendar-demo.md",
    },
    page::Block::Md("examples/browser/snippets/calendar-installation.md"),
    page::Block::Md("examples/browser/snippets/calendar-usage.md"),
    page::Block::Md("examples/browser/snippets/calendar-guide-about.md"),
    page::Block::Md("examples/browser/snippets/calendar-guide-date-picker.md"),
    page::Block::Md("examples/browser/snippets/calendar-guide-persian.md"),
    page::Block::Example {
        id: "hijri",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/calendar-hijri.md",
    },
    page::Block::Md("examples/browser/snippets/calendar-guide-selected-date-with-timezone.md"),
    page::Block::Example {
        id: "basic",
        title: "Basic",
        description: Some("Use `.bordered(true)` to give a standalone calendar a rounded border."),
        snippet: "examples/browser/snippets/calendar-basic.md",
    },
    page::Block::Example {
        id: "range",
        title: "Range Calendar",
        description: Some("Use `.mode(Mode::Range)` to enable range selection."),
        snippet: "examples/browser/snippets/calendar-range.md",
    },
    page::Block::Example {
        id: "caption",
        title: "Month and Year Selector",
        description: Some("Use `.caption_dropdown(true)` to show month and year dropdowns."),
        snippet: "examples/browser/snippets/calendar-caption.md",
    },
    page::Block::Example {
        id: "presets",
        title: "Presets",
        description: None,
        snippet: "examples/browser/snippets/calendar-presets.md",
    },
    page::Block::Example {
        id: "time",
        title: "Date and Time Picker",
        description: None,
        snippet: "examples/browser/snippets/calendar-time.md",
    },
    page::Block::Example {
        id: "booked-dates",
        title: "Booked dates",
        description: None,
        snippet: "examples/browser/snippets/calendar-booked-dates.md",
    },
    page::Block::Example {
        id: "custom-days",
        title: "Custom Cell Size",
        description: None,
        snippet: "examples/browser/snippets/calendar-custom-days.md",
    },
    page::Block::Md("examples/browser/snippets/calendar-guide-cell-size.md"),
    page::Block::Example {
        id: "week-numbers",
        title: "Week Numbers",
        description: Some("Use `.week_numbers(true)` to show week numbers."),
        snippet: "examples/browser/snippets/calendar-week-numbers.md",
    },
    page::Block::Md("examples/browser/snippets/calendar-api-reference.md"),
];
pub struct CalendarPage {
    state: page::State,
    wide: Option<bool>,
}
impl CalendarPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
            wide: None,
        })
    }
}
impl page::Page for CalendarPage {
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
        let index = self.state.page_index;
        Task::batch([
            self.state.init(),
            font::load(persian::FONT).map(move |_| Message::Page(index, page::Message::Noop)),
        ])
    }
    fn update(&mut self, event: page::Message) -> Task<Message> {
        self.state.update(&event)
    }
    fn reset_volatile(&mut self) {
        self.state.reset_volatile();
    }
    fn window_width(&mut self, width: f32) {
        self.wide = Some(width >= 768.);
    }
    fn backdrop(&self) -> page::Backdrop {
        page::Backdrop {
            padding: 40.into(),
            ..Default::default()
        }
    }
    fn docs_preview(&self, id: &str, outer: &Theme) -> Element<'_, Message, Theme> {
        let height = match id {
            "hijri" => 400.,
            "range" if self.wide == Some(false) => 576.,
            "presets" => 650.,
            "time" => 600.,
            "custom-days" => 560.,
            _ => 384.,
        };
        container(self.themed_preview(id, outer))
            .center_x(Fill)
            .center_y(height - 80.)
            .into()
    }
    fn preview(&self, id: &str) -> Element<'_, Message, Theme> {
        match id {
            "demo" => recipes::demo(),
            "hijri" => recipes::hijri(),
            "basic" => recipes::basic(),
            "range" => recipes::range(self.wide),
            "caption" => recipes::caption(),
            "presets" => recipes::presets(),
            "time" => recipes::time(self.wide.unwrap_or(true)),
            "booked-dates" => recipes::booked_dates(),
            "custom-days" => recipes::custom_days(self.wide.unwrap_or(true)),
            "week-numbers" => recipes::week_numbers(),
            _ => unreachable!("unknown Calendar preview: {id}"),
        }
    }
}
