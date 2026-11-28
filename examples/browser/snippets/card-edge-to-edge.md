```rust
use iced::Fill;
use icedcn::theme::container::Transparent;
use icedcn::theme::tweak::{Alpha, Bg, Token};
use icedcn::widget::{button, column, container, row, scroll_area, separator, space, text};
use icedcn::{Slot, card};

fn terms<'a, Message: Clone + 'a>() -> Slot<'a, Message> {
    const TERMS: [&str; 4] = [
        "These terms govern your use of the workspace, including access to shared documents, project files, and collaboration tools.",
        "You are responsible for the content you upload and for ensuring that your team has the appropriate permissions to view or edit it.",
        "We may update features or limits as the service evolves. When those changes materially affect your workflow, we will notify your workspace administrators.",
        "By continuing, you agree to keep your account credentials secure and to follow your organization's acceptable use policies.",
    ];
    let body = column(TERMS.map(|p| text::small(p).line_height(1.625).into()))
        .spacing(16)
        .padding(16)
        .width(Fill);
    let terms = container(scroll_area(body).height(192))
        .width(Fill)
        .style(Transparent + Bg(Token::Muted) + Alpha(0.5));
    card(
        column![
            container(
                card::header("Terms of Service")
                    .description("Review the terms before accepting the agreement.")
            )
            .padding(iced::Padding::ZERO.bottom(16)),
            separator(),
            terms,
            card::footer(
                row![
                    space::horizontal(),
                    button("Decline").style(button::Outline),
                    button("Accept")
                ]
                .spacing(8)
                .width(Fill)
            )
        ]
        .width(Fill)
        .spacing(0),
    )
    .width(Fill.max(384))
    .into()
}
```
