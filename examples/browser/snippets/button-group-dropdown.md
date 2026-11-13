```rust
dropdown_menu(|&m| button_group![
    button("Follow").style(button::Outline),
    group::decorated(button(icon::chevron_down()).style(button::Outline), move |b| m.trigger(b))
].into(), |_| column![
    menu::item(row![icon::volume_off().size(16), "Mute Conversation"].spacing(8)),
    menu::item(row![icon::check().size(16), "Mark as Read"].spacing(8)),
    menu::item(row![icon::triangle_alert().size(16), "Report Conversation"].spacing(8)),
    menu::item(row![icon::user_round_x().size(16), "Block User"].spacing(8)),
    menu::item(row![icon::share().size(16), "Share Conversation"].spacing(8)),
    menu::item(row![icon::copy().size(16), "Copy Conversation"].spacing(8)),
    menu::separator(),
    menu::item(row![icon::trash_2().size(16), "Delete Conversation"].spacing(8)).style(menu::Destructive)
].width(Fill).into())
    .align(dropdown_menu::Align::End).width(176)
```
