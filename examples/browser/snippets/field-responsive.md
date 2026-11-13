```rust
field::set(field::legend("Profile"), Some("Fill in your profile information."), field::group([
    field(input("Evil Rabbit")).responsive().label("Name").description("Provide your full name for identification").into(),
    field::actions([button("Submit").into(), button("Cancel").style(button::Outline).into()]).into(),
])).max_width(512.)
```
