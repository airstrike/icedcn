```rust
use icedcn::Size;
use icedcn::Slot;
use icedcn::theme::tweak::{Fg, Token, hover};
use icedcn::widget::{breadcrumb, button, dropdown_menu, menu};

#[derive(Debug, Clone)]
enum Message {
    GoHome,
    GoComponents,
}

fn destinations(_: &dropdown_menu::Handle) -> Slot<'static, Message> {
    menu::group([
        menu::item("Documentation").into(),
        menu::item("Themes").into(),
        menu::item("GitHub").into(),
    ])
}

fn view<'a>() -> Slot<'a, Message> {
    breadcrumb([
        breadcrumb::link("Home", Message::GoHome),
        breadcrumb::item(
            dropdown_menu(
                |m| {
                    m.trigger(
                        button(breadcrumb::ellipsis(None))
                            .size(Size::Sm)
                            .square()
                            .style(
                                button::Ghost
                                    + Fg(Token::MutedForeground)
                                    + hover(Fg(Token::Foreground)),
                            )
                            .translate(false),
                    )
                },
                destinations,
            )
            .align(dropdown_menu::Align::Start),
        ),
        breadcrumb::link("Components", Message::GoComponents),
        breadcrumb::page("Breadcrumb"),
    ])
    .into()
}
```
