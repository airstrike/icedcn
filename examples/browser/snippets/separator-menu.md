```rust
use iced::widget::responsive;
use iced::{Center, Element, Fill, Length, Pixels, Shrink, Subscription, Task, Widget, window};
use icedxdy::{Theme, Vertical, theme};
use icedxdy::widget::{Column, column, container, row, separator, text};

struct Menu {
    window_width: f32,
}

#[derive(Debug, Clone)]
enum Message {
    Width(f32),
}

impl Menu {
    fn new() -> (Self, Task<Message>) {
        (
            Self { window_width: 1024.0 },
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

    fn view(&self) -> Element<'static, Message, Theme> {
        let wide = self.window_width >= 768.0;
        responsive(move |bounds| {
            let cramped = wide && bounds.width < 355.0;
            let width = |portion| {
                if cramped { Length::FillPortion(portion) } else { Shrink }
            };
            let mut items = row![
                menu_item("Settings", "Manage preferences").width(width(6)),
                separator().orientation(Vertical),
                menu_item("Account", "Profile & security").width(width(5)),
            ]
            .spacing(if wide { 16 } else { 8 })
            .width(if cramped { Fill } else { Shrink })
            .align_y(Center);
            if wide {
                items = items
                    .push(separator().orientation(Vertical).into())
                    .push(menu_item("Help", "Support & docs").width(width(4)).into());
            }
            Element::from(container(items).center_x(Fill))
        })
        .height(Shrink)
        .boxed()
    }
}

fn menu_item<'a>(title: &'a str, description: &'a str) -> Column<'a, Message> {
    column![
        text::small(title).medium().line_height(Pixels(20.0)),
        text::xs(description)
            .line_height(Pixels(16.0))
            .wrapping(text::Wrapping::Word)
            .style(theme::text::Muted),
    ]
    .spacing(4)
}

fn main() -> iced::Result {
    iced::application(Menu::new, Menu::update, Menu::view)
        .subscription(Menu::subscription)
        .theme(|_: &Menu| Theme::light())
        .run()
}
```
