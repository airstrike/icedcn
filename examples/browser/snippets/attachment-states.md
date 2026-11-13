```rust
attachment("financial-model.xlsx")
    .media(icon::file_warning())
    .state(State::Error)
    .description("Upload failed. Try again.")
    .action(attachment::action(icon::refresh_ccw()).on_press(Edit::Retry))
    .action(remove("financial-model.xlsx"))
```
