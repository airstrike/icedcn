```rust
stack([
    input_group(input("Enter file name")).addon(end(dropdown_menu(
        |d| d.trigger(input_group::button(icon::ellipsis()).square()),
        |_| menu::group(["Settings","Copy path","Open location"].map(|item| menu::item(item).into())),
    ).align(dropdown_menu::Align::End))).into(),
    input_group(input("Enter search query")).addon(end(dropdown_menu(
        |d| d.trigger(input_group::button(row!["Search In...", icon::chevron_down()].spacing(4))),
        |_| menu::group(["Documentation","Blog Posts","Changelog"].map(|item| menu::item(item).into())),
    ).align(dropdown_menu::Align::End))).into(),
])
```
