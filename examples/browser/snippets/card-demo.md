```rust
use iced::Alignment::{Center, End, Start};
use iced::widget::responsive;
use iced::{Element, Fill, Shrink, Widget};
use icedcn::widget::{button, column, container, field, input, row, text};
use icedcn::{Slot, card};

fn login_card<'a, M: Clone + 'a>(input_size: f32, forgot: M) -> card::Card<'a, M> {
    let password = responsive(move |area| {
        let wraps = area.width < 211.;
        let link = button(text::small("Forgot your password?").weight(icedcn::Weight::Normal))
            .style(button::Link)
            .anchor()
            .translate(false)
            .height(if wraps { 40 } else { 20 })
            .padding(0)
            .on_press(forgot.clone());
        let recovery = container(link)
            .width(Fill)
            .align_x(if wraps { Start } else { End });
        Element::from(
            row![text::label("Password"), recovery]
                .align_y(Center)
                .width(Fill),
        )
    })
    .height(Shrink)
    .boxed();

    card(
        column![
            card::header("Login to your account")
                .description("Enter your email below to login to your account")
                .action(button("Sign Up").style(button::Link)),
            card::content(
                column![
                    field(input("m@example.com").text_size(input_size).width(Fill))
                        .label(text::label("Email")),
                    field(input("").secure(true).text_size(input_size).width(Fill))
                        .label(Slot::from(password)),
                ]
                .spacing(24)
                .width(Fill)
            ),
            card::footer(
                column![
                    button("Login").width(Fill),
                    button("Login with Google")
                        .style(button::Outline)
                        .width(Fill),
                ]
                .spacing(8)
                .width(Fill)
            ),
        ]
        .width(Fill),
    )
    .width(Fill.max(384))
}
```

Pass the message for your password-recovery link. The preview uses 14px input text on desktop and 16px below 768px.
