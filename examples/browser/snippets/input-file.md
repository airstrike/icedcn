```rust
field(input_group(input("").value(self.file_name.as_deref().unwrap_or("No file chosen")).width(Fill))
    .addon(input_group::addon(input_group::button("Choose File").on_press(Message::Page(pi, page::Message::ChooseFile)))))
    .label("Picture").description("Select a picture to upload.").width(320)
```
