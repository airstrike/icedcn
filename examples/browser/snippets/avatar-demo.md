```rust
use iced::widget::image;
use iced::{Subscription, Task, window};
use icedxdy::Theme;
use icedxdy::widget::avatar;
use icedxdy::widget::row;

use icedxdy::theme::avatar::badge_style;
use icedxdy::theme::container;

fn presence(theme: &Theme) -> container::Style {
    let mut style = badge_style(theme);
    let color = if theme.neutral.base.l < 0.5 {
        iced::Color::from_rgb8(1, 102, 48)
    } else {
        iced::Color::from_rgb8(0, 166, 62)
    };
    style.background = Some(icedxdy::Color::from_srgb(color).into());
    style.border.width = 0.0;
    style
}

struct Demo {
    window_width: f32,
    portrait: image::Handle,
    maxleiter: image::Handle,
    evilrabbit: image::Handle,
}

#[derive(Clone, Debug)]
enum Message {
    Width(f32),
}

impl Demo {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                window_width: 1024.0,
                portrait: image::Handle::from_path("assets/avatars/02.png"),
                maxleiter: image::Handle::from_path("assets/avatars/maxleiter.png"),
                evilrabbit: image::Handle::from_path("assets/avatars/evilrabbit.png"),
            },
            window::oldest()
                .and_then(window::size)
                .map(|size| Message::Width(size.width)),
        )
    }

    fn update(&mut self, Message::Width(width): Message) {
        self.window_width = width;
    }

    fn subscription(&self) -> Subscription<Message> {
        window::resize_events().map(|(_, size)| Message::Width(size.width))
    }

    fn view(&self) -> iced::Element<'_, Message, Theme> {
        row![
            avatar(Some(self.portrait.clone())).fallback("AC"),
            avatar::badge(
                avatar(Some(self.evilrabbit.clone())).fallback("ER"),
                avatar::dot()
            )
            .badge_style(presence),
            avatar::group([
                avatar(Some(self.portrait.clone())).fallback("AC"),
                avatar(Some(self.maxleiter.clone())).fallback("LR"),
                avatar(Some(self.evilrabbit.clone())).fallback("ER"),
            ])
            .count("+3"),
        ]
        .spacing(if self.window_width >= 768.0 { 48 } else { 24 })
        .wrap()
        .align_y(iced::Center)
        .into()
    }
}

fn main() -> iced::Result {
    iced::application(Demo::new, Demo::update, Demo::view)
        .subscription(Demo::subscription)
        .theme(|_: &Demo| Theme::light())
        .run()
}
```
