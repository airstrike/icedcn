```rust
use iced::Fill;
use icedcn::View;
use icedcn::widget::{field, input};

fn credentials<'a, M: Clone + 'a>(text_size: f32) -> View<'a, M> {
    field::group([
        field(input("Max Leiter").width(Fill).text_size(text_size))
            .label("Username")
            .description("Choose a unique username for your account.")
            .into(),
        field(
            input("••••••••")
                .width(Fill)
                .text_size(text_size)
                .secure(true),
        )
        .label("Password")
        .description("Must be at least 8 characters long.")
        .description_above()
        .into(),
    ])
    .width(320)
    .into()
}
```

Use 14 px input text on desktop or 16 px on a narrow viewport.
