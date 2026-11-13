```rust
fn date(month:u32, day:u32) -> Date {
    Date::from_ymd_opt(today().year(),month,day).unwrap_or_else(today)
}

fn presets<'a>() -> Element<'a, Message, Theme> { component(Presets).boxed() }
struct Presets;

#[derive(Default)]
struct Dates { value: Option<Option<Selection>>, month:Option<Date> }

#[derive(Clone,Debug)]
enum Edit { Pick(Option<Selection>), Show(Date), Offset(i64) }

impl<'a> Component<'a, Message, Theme> for Presets {
    type State=Dates;
    type Event=Edit;
    fn update(&self,state:&mut Dates,edit:Edit,_:&iced::Renderer)->Option<Message>{
        match edit {
            Edit::Pick(value) => state.value=Some(value),
            Edit::Show(month) => state.month=Some(month),
            Edit::Offset(days) => {
                let day=today()+Duration::days(days);
                state.value=Some(Some(Selection::single(day))); state.month=Some(day);
            }
        }
        None
    }
    fn view(&self,state:&Dates)->impl Widget<Edit, Theme>+'a {
        let picker=calendar().selected(state.value.unwrap_or(Some(Selection::single(date(2,12)))))
            .month(state.month.unwrap_or_else(today)).on_select(Edit::Pick).on_month_change(Edit::Show);
        let presets = [("Today",0),("Tomorrow",1),("In 3 days",3),("In a week",7),("In 2 weeks",14)];
        let actions = presets.chunks(3).map(|chunk| {
            icedxdy::widget::Row::with_children(chunk.iter().map(|&(label,days)|
                button(label).style(button::Outline).size(icedxdy::Size::Xs).on_press(Edit::Offset(days)).into()
            )).spacing(4).into()
        });
        Element::from(column![picker,icedxdy::widget::Column::with_children(actions).spacing(4)].padding(8))
    }
}
```
