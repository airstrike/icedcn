```rust
use iced::Fill;
use icedcn::theme::container::Transparent;
use icedcn::theme::tweak::{Token, border};
use icedcn::widget::{column, container, context_menu, menu, text};
use icedcn::{Slot, radius};

fn view<'a>(bookmarks: bool, full_urls: bool, person: &'static str) -> Slot<'a, Message> {
    // `bookmarks`, `full_urls` and `person` live in your state.
    context_menu(
        |_| area(),
        move |_| {
            column![
                menu::group([
                    menu::item("Back").shortcut("⌘[").into(),
                    menu::item("Forward").shortcut("⌘]").disabled(true).into(),
                    menu::item("Reload").shortcut("⌘R").into(),
                    menu::sub(
                        "More Tools",
                        [
                            menu::item("Save Page...").into(),
                            menu::item("Create Shortcut...").into(),
                            menu::item("Name Window...").into(),
                            menu::separator(),
                            menu::item("Developer Tools").into(),
                            menu::separator(),
                            menu::item("Delete").style(menu::Destructive).into(),
                        ],
                    ),
                ]),
                menu::separator(),
                menu::group([
                    menu::checkbox_item("Show Bookmarks", bookmarks, Message::Bookmarks).into(),
                    menu::checkbox_item("Show Full URLs", full_urls, Message::FullUrls).into(),
                ]),
                menu::separator(),
                menu::group([
                    menu::label("People"),
                    menu::radio_item("Pedro Duarte", "pedro", Some(person), Message::Person).into(),
                    menu::radio_item("Colm Tuite", "colm", Some(person), Message::Person).into(),
                ]),
            ]
            .width(Fill)
            .into()
        },
    )
    .width(192)
    .into()
}

/// The area that responds to a secondary click.
fn area<'a>() -> Slot<'a, Message> {
    container(text("Right click here"))
        .center(Fill)
        .width(320)
        .height(180)
        .class(Transparent + border::Width(1.) + border::Color(Token::Border) + radius(radius::XL))
        .into()
}
```
