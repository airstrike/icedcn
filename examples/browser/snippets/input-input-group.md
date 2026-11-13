```rust
field(input_group(input("example.com"))
    .addon(input_group::addon(input_group::text("https://")))
    .addon(input_group::addon(icon::info()).align(input_group::Align::InlineEnd))).label("Website URL").width(320)
```
