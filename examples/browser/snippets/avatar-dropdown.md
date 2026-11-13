```rust
dropdown_menu(
    |m| m.trigger(button(avatar(Some(shadcn_handle())).fallback("CN")).style(button::Ghost).square().rounded()),
    |_| column![menu::item("Profile"), menu::item("Billing"), menu::item("Settings"), menu::separator(), menu::item("Log out").style(menu::Destructive)].width(iced::Fill).into(),
).width(128)
```
