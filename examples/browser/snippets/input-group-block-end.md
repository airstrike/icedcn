```rust
stack([
    field(input_group(input("Enter amount")).addon(input_group::addon(input_group::text("USD")).align(Align::BlockEnd)))
        .label("Input").description("Footer positioned below the input.").into(),
    field(input_group(textarea("Write a comment...")).addon(input_group::addon(row![
        input_group::text("0/280"), space::horizontal(), input_group::button("Post").style(button::Default)
    ]).align(Align::BlockEnd))).label("Textarea").description("Footer positioned below the textarea.").into(),
])
```
