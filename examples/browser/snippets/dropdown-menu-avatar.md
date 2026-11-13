```rust
use icedxdy::widget::{avatar, button, dropdown_menu, menu};

dropdown_menu(
    |m| {
        m.trigger(
            button(avatar(Some(shadcn_handle())).fallback("LR"))
                .square()
                .rounded()
                .translate(false)
                .style(button::Ghost),
        )
    },
    |_| {
        menu::group([
            menu::group([
                menu::item(with_icon(icon::badge_check(), "Account")).into(),
                menu::item(with_icon(icon::credit_card(), "Billing")).into(),
                menu::item(with_icon(icon::bell(), "Notifications")).into(),
            ]),
            menu::separator(),
            menu::item(with_icon(icon::log_out(), "Sign Out")).into(),
        ])
    },
)
.align(dropdown_menu::Align::End)
```
