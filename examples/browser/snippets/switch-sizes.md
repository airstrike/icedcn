```rust
field::group([
    field(switch().size(switch::Size::Sm)).orientation(Horizontal).label("Small").into(),
    field(switch()).orientation(Horizontal).label("Default").into(),
])
.width(160)
```
