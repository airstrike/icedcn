```rust
use iced::Fill;
use iced::Widget;
use iced::alignment::Horizontal::Right;
use iced::alignment::Vertical::Bottom;
use iced::widget::svg;
use icedcn::Size;
use icedcn::Slot;
use icedcn::Theme;
use icedcn::widget::bubble::Secondary;
use icedcn::widget::{bubble, button, column, container, tooltip};

fn example() -> Slot<'static, ()> {
    frame(
        column![
            bubble("Did you remove the stale route?").style(Secondary),
            bubble("Yes, removed it from the registry.")
                .align(Right)
                .reactions(
                    tooltip(
                        button(glyph(r#"<path d="m20 6-11 11-5-5"/>"#, false, false))
                            .size(Size::Xs)
                            .square()
                            .style(button::Ghost),
                        "Read on Jan 5, 2026 at 4:32 PM"
                    ),
                    Right,
                    Bottom
                )
                .reaction_padding(0)
        ]
        .spacing(16)
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
```
