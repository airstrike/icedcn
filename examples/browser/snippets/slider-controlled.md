```rust
struct Temperature;
impl<'a> Component<'a, Message, Theme> for Temperature {
    type State = Option<[f32; 2]>;
    type Event = [f32; 2];

    fn update(
        &self,
        state: &mut Self::State,
        values: Self::Event,
        _: &iced::Renderer,
    ) -> Option<Message> {
        *state = Some(values);
        None
    }
    fn view(&self, state: &Self::State) -> impl Widget<Self::Event, Theme> + 'a {
        let values = state.unwrap_or([0.3, 0.7]);
        Element::from(
            column![
                row![
                    text::label("Temperature"),
                    space::horizontal(),
                    text::muted(format!("{}, {}", values[0], values[1]))
                ]
                .width(Fill),
                slider::range(0.0..=1.0)
                    .value(values)
                    .step(0.1)
                    .on_value_change(|values| values),
            ]
            .spacing(12)
            .max_width(320.0)
            .width(Fill),
        )
    }
}
```
