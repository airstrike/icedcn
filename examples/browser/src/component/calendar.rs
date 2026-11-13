use crate::{Message, page};
use iced::Widget;
use iced::widget::component::{Component, component};
use iced::{Element, Task};
use icedxdy::Size;
use icedxdy::Theme;
use icedxdy::widget::calendar::{Date, Datelike, Duration, Mode, Selection, System, today};
use icedxdy::widget::{Column, Row, button, calendar, column, field, input, row};

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
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "hijri",
                title: "Persian / Hijri / Jalali Calendar",
                description: None,
                snippet: "examples/browser/snippets/calendar-hijri.md",
            },
            page::Block::Example {
                id: "basic",
                title: "Basic",
                description: None,
                snippet: "examples/browser/snippets/calendar-basic.md",
            },
            page::Block::Example {
                id: "range",
                title: "Range Calendar",
                description: None,
                snippet: "examples/browser/snippets/calendar-range.md",
            },
            page::Block::Example {
                id: "caption",
                title: "Month and Year Selector",
                description: None,
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
            page::Block::Example {
                id: "week-numbers",
                title: "Week Numbers",
                description: None,
                snippet: "examples/browser/snippets/calendar-week-numbers.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/calendar-api-reference.md"),
];
pub struct CalendarPage {
    state: page::State,
}

impl CalendarPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
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
            "demo" => calendar()
                .default_selected(Selection::single(today()))
                .caption_dropdown(true)
                .into(),
            "hijri" => calendar()
                .system(System::Persian)
                .default_selected(Selection::single(
                    Date::from_ymd_opt(2025, 6, 12).unwrap_or_else(today),
                ))
                .into(),
            "basic" => calendar().into(),
            "range" => calendar()
                .mode(Mode::Range)
                .number_of_months(2)
                .default_selected(Selection::range(
                    date(1, 12),
                    date(1, 12) + Duration::days(30),
                ))
                .unavailable(|day| day > today() || day.year() < 1900)
                .into(),
            "caption" => calendar().caption_dropdown(true).into(),
            "presets" => presets().into(),
            "time" => column![
                calendar().default_selected(Selection::single(date(today().month(), 12))),
                row![
                    field(input("HH:mm:ss").default_value("10:30:00")).label("Start Time"),
                    field(input("HH:mm:ss").default_value("12:30:00")).label("End Time")
                ]
                .spacing(8)
                .width(244)
            ]
            .spacing(8)
            .into(),
            "booked-dates" => calendar()
                .default_selected(Selection::single(date(2, 3)))
                .unavailable(|day| (date(2, 12)..=date(2, 26)).contains(&day))
                .into(),
            "custom-days" => calendar()
                .mode(Mode::Range)
                .caption_dropdown(true)
                .cell_size(46.)
                .default_selected(Selection::range(date(12, 8), date(12, 18)))
                .annotate(|day| {
                    if day.weekday().number_from_monday() >= 6 {
                        "$120"
                    } else {
                        "$100"
                    }
                    .to_owned()
                })
                .into(),
            "week-numbers" => calendar()
                .default_selected(Selection::single(date(2, 3)))
                .week_numbers(true)
                .into(),
            _ => unreachable!("unknown preview: {id}"),
        }
    }
}

fn date(month: u32, day: u32) -> Date {
    Date::from_ymd_opt(today().year(), month, day).unwrap_or_else(today)
}

fn presets<'a>() -> Element<'a, Message, Theme> {
    component(Presets).boxed()
}
struct Presets;

#[derive(Default)]
struct Dates {
    value: Option<Option<Selection>>,
    month: Option<Date>,
}

#[derive(Clone, Debug)]
enum Edit {
    Pick(Option<Selection>),
    Show(Date),
    Offset(i64),
}

impl<'a> Component<'a, Message, Theme> for Presets {
    type State = Dates;
    type Event = Edit;
    fn update(&self, state: &mut Dates, edit: Edit, _: &iced::Renderer) -> Option<Message> {
        match edit {
            Edit::Pick(value) => state.value = Some(value),
            Edit::Show(month) => state.month = Some(month),
            Edit::Offset(days) => {
                let day = today() + Duration::days(days);
                state.value = Some(Some(Selection::single(day)));
                state.month = Some(day);
            }
        }
        None
    }
    fn view(&self, state: &Dates) -> impl Widget<Edit, Theme> + 'a {
        let picker = calendar()
            .selected(state.value.unwrap_or(Some(Selection::single(date(2, 12)))))
            .month(state.month.unwrap_or_else(today))
            .on_select(Edit::Pick)
            .on_month_change(Edit::Show);
        let presets = [
            ("Today", 0),
            ("Tomorrow", 1),
            ("In 3 days", 3),
            ("In a week", 7),
            ("In 2 weeks", 14),
        ];
        let actions = presets.chunks(3).map(|chunk| {
            Row::with_children(chunk.iter().map(|&(label, days)| {
                button(label)
                    .style(button::Outline)
                    .size(Size::Xs)
                    .on_press(Edit::Offset(days))
                    .into()
            }))
            .spacing(4)
            .into()
        });
        Element::from(column![picker, Column::with_children(actions).spacing(4)].padding(8))
    }
}
