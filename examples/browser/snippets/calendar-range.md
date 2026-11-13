```rust
calendar().mode(Mode::Range).number_of_months(2).default_selected(Selection::range(date(1,12), date(1,12) + Duration::days(30))).unavailable(|day| day>today() || day.year()<1900)
```
