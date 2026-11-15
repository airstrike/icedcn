```rust
struct FontWeight;
impl<'a> Component<'a, Message, Theme> for FontWeight {
    type State = Option<String>;
    type Event = Vec<String>;

    fn update(
        &self,
        state: &mut Self::State,
        selection: Self::Event,
        _: &iced::Renderer,
    ) -> Option<Message> {
        *state = Some(selection.into_iter().next().unwrap_or_default());
        None
    }

    fn view(&self, state: &Self::State) -> impl Widget<Self::Event, Theme> + 'a {
        let selected = state.as_deref().unwrap_or("normal");
        let group = toggle_group(|| {
            [
                ("Light", Weight::Light),
                ("Normal", Weight::Normal),
                ("Medium", Weight::Medium),
                ("Bold", Weight::Bold),
            ]
            .map(|(name, weight)| {
                toggle_group::item(
                    name.to_lowercase(),
                    column![
                        text::Text::new("Aa")
                            .size(24.0)
                            .line_height(1.0)
                            .weight(weight),
                        text::muted(name).size(12.0)
                    ]
                    .align_x(iced::Center),
                )
                .square(64)
            })
        })
        .value([selected])
        .on_value_change(|values| values)
        .style(toggle_group::Outline)
        .spacing(8)
        .size(Size::Lg);
        let description = row![
            text::muted("Use"),
            text::inline_code(format!("font-{selected}")),
            text::muted("to set the font weight.")
        ]
        .spacing(4);
        Element::from(
            field(group)
                .label("Font Weight")
                .description(Slot::from(description))
                .width(iced::Shrink),
        )
    }
}
```
