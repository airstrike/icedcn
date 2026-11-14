```rust
use iced::Fill;
use iced::Widget;
use iced::alignment::Horizontal::Right;
use iced::widget::svg;
use icedcn::Slot;
use icedcn::Theme;
use icedcn::theme::tweak::{Fg, Token};
use icedcn::widget::bubble::Muted;
use icedcn::widget::{bubble, button, collapsible, column, container, row, text};

fn example() -> Slot<'static, ()> {
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

fn glyph(path: &'static str, muted: bool, destructive: bool) -> Slot<'static, ()> {
    let source = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">{path}</svg>"#
    );
    svg(svg::Handle::from_memory(source.into_bytes()))
        .width(16)
        .height(16)
        .style(move |theme: &Theme, _| svg::Style {
            color: Some(
                if destructive {
                    theme.destructive.base
                } else if muted {
                    theme.muted.foreground
                } else {
                    theme.neutral.foreground
                }
                .into(),
            ),
        })
        .boxed()
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
            r#"<path d="m18 15-6-6-6 6"/>"#
        } else {
            r#"<path d="m6 9 6 6 6-6"/>"#
        };
        column![
            text(description),
            c.trigger(
                button(
                    row![
                        text(if open { "Show less" } else { "Show more" }),
                        glyph(chevron, true, false)
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
