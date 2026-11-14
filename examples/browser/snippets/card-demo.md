```rust
use iced::Alignment::Center;
use iced::{Element, Fill, Widget};
use icedcn::{card, Size, Slot, Theme};
use icedcn::widget::{button, column, field, input, row, space, text};

fn login_card<'a, M: Clone + 'a>(spacing: f32, forgot: M) -> Slot<'a, M> {
    let password_label = row![
        text::label("Password"),
        space::horizontal(),
        button(
            text::small("Forgot your password?")
                .weight(icedcn::Weight::Normal)
                .line_height(iced::Pixels(20.0))
        )
        .style(button::Link)
        .anchor()
        .translate(false)
        .height(20)
        .padding(0)
        .on_press(forgot)
    ]
    .align_y(Center)
    .width(Fill);
    card(
        column![
            card::header("Login to your account")
                .description("Enter your email below to login to your account")
                .action(button("Sign Up").style(button::Link))
                .inset(spacing),
            card::content(
                column![
                    field(input("m@example.com").width(Fill)).label("Email"),
                    field(input("").secure(true).width(Fill)).label(Slot::from(password_label))
                ]
                .spacing(24)
                .width(Fill)
            )
            .padding([0.0, spacing]),
            card::footer_with_spacing(
                column![
                    button("Login").width(Fill),
                    button("Login with Google")
                        .style(button::Outline)
                        .width(Fill)
                ]
                .spacing(8)
                .width(Fill),
                spacing
            )
        ]
        .width(Fill)
        .spacing(spacing)
        .padding(iced::Padding::ZERO.top(spacing)),
    )
    .width(Fill.max(384))
    .boxed()
    .into()
}

fn preview() -> Element<'static, Message, Theme> {
    login_card(16.0, Message::Forgot).into()
}
```
