```rust
row![
            badge![icon::badge_check().size(12), "Verified"]
                .icon_start()
                .style(badge::Secondary),
            badge!["Bookmark", icon::bookmark().size(12)]
                .icon_end()
                .style(badge::Outline),
        ]
        .spacing(8)
        .wrap()
        .into()
```
