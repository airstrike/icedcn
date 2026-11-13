```rust
fn people() -> [(&'static str, &'static str, image::Handle); 3] {
    [("shadcn", "shadcn@vercel.com", shadcn_handle()), ("maxleiter", "maxleiter@vercel.com", maxleiter_handle()), ("evilrabbit", "evilrabbit@vercel.com", evilrabbit_handle())]
}

dropdown_menu(
    |m| m.trigger(button!["Select", icon::chevron_down()].style(button::Outline)),
    |_| menu::group(people().map(|(name,email,portrait)| menu::item(item::item([
        item::media(avatar(Some(portrait)).fallback(&name[..1]).size(26.0)).into(),
        item::content([item::title(name).into(), item::description(email).into()]).spacing(0).into()
    ]).size(item::Size::Xs)).into()))
).width(192).align(dropdown_menu::Align::End)
```
