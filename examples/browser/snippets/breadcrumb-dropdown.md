```rust
use crate::icon;
use icedcn::View;
use icedcn::Weight;
use icedcn::theme::text::muted;
use icedcn::theme::tweak::{Bg, Fg, Token};
use icedcn::widget::{breadcrumb, button, dropdown_menu, menu, row};

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
                        button(
                            row![
                                "Components",
                                icon::chevron_down()
                                    .size(14)
                                    .line_height(1.0)
                                    .style(|theme| muted(theme).into())
                            ]
                            .spacing(4),
                        )
                        .height(20)
                        .padding(0)
                        .weight(Weight::Normal)
                        .translate(false)
                        .focus_ring(2, |theme| theme.neutral.foreground)
                        .style(
                            button::Ghost
                                + Bg(icedcn::Color::TRANSPARENT.into())
                                + Fg(Token::MutedForeground),
                        ),
                    )
                },
                destinations,
            )
            .align(dropdown_menu::Align::Start),
        ),
        breadcrumb::page("Breadcrumb"),
    ])
    .separator(|| {
        icon::dot()
            .size(14)
            .line_height(1.0)
            .style(|theme| muted(theme).into())
            .into()
    })
    .into()
}
```
