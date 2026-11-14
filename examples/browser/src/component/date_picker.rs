use crate::icon;
use crate::{Message, page};
use iced::Widget;
use iced::widget::component::{Component, component};
use iced::{Element, Task};
use icedcn::Slot;
use icedcn::Theme;
use icedcn::widget::calendar::{Date, Datelike, Duration, Mode, Selection, Weekday, today};
use icedcn::widget::{
    calendar, date_picker, field, input, input_group, popover, row, space, text,
};

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
    page::Block::Md("examples/browser/snippets/date-picker-composition.md"),
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "basic",
                title: "Basic",
                description: None,
                snippet: "examples/browser/snippets/date-picker-basic.md",
            },
            page::Block::Example {
                id: "range",
                title: "Range Picker",
                description: None,
                snippet: "examples/browser/snippets/date-picker-range.md",
            },
            page::Block::Example {
                id: "dob",
                title: "Date of Birth",
                description: None,
                snippet: "examples/browser/snippets/date-picker-dob.md",
            },
            page::Block::Example {
                id: "input",
                title: "Input",
                description: None,
                snippet: "examples/browser/snippets/date-picker-input.md",
            },
            page::Block::Example {
                id: "time",
                title: "Time Picker",
                description: None,
                snippet: "examples/browser/snippets/date-picker-time.md",
            },
            page::Block::Example {
                id: "natural-language",
                title: "Natural Language Picker",
                description: None,
                snippet: "examples/browser/snippets/date-picker-natural-language.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/date-picker-api-reference.md"),
];
pub struct DatePickerPage {
    state: page::State,
}

impl DatePickerPage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
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

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        match id {
            "demo" => date_picker()
                .width(256)
                .label(|value| {
                    row![
                        Slot::from(icon::calendar()),
                        text::small(label(value, "Pick a date")),
                        space::horizontal()
                    ]
                    .spacing(8)
                    .into()
                })
                .into(),
            "basic" => field(date_picker().label(|value| {
                row![
                    text::small(label(value, "Pick a date")),
                    space::horizontal(),
                    icon::chevron_down()
                ]
                .spacing(8)
                .into()
            }))
            .label("Date")
            .into(),
            "range" => range_preview().into(),
            "dob" => field(
                date_picker()
                    .caption_dropdown(true)
                    .placeholder("Select date")
                    .label(|value| {
                        row![
                            text::small(
                                value
                                    .map(|s| s.start.format("%-m/%-d/%Y").to_string())
                                    .unwrap_or_else(|| "Select date".into())
                            ),
                            space::horizontal(),
                            icon::chevron_down()
                        ]
                        .into()
                    }),
            )
            .label("Date of birth")
            .into(),
            "input" => component(Typed { natural: false }).boxed().into(),
            "time" => row![
                field(date_picker().placeholder("Select date")).label("Date"),
                field(input("HH:mm:ss").default_value("10:30:00"))
                    .label("Time")
                    .width(128)
            ]
            .spacing(12)
            .into(),
            "natural-language" => component(Typed { natural: true }).boxed().into(),
            _ => unreachable!("unknown preview: {id}"),
        }
    }
}

fn label(value: Option<Selection>, placeholder: &str) -> String {
    value
        .map(|selection| selection.start.format("%B %-d, %Y").to_string())
        .unwrap_or_else(|| placeholder.to_owned())
}

fn range_preview<'a>() -> Element<'a, Message, Theme> {
    let from = Date::from_ymd_opt(today().year(), 1, 20).unwrap_or_else(today);
    field(
        date_picker()
            .mode(Mode::Range)
            .default_value(Selection::range(from, from + Duration::days(20)))
            .width(288),
    )
    .label("Date Picker Range")
    .width(288)
    .into()
}

struct Typed {
    natural: bool,
}
#[derive(Default)]
struct Entry {
    open: bool,
    value: Option<String>,
    selected: Option<Date>,
}
#[derive(Clone, Debug)]
enum Edit {
    Open(bool),
    Type(String),
    Pick(Option<Selection>),
}

