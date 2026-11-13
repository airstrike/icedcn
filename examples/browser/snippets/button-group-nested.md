```rust
row![
    button(icon::plus()).square().style(button::Outline),
    input_group(input("Send a message...")).addon(input_group::addon(tooltip(icon::audio_lines(), "Voice Mode")).align(Align::InlineEnd))
].spacing(8).align_y(iced::Center)
```
