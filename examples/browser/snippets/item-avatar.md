```rust
use crate::icon;
use iced::Fill;
use icedcn::Size;
use icedcn::widget::{avatar, button, column, item};

column([
    item::item([
        item::media(
            avatar(Some(evilrabbit_handle()))
                .fallback("ER")
                .size(avatar::Lg),
        )
        .into(),
        item::content([
            item::title("Evil Rabbit").into(),
            item::description("Last seen 5 months ago").into(),
        ])
        .into(),
        item::actions([button(icon::plus())
            .rounded()
            .square()
            .size(Size::Sm)
            .style(button::Outline)
            .into()]),
    ])
    .style(item::Outline)
    .into(),
    item::item([
        item::media(avatar::group([
            avatar(Some(alex_handle())).fallback("AC"),
            avatar(Some(maxleiter_handle())).fallback("LR"),
            avatar(Some(evilrabbit_handle())).fallback("ER"),
        ]))
        .into(),
        item::content([
            item::title("No Team Members").into(),
            item::description("Invite your team to collaborate on this project.").into(),
        ])
        .into(),
        item::actions([button("Invite")
            .style(button::Outline)
            .size(Size::Sm)
            .into()]),
    ])
    .style(item::Outline)
    .into(),
])
.spacing(24)
.width(Fill)
.max_width(448.0)
```
