```rust
use iced::Fill;
use iced::alignment::Horizontal::Right;
use iced::alignment::Vertical::Bottom;
use iced::font::Weight;
use iced::widget::text::LineHeight;
use iced::widget::{rich_text, text::Span};
use iced::{Font, Widget, border};
use icedcn::Slot;
use icedcn::Theme;
use icedcn::widget::bubble::{Destructive, Ghost, Muted, Outline, Secondary, Tinted};
use icedcn::widget::{bubble, column, container, text};

fn example<'a>(theme: &Theme) -> Slot<'a, ()> {
    frame(
        column![
            bubble("This is the default primary bubble."),
            bubble("This is the secondary variant.")
                .style(Secondary)
                .align(Right),
            bubble("This one is muted. It uses a lower emphasis color for the chat bubble.")
                .style(Muted)
                .reactions("👍", Right, Bottom),
            bubble(
                "This one is tinted. The tint is a softer color derived from the primary color."
            )
            .style(Tinted)
            .align(Right),
            bubble("We can also use an outlined variant.").style(Outline),
            bubble("Or a destructive variant with a reaction.")
                .style(Destructive)
                .align(Right)
                .reactions("🔥", Right, Bottom),
            bubble(ghost_text(theme)).style(Ghost)
        ]
        .spacing(48)
        .width(Fill),
    )
}

fn frame<'a>(body: impl Into<Slot<'a, ()>>) -> Slot<'a, ()> {
    container(body)
        .padding([48, 0])
        .width(Fill)
        .max_width(384)
        .into()
}

fn ghost_text<'a>(theme: &Theme) -> Slot<'a, ()> {
    let bold = Font::DEFAULT.weight(Weight::Bold);
    let code = Span::<()>::new("code")
        .font("Geist Mono")
        .size(12.25)
        .color(theme.neutral.foreground)
        .background(iced::Background::Color(theme.muted.base.into()))
        .border(border::rounded(4))
        .padding([1, 4]);
    column![
        rich_text([Span::<()>::new("Ghost bubbles work for assistant text, "), Span::<()>::new("markdown").font(bold), Span::<()>::new(", and other content that should not be framed.")])
            .size(14).line_height(LineHeight::Absolute(22.75.into())).boxed(),
        rich_text([Span::<()>::new("This is perfect for assistant messages that should not have a frame and can take the full width of the container. You can also render "), code, Span::<()>::new(" in it.")])
            .size(14).line_height(LineHeight::Absolute(22.75.into())).boxed(),
        text("Ghost bubbles are full width and can take the full width of the container.")
    ].spacing(16).into()
}
```
