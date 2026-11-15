```rust
use crate::icon;
use iced::{Element, Fill, Pixels, Widget};
use icedcn::theme::text::muted;
use icedcn::widget::calendar::{Datelike, Selection, today};
use icedcn::widget::{calendar, column, container, field, input, input_group, rule, text};
use icedcn::{Theme, card};

#[derive(Clone, Debug)]
pub enum Edit {
    Start(String),
    End(String),
}

pub struct Hours {
    start: String,
    end: String,
}

impl Default for Hours {
    fn default() -> Self {
        Self {
            start: "10:30:00".into(),
            end: "12:30:00".into(),
        }
    }
}

impl Hours {
    pub fn update(&mut self, edit: Edit) {
        let (target, value) = match edit {
            Edit::Start(value) => (&mut self.start, value),
            Edit::End(value) => (&mut self.end, value),
        };
        let valid = value.split(':').enumerate().all(|(i, part)| {
            i < 3
                && part.len() <= 2
                && part.bytes().all(|b| b.is_ascii_digit())
                && (part.is_empty()
                    || part
                        .parse::<u8>()
                        .is_ok_and(|n| n < if i == 0 { 24 } else { 60 }))
        });
        if valid {
            *target = value;
        }
    }

    pub fn view(&self, window_width: f32) -> Element<'_, Edit, Theme> {
        let time_field = |label: &'static str, value: &str, edit: fn(String) -> Edit| {
            let control = input("HH:mm:ss")
                .value(value.to_owned())
                .on_input(edit)
                .text_size(if window_width < 768. { 16. } else { 14. });
            field(
                input_group(control).addon(input_group::addon(
                    icon::clock()
                        .size(16)
                        .line_height(1.0)
                        .style(|theme| muted(theme).into()),
                )),
            )
            .label(text::label(label).line_height(Pixels(19.25)))
        };
        let start = time_field("Start Time", &self.start, Edit::Start);
        let end = time_field("End Time", &self.end, Edit::End);
        let day = today().with_day(12).expect("every month contains the 12th");
        let picker = calendar()
            .padding(0.)
            .default_selected(Selection::single(day));
        card(
            column![
                container(picker).padding(12),
                rule::horizontal(1).boxed(),
                container(field::group([start.into(), end.into()]))
                    .width(Fill)
                    .padding(12),
            ]
            .spacing(0)
            .width(220),
        )
        .padding(0)
        .into()
    }
}
```
