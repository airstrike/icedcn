```rust
struct Verification {
    form: bool,
}

#[derive(Default)]
struct VerificationState {
    code: String,
    attempted: bool,
}

#[derive(Debug, Clone)]
enum Edit {
    Code(String),
    Resend,
    Verify,
}

impl<'a> Component<'a, Message, Theme> for Verification {
    type State = VerificationState;
    type Event = Edit;

    fn update(
        &self,
        state: &mut VerificationState,
        edit: Edit,
        _: &iced::Renderer,
    ) -> Option<Message> {
        match edit {
            Edit::Code(code) => {
                state.code = code;
                state.attempted = false;
            }
            Edit::Resend => *state = VerificationState::default(),
            Edit::Verify => state.attempted = true,
        }
        None
    }

    fn view(&self, state: &VerificationState) -> impl Widget<Edit, Theme> + 'a {
        let entry = input_otp(6)
            .value(state.code.clone())
            .on_value_change(Edit::Code);
        if !self.form {
            let caption = if state.code.is_empty() {
                "Enter your one-time password.".to_owned()
            } else {
                format!("You entered: {}", state.code)
            };
            return Element::from(
                column![entry, text::small(caption)]
                    .spacing(8)
                    .align_x(Horizontal::Center),
            );
        }
        let invalid = state.attempted && state.code.len() != 6;
        let mut code = field(entry.group_size(3).cell_size(44., 48.).invalid(invalid))
            .label(View::from(row![
                text::label("Verification code"),
                space::horizontal(),
                button(row![icon::refresh_ccw(), "Resend Code"].spacing(4))
                    .size(Size::Xs)
                    .style(button::Outline)
                    .on_press(Edit::Resend)
            ]))
            .description("I no longer have access to this email address.");
        if invalid {
            code = code.error("Enter the six-character code.");
        }
        let action = if state.attempted && !invalid {
            "Verified"
        } else {
            "Verify"
        };
        Element::from(
            container(card(column![
                card::header("Verify your login").description(
                    "Enter the verification code we sent to your email address: m@example.com."
                ),
                card::content(code),
                card::footer(
                    column![
                        button(action).on_press(Edit::Verify).width(Fill),
                        text::muted("Having trouble signing in? Contact support")
                    ]
                    .spacing(8)
                )
            ]))
            .width(360),
        )
    }
}
```
