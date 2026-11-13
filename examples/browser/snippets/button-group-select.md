```rust
row![button_group![
    group::Item::Content(select([("$", "$ US Dollar"), ("€", "€ Euro"), ("£", "£ British Pound")].map(|(v,label)| select::item(v,label))).default_value("$").width(128).icon(icon::chevron_down).into()),
    group::Item::Content(input("10.00").width(120).into())
], button(icon::arrow_right()).square().style(button::Outline)].spacing(8)
```
