```rust
fn label(value:Option<Selection>, placeholder:&str)->String {
    value.map(|selection|selection.start.format("%B %-d, %Y").to_string()).unwrap_or_else(||placeholder.to_owned())
}

fn range_preview<'a>()->Element<'a,Message,Theme>{
    let from=Date::from_ymd_opt(today().year(),1,20).unwrap_or_else(today);
    field(date_picker().mode(Mode::Range).default_value(Selection::range(from,from+Duration::days(20))).width(288))
        .label("Date Picker Range").width(288).into()
}

struct Typed { natural:bool }
#[derive(Default)]
struct Entry {open:bool, value:Option<String>,selected:Option<Date>}
#[derive(Clone,Debug)]
enum Edit { Open(bool),Type(String),Pick(Option<Selection>) }

fn parse(value:&str,today:Date,natural:bool)->Option<Date> {
    let phrase=value.trim().to_lowercase();
    if natural {
        let offset=match phrase.as_str() {
            "today"=>Some(0),"tomorrow"=>Some(1),"yesterday"=>Some(-1),"next week"=>Some(7),_=>None,
        };
        if let Some(days)=offset {return today.checked_add_signed(Duration::days(days));}
        if let Some(next)=phrase.strip_prefix("next ") {
            let weekday=match next { "monday"=>Some(Weekday::Mon),"tuesday"=>Some(Weekday::Tue),"wednesday"=>Some(Weekday::Wed),
                "thursday"=>Some(Weekday::Thu),"friday"=>Some(Weekday::Fri),"saturday"=>Some(Weekday::Sat),"sunday"=>Some(Weekday::Sun),_=>None};
            if let Some(weekday)=weekday {
                let delta=(i64::from(weekday.num_days_from_monday()) - i64::from(today.weekday().num_days_from_monday())-1).rem_euclid(7)+1;
                return today.checked_add_signed(Duration::days(delta));
            }
        }
        let mut words=phrase.split_whitespace();
        if words.next()==Some("in") && let (Some(n),Some(unit),None)=(words.next(),words.next(),words.next()) {
            let n:i64=n.parse().ok()?;
            let multiplier=match unit { "day"|"days"=>1,"week"|"weeks"=>7,_=>return None };
            return today.checked_add_signed(Duration::try_days(n.checked_mul(multiplier)?)?);
        }
    }
    ["%Y-%m-%d","%B %d, %Y","%b %d, %Y","%m/%d/%Y"].iter().find_map(|format|Date::parse_from_str(value.trim(),format).ok())
}

impl Typed {
    fn initial(&self)->Date {
        if self.natural {today()+Duration::days(2)} else {Date::from_ymd_opt(2025,6,1).unwrap_or_else(today)}
    }
}

impl<'a> Component<'a,Message,Theme> for Typed {
    type State=Entry;
    type Event=Edit;
    fn update(&self,state:&mut Entry,edit:Edit,_:&iced::Renderer)->Option<Message>{
        match edit {
            Edit::Open(open)=>state.open=open,
            Edit::Type(value)=>{
                if let Some(date)=parse(&value,today(),self.natural) {state.selected=Some(date);}
                state.value=Some(value);
            }
            Edit::Pick(value)=>{
                state.selected=value.map(|s|s.start);
                state.value=Some(state.selected.map(|d|d.format("%B %d, %Y").to_string()).unwrap_or_default());
                state.open=false;
            }
        }
        None
    }
    fn view(&self,state:&Entry)->impl Widget<Edit,Theme>+'a {
        let selected=state.selected.unwrap_or_else(||self.initial());
        let value=state.value.clone().unwrap_or_else(|| if self.natural {"In 2 days".into()} else {selected.format("%B %d, %Y").to_string()});
        let popup=popover(|p|p.trigger(input_group::button(icon::calendar()).square()),move |_| calendar()
            .selected(Some(Selection::single(selected))).default_month(selected).on_select(Edit::Pick).into())
            .align(popover::Align::End).width(256.).open(state.open).on_open_change(Edit::Open);
        let (label,placeholder)=if self.natural{("Schedule Date","Tomorrow or next week")}else{("Subscription Date","June 01, 2025")};
        let mut control=field(input_group(input(placeholder).value(value).on_input(Edit::Type)).addon(input_group::addon(popup).align(input_group::Align::InlineEnd))).label(label).width(320);
        if self.natural {control=control.description(format!("Your post will be published on {}.",selected.format("%B %d, %Y")));}
        Element::from(control)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn interprets_relative_dates_across_month_and_year_boundaries(){
        let base=Date::from_ymd_opt(2025,12,31).unwrap();
        assert_eq!(parse("tomorrow",base,true),Date::from_ymd_opt(2026,1,1));
        assert_eq!(parse("next Wednesday",base,true),Date::from_ymd_opt(2026,1,7));
        assert_eq!(parse("in 2 weeks",base,true),Date::from_ymd_opt(2026,1,14));
        assert_eq!(parse("02/29/2026",base,false),None);
    }
}
```
