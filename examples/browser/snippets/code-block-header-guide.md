A long title truncates. To open the file from the title, wrap `code_block::title` in a button.

```rust
code_block::header![
    button(code_block::title("src/view.rs"))
        .style(button::Link)
        .on_press(Message::Open),
]
```
