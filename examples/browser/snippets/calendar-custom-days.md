```rust
calendar().mode(Mode::Range).caption_dropdown(true).cell_size(46.).default_selected(Selection::range(date(12,8),date(12,18))).annotate(|day| if day.weekday().number_from_monday()>=6 { "$120" } else { "$100" }.to_owned())
```
