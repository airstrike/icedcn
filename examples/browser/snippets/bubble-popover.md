```rust
use iced::Fill;
use iced::Widget;
use iced::alignment::Horizontal::Right;
use iced::alignment::Vertical::Bottom;
use iced::widget::svg;
use icedxdy::Size;
use icedxdy::Slot;
use icedxdy::Theme;
use icedxdy::widget::bubble::Destructive;
use icedxdy::widget::{bubble, button, column, container, popover};

fn example() -> Slot<'static, ()> {
    frame(
        column![
            bubble("Run the build script.").align(Right),
            bubble("Failed to run the command.")
                .style(Destructive)
                .reactions(
                    popover(
                        |p| p.trigger(
                            button(glyph(
                                r#"<circle cx="12" cy="12" r="10"/><path d="M12 16v-4M12 8h.01"/>"#,
                                false,
                                p.is_open()
                            ))
                            .size(Size::Xs)
                            .square()
                            .style(button::Ghost)
                        ),
                        |_| popover::header(
                            "Command failed with exit code 1",
                            "ENOENT: no such file or directory, open pnpm-lock.yaml"
                        )
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
