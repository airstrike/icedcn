```rust
use iced::Fill;
use iced::alignment::Horizontal::{Left, Right};
use iced::alignment::Vertical::{Bottom, Top};
use icedxdy::Size;
use icedxdy::Slot;
use icedxdy::widget::bubble::{Destructive, Muted};
use icedxdy::widget::{bubble, button, column, container, row, toast, toaster};

fn example() -> Slot<'static, ()> {
    toaster(frame(
        column![
            bubble("I don't need tests, I know my code works.")
                .style(Muted)
                .align(Right)
                .reactions(row!["👍", "😮"].spacing(4), Left, Bottom),
            bubble("Bold. Fine I'll add some tests. I'll let you know when they're done.")
                .style(Muted)
                .reactions(row!["👀", "🚀", "+2"].spacing(4), Right, Bottom),
            bubble("Tests passed on the first try. All 142 of them. Looking good!")
                .align(Right)
                .reactions(row!["🎉", "👏"].spacing(4), Left, Top),
            bubble("Are you sure I can run this command?")
                .style(Destructive)
                .reactions(
                    toast::trigger(
                        button("Yes, run it").size(Size::Xs).style(button::Ghost),
                        toast::toast("You clicked yes, running command...")
                            .kind(toast::Kind::Success)
                    ),
                    Right,
                    Bottom
                )
                .reaction_padding(0)
        ]
        .spacing(48)
        .width(Fill),
    ))
    .into()
}

fn frame(body: impl Into<Slot<'static, ()>>) -> Slot<'static, ()> {
    container(body)
        .padding([48, 0])
        .width(Fill)
        .max_width(384)
        .into()
}
```
