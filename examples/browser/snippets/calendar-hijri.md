```rust
calendar().system(System::Persian).default_selected(Selection::single(Date::from_ymd_opt(2025,6,12).unwrap_or_else(today)))
```
