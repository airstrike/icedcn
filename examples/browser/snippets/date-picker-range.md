```rust
fn label(value:Option<Selection>, placeholder:&str)->String {
    value.map(|selection|selection.start.format("%B %-d, %Y").to_string()).unwrap_or_else(||placeholder.to_owned())
}

fn range_preview<'a>()->Element<'a,Message,Theme>{
    let from=Date::from_ymd_opt(today().year(),1,20).unwrap_or_else(today);
    field(date_picker().mode(Mode::Range).default_value(Selection::range(from,from+Duration::days(20))).width(288))
        .label("Date Picker Range").width(288).into()
}
```