fn parse(value: &str, today: Date, natural: bool) -> Option<Date> {
    let phrase = value.trim().to_lowercase();
    if natural {
        let offset = match phrase.as_str() {
            "today" => Some(0),
            "tomorrow" => Some(1),
            "yesterday" => Some(-1),
            "next week" => Some(7),
            _ => None,
        };
        if let Some(days) = offset {
            return today.checked_add_signed(Duration::days(days));
        }
        if let Some(next) = phrase.strip_prefix("next ") {
            let weekday = match next {
                "monday" => Some(Weekday::Mon),
                "tuesday" => Some(Weekday::Tue),
                "wednesday" => Some(Weekday::Wed),
                "thursday" => Some(Weekday::Thu),
                "friday" => Some(Weekday::Fri),
                "saturday" => Some(Weekday::Sat),
                "sunday" => Some(Weekday::Sun),
                _ => None,
            };
            if let Some(weekday) = weekday {
                let delta = (i64::from(weekday.num_days_from_monday())
                    - i64::from(today.weekday().num_days_from_monday())
                    - 1)
                .rem_euclid(7)
                    + 1;
                return today.checked_add_signed(Duration::days(delta));
            }
        }
        let mut words = phrase.split_whitespace();
        if words.next() == Some("in")
            && let (Some(n), Some(unit), None) = (words.next(), words.next(), words.next())
        {
            let n: i64 = n.parse().ok()?;
            let multiplier = match unit {
                "day" | "days" => 1,
                "week" | "weeks" => 7,
                _ => return None,
            };
            return today.checked_add_signed(Duration::try_days(n.checked_mul(multiplier)?)?);
        }
    }
    ["%Y-%m-%d", "%B %d, %Y", "%b %d, %Y", "%m/%d/%Y"]
        .iter()
        .find_map(|format| Date::parse_from_str(value.trim(), format).ok())
}

impl Typed {
    fn initial(&self) -> Date {
        if self.natural {
            today() + Duration::days(2)
        } else {
            Date::from_ymd_opt(2025, 6, 1).unwrap_or_else(today)
        }
    }
}

impl<'a> Component<'a, Message, Theme> for Typed {
    type State = Entry;
    type Event = Edit;
    fn update(&self, state: &mut Entry, edit: Edit, _: &iced::Renderer) -> Option<Message> {
        match edit {
            Edit::Open(open) => state.open = open,
            Edit::Type(value) => {
                if let Some(date) = parse(&value, today(), self.natural) {
                    state.selected = Some(date);
                }
                state.value = Some(value);
            }
            Edit::Pick(value) => {
                state.selected = value.map(|s| s.start);
                state.value = Some(
                    state
                        .selected
                        .map(|d| d.format("%B %d, %Y").to_string())
                        .unwrap_or_default(),
                );
                state.open = false;
            }
        }
        None
    }
    fn view(&self, state: &Entry) -> impl Widget<Edit, Theme> + 'a {
        let selected = state.selected.unwrap_or_else(|| self.initial());
        let value = state.value.clone().unwrap_or_else(|| {
            if self.natural {
                "In 2 days".into()
            } else {
                selected.format("%B %d, %Y").to_string()
            }
        });
        let popup = popover(
            |p| p.trigger(input_group::button(icon::calendar()).square()),
            move |_| {
                calendar()
                    .selected(Some(Selection::single(selected)))
                    .default_month(selected)
                    .on_select(Edit::Pick)
                    .into()
            },
        )
        .align(popover::Align::End)
        .width(256.)
        .open(state.open)
        .on_open_change(Edit::Open);
        let (label, placeholder) = if self.natural {
            ("Schedule Date", "Tomorrow or next week")
        } else {
            ("Subscription Date", "June 01, 2025")
        };
        let mut control = field(
            input_group(input(placeholder).value(value).on_input(Edit::Type))
                .addon(input_group::addon(popup).align(input_group::Align::InlineEnd)),
        )
        .label(label)
        .width(320);
        if self.natural {
            control = control.description(format!(
                "Your post will be published on {}.",
                selected.format("%B %d, %Y")
            ));
        }
        Element::from(control)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn interprets_relative_dates_across_month_and_year_boundaries() {
        let base = Date::from_ymd_opt(2025, 12, 31).unwrap();
        assert_eq!(
            parse("tomorrow", base, true),
            Date::from_ymd_opt(2026, 1, 1)
        );
        assert_eq!(
            parse("next Wednesday", base, true),
            Date::from_ymd_opt(2026, 1, 7)
        );
        assert_eq!(
            parse("in 2 weeks", base, true),
            Date::from_ymd_opt(2026, 1, 14)
        );
        assert_eq!(parse("02/29/2026", base, false), None);
    }
}
