```rust
row![
            badge!["Open Link", icon::arrow_up_right().size(12)]
                .icon_end()
                .on_press(page::Message::LinkClicked("#link".into())),
        ]
        .spacing(8)
        .wrap()
        .into()
```
