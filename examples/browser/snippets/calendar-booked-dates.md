```rust
calendar().default_selected(Selection::single(date(2,3))).unavailable(|day| (date(2,12)..=date(2,26)).contains(&day))
```
