```rust
field::set(field::legend("Address Information"), Some("We need your address to deliver your order."),
    field::group([
        field(input("123 Main St")).label("Street Address").into(),
        row![field(input("New York")).label("City").width(Fill), field(input("90502")).label("Postal Code").width(Fill)].spacing(16).into(),
    ])).max_width(384.)
```
