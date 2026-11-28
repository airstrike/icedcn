```rust
use crate::icon;
use iced::Fill;
use iced::alignment::Horizontal::Right;
use iced::alignment::Vertical::Bottom;
use icedcn::Size;
use icedcn::View;
use icedcn::theme::tweak::{Fg, Token};
use icedcn::widget::bubble::Destructive;
use icedcn::widget::{bubble, button, column, container, popover};

fn example<'a>() -> View<'a> {
    frame(
        column![
            bubble("Run the build script.").align(Right),
            bubble("Failed to run the command.")
                .style(Destructive)
                .reactions(
                    popover(
                        |p| p.trigger(
                            button(icon::info().size(16).line_height(1.0))
                                .style(
                                    button::Ghost
                                        + Fg(if p.is_open() {
                                            Token::Destructive
                                        } else {
                                            Token::Foreground
                                        })
                                )
                                .size(Size::Xs)
                                .square()
                        ),
                        |_| popover::header(
                            "Command failed with exit code 1",
                            "Could not open Cargo.lock: No such file or directory"
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

fn frame<'a>(body: impl Into<View<'a>>) -> View<'a> {
    container(body)
        .padding([48, 0])
        .width(Fill)
        .max_width(384)
        .into()
}
```
