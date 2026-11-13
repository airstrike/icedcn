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
#[derive(Clone)]
enum Event {
    Spacing(Vec<String>),
    Forgot,
}
struct Spacing {
    page: usize,
}
impl<'a> Component<'a, Message, Theme> for Spacing {
    type State = Option<u16>;
    type Event = Event;
    fn update(&self, state: &mut Self::State, event: Event, _: &iced::Renderer) -> Option<Message> {
        match event {
            Event::Spacing(value) => {
                if let Some(n) = value.first().and_then(|v| v.parse::<u16>().ok()) {
                    *state = Some(n);
                }
                None
            }
            Event::Forgot => Some(Message::Page(
                self.page,
                page::Message::LinkClicked("#".into()),
            )),
        }
    }
    fn view(&self, state: &Self::State) -> impl Widget<Event, Theme> + 'a {
        let spacing = state.unwrap_or(16);
        Element::from(
            column![
                toggle_group(|| [16, 20, 24, 32]
                    .map(|n| toggle_group::item(n.to_string(), text::small(format!("{n}px")))))
                .style(toggle_group::Outline)
                .size(Size::Sm)
                .value([spacing.to_string()])
                .on_value_change(Event::Spacing),
                login_card(f32::from(spacing), Event::Forgot),
            ]
            .spacing(16)
            .align_x(Center)
            .width(Fill),
        )
    }
}
```
