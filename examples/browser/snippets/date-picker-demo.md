```rust
date_picker().width(256).label(|value|row![Slot::from(icon::calendar()), text::small(label(value,"Pick a date")),space::horizontal()].spacing(8).into())
```
