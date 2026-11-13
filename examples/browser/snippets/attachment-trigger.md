```rust
dialog(|handle| {
    let opener = *handle;
    attachment("research-summary.pdf")
        .description("Open preview dialog")
        .media(icon::file_search())
        .trigger(move |body| opener.trigger(preview_button(body, None)))
        .action(attachment::action(icon::copy()).on_press(Edit::Copy))
        .action(remove("research-summary.pdf"))
        .into()
}, |_| dialog::header("research-summary.pdf", "Preview the selected file."))
```
