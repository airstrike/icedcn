```rust
row![button_group![
    group::Item::Content(select([("$", "$ US Dollar"), ("€", "€ Euro"), ("£", "£ British Pound")].map(|(v,label)| select::item(v,label))).default_value("$").format_value(|value,_|value.to_owned()).width(52).icon(icon::chevron_down).into()),
    input("10.00").width(120)
], button(icon::arrow_right()).square().style(button::Outline)].spacing(8)
```
