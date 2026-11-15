```rust
container(
    slider::range(0.0..=100.0)
        .default_value([25.0, 50.0])
        .step(5),
)
.width(Fill)
.max_width(320)
```
