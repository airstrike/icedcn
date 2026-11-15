```rust
use iced::{Element, Fill};
use icedcn::widget::calendar::{Date, Datelike, Duration, Selection, today};
use icedcn::widget::{Column, Row, button, calendar, column, container};
use icedcn::{Size, Theme, card};

#[derive(Clone, Debug)]
pub enum Edit {
    Pick(Option<Selection>),
    Show(Date),
    Offset(i64),
}

pub struct Dates {
    selected: Option<Selection>,
    month: Date,
}

impl Default for Dates {
    fn default() -> Self {
        let month = today();
        let february = Date::from_ymd_opt(month.year(), 2, 12).expect("valid date");
        Self {
            selected: Some(Selection::single(february)),
            month,
        }
    }
}

impl Dates {
    pub fn update(&mut self, edit: Edit) {
        match edit {
            Edit::Pick(value) => self.selected = value,
            Edit::Show(month) => self.month = month,
            Edit::Offset(days) => {
                self.month = today() + Duration::days(days);
                self.selected = Some(Selection::single(self.month));
            }
        }
    }

    pub fn view(&self) -> Element<'_, Edit, Theme> {
        let picker = calendar()
            .selected(self.selected)
            .month(self.month)
            .on_select(Edit::Pick)
            .on_month_change(Edit::Show)
            .fixed_weeks(true)
            .cell_size(38.)
            .padding(0.);
        let presets = [
            ("Today", 0),
            ("Tomorrow", 1),
            ("In 3 days", 3),
            ("In a week", 7),
            ("In 2 weeks", 14),
        ];
        let rows = presets.chunks(3).map(|chunk| {
            Row::with_children(chunk.iter().map(|&(label, days)| {
                button(label)
                    .style(button::Outline)
                    .size(Size::Sm)
                    .width(Fill)
                    .on_press(Edit::Offset(days))
                    .into()
            }))
            .spacing(8)
            .into()
        });
        card(
            column![
                container(picker).padding(12).center_x(Fill),
                card::footer_with_spacing(Column::with_children(rows).spacing(8), 12.),
            ]
            .spacing(0)
            .width(300),
        )
        .padding(0)
        .into()
    }
}
```
