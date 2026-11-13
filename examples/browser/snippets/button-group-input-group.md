```rust
struct Voice;
impl<'a> Component<'a, Message, Theme> for Voice {
    type State = bool;
    type Event = bool;
    fn update(&self, state: &mut bool, enabled: bool, _: &iced::Renderer) -> Option<Message> { *state = enabled; None }
    fn view(&self, enabled: &bool) -> impl Widget<bool, Theme> + 'a {
        let toggle = input_group::button(icon::audio_lines()).square().on_press(!*enabled);
        let toggle = if *enabled {toggle.style(button::Secondary)} else {toggle};
        Element::from(row![button(icon::plus()).rounded().square().style(button::Outline),
            input_group(input(if *enabled { "Record and send audio..." } else { "Send a message..." }).disabled(*enabled))
            .addon(input_group::addon(tooltip(toggle, "Voice Mode")).align(Align::InlineEnd))
        ].spacing(8).align_y(iced::Center))
    }
}

```
