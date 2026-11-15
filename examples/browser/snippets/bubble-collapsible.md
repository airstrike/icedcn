```rust
use crate::icon;
use iced::Fill;
use iced::alignment::Horizontal::Right;
use icedcn::Slot;
use icedcn::theme::text::muted;
use icedcn::theme::tweak::{Fg, Token};
use icedcn::widget::bubble::Muted;
use icedcn::widget::{bubble, button, collapsible, column, container, row, text};

fn example<'a>() -> Slot<'a, ()> {
    frame(
        column![
            bubble("How can I help you today?").style(Muted),
            bubble(review()).style(Muted).align(Right)
        ]
        .spacing(32)
        .width(Fill),
    )
}

fn frame(body: impl Into<Slot<'static, ()>>) -> Slot<'static, ()> {
    container(body)
        .padding([48, 0])
        .width(Fill)
        .max_width(384)
        .into()
}

fn review() -> Slot<'static, ()> {
    const REVIEW: &str = "The accessibility review found two focus states that were visually too subtle in dark mode.\n\nI checked the dialog, menu, and drawer paths because each one renders focusable controls inside a layered surface.\n\nThe dialog and drawer are fine. The menu needs the hover and focus tokens split so keyboard focus stays visible when the pointer is not involved.\n\nI also recommend keeping the change in the style file instead of the primitive so the other themes can choose their own focus treatment later.";
    collapsible(|c| {
        let description = if c.is_open() {
            REVIEW.into()
        } else {
            format!("{}...", &REVIEW[..180])
        };
        let open = c.is_open();
        let chevron = if open {
            icon::chevron_up()
        } else {
            icon::chevron_down()
        };
        column![
            text(description),
            c.trigger(
                button(
                    row![
                        text(if open { "Show less" } else { "Show more" }),
                        chevron
                            .size(16)
                            .line_height(1.0)
                            .style(|theme| muted(theme).into())
                    ]
                    .spacing(4)
                )
                .style(button::Link + Fg(Token::MutedForeground))
                .padding(0)
                .inset(0)
            )
        ]
        .into()
    })
    .into()
}
```
