// The RTL example is outside the supported browser examples.
use crate::{Message, page};
use iced::widget::component::{Component, component};
use iced::{Element, Fill, Task, Widget};
use icedcn::widget::calendar::{Date, Datelike, Duration, Mode, Selection, today};
use icedcn::widget::{date_picker, field, input, row, text};
use icedcn::{Slot, Theme};

pub static META: page::Meta = page::Meta {
    title: "Date Picker",
    description: "A date picker component with range and presets.",
};

pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/date-picker-demo.md",
    },
    page::Block::Md("examples/browser/snippets/date-picker-usage.md"),
    page::Block::Md("examples/browser/snippets/date-picker-composition.md"),
    page::Block::Example {
        id: "basic",
        title: "Basic",
        description: Some("Choose a single date."),
        snippet: "examples/browser/snippets/date-picker-basic.md",
    },
    page::Block::Example {
        id: "range",
        title: "Range Picker",
        description: Some("Choose a start and end date across two months."),
        snippet: "examples/browser/snippets/date-picker-range.md",
    },
    page::Block::Example {
        id: "dob",
        title: "Date of Birth",
        description: Some("Choose a month and year from the calendar captions."),
        snippet: "examples/browser/snippets/date-picker-dob.md",
    },
    page::Block::Example {
        id: "input",
        title: "Input",
        description: Some("Type a date or select one from the calendar."),
        snippet: "examples/browser/snippets/date-picker-input.md",
    },
    page::Block::Example {
        id: "time",
        title: "Time Picker",
        description: Some("Choose a date and enter a time, including seconds."),
        snippet: "examples/browser/snippets/date-picker-time.md",
    },
    page::Block::Example {
        id: "natural-language",
        title: "Natural Language Picker",
        description: Some("Enter relative dates such as tomorrow, next Friday or in 2 weeks."),
        snippet: "examples/browser/snippets/date-picker-natural-language.md",
    },
    page::Block::Md("examples/browser/snippets/date-picker-api-reference.md"),
];
pub struct DatePickerPage {
    state: page::State,
    date: Date,
    window: f32,
}

impl DatePickerPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Self::at(page_index, today())
    }
    /// Build the same recipes relative to a supplied local day.
    pub fn at(page_index: usize, date: Date) -> Box<dyn page::Page> {
        Box::new(Self {
            date,
            window: 1440.,
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

impl page::Page for DatePickerPage {
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
        self.window = width;
    }
    fn backdrop(&self) -> page::Backdrop {
        page::Backdrop {
            padding: 40.into(),
            ..Default::default()
        }
    }
    fn preview(&self, id: &str) -> Element<'_, Message, Theme> {
        let compact = self.window < 768.;
        let size = if compact { 16. } else { 14. };
        let label = |s| text::label(s).line_height(1.375);
        let picker = || date_picker().default_month(self.date);
        match id {
            "demo" => picker().close_on_select(false).into(),
            "basic" => field(picker().width(Fill).chevron(false).close_on_select(false))
                .label(label("Date"))
                .width(176)
                .into(),
            "dob" => field(
                picker()
                    .width(Fill)
                    .chevron(false)
                    .placeholder("Select date")
                    .format("%-m/%-d/%Y")
                    .caption_dropdown(true),
            )
            .label(label("Date of birth"))
            .width(176)
            .into(),
            "range" => {
                let start = Date::from_ymd_opt(self.date.year(), 1, 20).unwrap_or(self.date);
                field(
                    picker()
                        .width(Fill)
                        .mode(Mode::Range)
                        .stacked(compact)
                        .default_value(Selection::range(start, start + Duration::days(20))),
                )
                .label(label("Date Picker Range"))
                .width(240)
                .into()
            }
            "input" => field(
                date_picker::input("June 01, 2025")
                    .text_size(size)
                    .default_value(Date::from_ymd_opt(2025, 6, 1).expect("valid example date")),
            )
            .label(label("Subscription Date"))
            .width(192)
            .into(),
            "time" => {
                let time_width = if compact { 99 } else { 90 };
                row![
                    field(
                        picker()
                            .width(Fill)
                            .placeholder("Select date")
                            .caption_dropdown(true)
                    )
                    .label(label("Date"))
                    .width(Fill),
                    field(input("HH:mm:ss").text_size(size).default_value("10:30:00"))
                        .label(label("Time"))
                        .width(time_width)
                ]
                .spacing(20)
                .width(320)
                .into()
            }
            "natural-language" => component(Schedule {
                initial: self.date + Duration::days(2),
                size,
            })
            .boxed(),
            _ => unreachable!("unknown Date Picker preview: {id}"),
        }
    }
}

struct Schedule {
    initial: Date,
    size: f32,
}
impl Component<'static, Message, Theme> for Schedule {
    type State = Option<Option<Date>>;
    type Event = Option<Date>;
    fn update(
        &self,
        state: &mut Self::State,
        date: Self::Event,
        _: &iced::Renderer,
    ) -> Option<Message> {
        *state = Some(date);
        None
    }
    fn view(&self, state: &Self::State) -> impl Widget<Self::Event, Theme> + 'static {
        schedule(state.unwrap_or(Some(self.initial)), self.size).compose()
    }
}
fn schedule(date: Option<Date>, size: f32) -> Slot<'static, Option<Date>> {
    let display = date
        .map(|d| d.format("%B %d, %Y").to_string())
        .unwrap_or_default();
    field(
        date_picker::input("Tomorrow or next week")
            .default_text("In 2 days")
            .parse_with(date_picker::relative)
            .caption_dropdown(true)
            .text_size(size)
            .value(date)
            .on_value_change(|date| date),
    )
    .label(text::label("Schedule Date").line_height(1.375))
    .description(format!("Your post will be published on {display}."))
    .width(320)
    .into()
}
