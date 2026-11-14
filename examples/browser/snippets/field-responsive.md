```rust
use icedcn::Slot;
use icedcn::widget::{button, field};

fn profile<M: Clone + 'static>(name: field::Field<'static, M>, submit: M) -> Slot<'static, M> {
    field::set(
        field::legend("Profile"),
        Some("Fill in your profile information."),
        field::group([
            name.responsive().description("Provide your full name for identification").into(),
            field::actions([
                button("Submit").on_press(submit).into(),
                button("Cancel").style(button::Outline).into(),
            ])
            .into(),
        ]),
    )
    .max_width(512.)
    .into()
}
```

Give the input a preferred width, such as `.width(192)`, and pass its labeled field and submit message. The compact layout stretches the control to the available width. Use the same message with the input’s `.on_submit(...)` for keyboard submission.
