```rust
container(input_group(input("Search..."))
    .addon(input_group::addon(icon::search()))
    .addon(input_group::addon(kbd_group![kbd("⌘"), kbd("K")]).align(Align::InlineEnd)))
    .max_width(320).width(iced::Fill)
```
