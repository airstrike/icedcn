```rust
fn login_card<'a, M: Clone + 'a>(spacing: f32, forgot: M) -> Slot<'a, M> {
    let password_label = row![
        text::label("Password"),
        space::horizontal(),
        button("Forgot your password?")
            .style(button::Link)
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
    .width(384)
    .boxed()
    .into()
}
```
