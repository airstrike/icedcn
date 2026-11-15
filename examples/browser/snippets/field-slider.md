```rust
use iced::widget::component::Component;
use iced::{Element, Widget};
use icedcn::Theme;
use icedcn::widget::{field, slider};

struct Budget;

impl<'a, Message: 'a> Component<'a, Message, Theme> for Budget {
    type State = Option<[f32; 2]>;
    type Event = [f32; 2];

    fn update(
        &self,
        state: &mut Self::State,
        next: Self::Event,
        _: &iced::Renderer,
    ) -> Option<Message> {
        *state = Some(next);
        None
    }

    fn view(&self, state: &Self::State) -> impl Widget<Self::Event, Theme> + 'a {
        let [low, high] = state.unwrap_or([200., 800.]);
        Element::from(
            field(
                slider::range(0.0..=1000.0)
                    .step(10.)
                    .value([low, high])
                    .on_value_change(|values| values),
            )
            .label("Price Range")
            .description(format!("Set your budget range (${low:.0} - {high:.0})."))
            .description_above()
            .width(320),
        )
    }
}
```

Mount the budget field with `iced::widget::component::component(Budget)`.
