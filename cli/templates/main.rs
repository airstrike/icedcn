mod theme;

use iced::theme::Mode;
use iced::{Element, Subscription, Task, system};
use icedcn::Theme;
use icedcn::widget::{button, center, column, text};

struct Counter {
    value: u32,
    theme: Theme,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    SystemMode(Mode),
}

impl Counter {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                value: 0,
                theme: theme::create(Mode::None),
            },
            system::theme().map(Message::SystemMode),
        )
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value = self.value.saturating_add(1),
            Message::SystemMode(mode) => self.theme = theme::create(mode),
        }
    }

    fn view(&self) -> Element<'_, Message, Theme> {
        center(
            column![
                text::h1(self.value.to_string()),
                button("Add one").on_press(Message::Increment),
            ]
            .spacing(16)
            .align_x(iced::Center),
        )
        .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn subscription(&self) -> Subscription<Message> {
        system::theme_changes().map(Message::SystemMode)
    }
}

fn main() -> iced::Result {
    iced::application(Counter::new, Counter::update, Counter::view)
        .title("__NAME__")
        .theme(Counter::theme)
        .subscription(Counter::subscription)
        .window_size([640.0, 480.0])
        .run()
}
