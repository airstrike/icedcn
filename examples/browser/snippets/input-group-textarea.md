```rust
stack([input_group(textarea("console.log('Hello, world!');").min_height(200.))
    .addon(input_group::addon(row![icon::file_code(), input_group::text("main.rs"), space::horizontal(),
        input_group::button(icon::refresh_ccw()).square(), input_group::button(icon::copy()).square()].spacing(4)).align(Align::BlockStart))
    .addon(input_group::addon(row![input_group::text("Line 1, Column 1"),space::horizontal(),
        input_group::button(row!["Run",icon::corner_down_left()].spacing(4)).style(button::Default)]).align(Align::BlockEnd)).into()])
```
