```rust
use iced::widget::component::{Component, component};
use iced::{Element, Fill, Pixels, Widget};
use icedcn::core::Border;
use icedcn::theme::container::Style;
use icedcn::widget::calendar::{Date, Datelike, Duration, Mode, Selection, System, today};
use icedcn::widget::{
    Column, Row, button, calendar, column, container, field, input, input_group, rule, text,
};
use icedcn::{Size, Slot, Theme, card};

pub fn date(month: u32, day: u32) -> Date {
    Date::from_ymd_opt(today().year(), month, day).unwrap_or_else(today)
}
pub fn presets<Message: 'static>() -> Element<'static, Message, Theme> {
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
impl<'a, Message: 'static> Component<'a, Message, Theme> for Presets {
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
            .weekday_line_height(18.285715)
            .padding(0.)
            .cell_size(38.)
            .fixed_weeks(true)
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
                card::footer_with_spacing(Column::with_children(actions).spacing(8), 12.),
            ]
            .width(300),
        )
        .boxed()
    }
}
```
