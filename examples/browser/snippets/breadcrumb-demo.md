```rust
use icedcn::Size;
use icedcn::View;
use icedcn::theme::tweak::{Fg, Token, hover};
use icedcn::widget::{breadcrumb, button, dropdown_menu, menu};

#[derive(Debug, Clone)]
enum Message {
    GoHome,
    GoComponents,
}

fn destinations<'a>(_: &dropdown_menu::Handle) -> View<'a, Message> {
    menu::group([
        menu::item("Documentation").into(),
        menu::item("Themes").into(),
        menu::item("GitHub").into(),
    ])
}

fn view<'a>() -> View<'a, Message> {
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
