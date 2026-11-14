```rust
stack([
    field(input_group(input("Enter your name")).addon(input_group::addon(input_group::text("Full Name")).align(Align::BlockStart)))
        .label("Input").description("Header positioned above the input.").into(),
    field(input_group(textarea("console.log('Hello, world!');"))
        .addon(input_group::addon(row![icon::file_code(), input_group::text("main.rs"), space::horizontal(),
            input_group::button(icon::copy()).square()].spacing(8)).align(Align::BlockStart)))
        .label("Textarea").description("Header positioned above the textarea.").into(),
])
```
