```rust
use iced::alignment::Horizontal;
use iced::widget::svg;
use iced::{Element, Fill, Font, Subscription, Task, Widget, clipboard, time};
use icedxdy::core::Border;
use icedxdy::widget::code_block::LineStyle;
use icedxdy::widget::{
    CodeBlock, bubble, button, code_block, column, container, markdown, message, row, tabs, text,
    tooltip, trace,
};
use icedxdy::{Size, Slot, Theme, radius, theme};
use std::collections::BTreeMap;
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub enum Message {
    Noop,
}

pub struct Demo {}
impl Demo {
    pub fn new() -> Self {
        Self {}
    }
    fn event(&self, event: Message) -> Message {
        event
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Noop => {}
        }

        Task::none()
    }
    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
    pub fn view(&self, theme: &Theme) -> Element<'_, Message, Theme> {
        container(self.content(theme.neutral.is_dark))
            .width(Fill)
            .max_width(448)
            .into()
    }
    fn content(&self, dark: bool) -> Slot<'_, Message> {
        let normal = |_: usize, _: &str| LineStyle::Default;
        let id = "highlighted-lines";
        plain_lines(HIGHLIGHTED_LINES, |i, _| {
            if [2, 3].contains(&i) {
                LineStyle::Highlighted
            } else {
                LineStyle::Default
            }
        })
        .start_line(1)
        .into()
    }
}

fn plain_lines(
    source: &'static str,
    style: impl Fn(usize, &str) -> LineStyle,
) -> CodeBlock<'static, Message> {
    CodeBlock::lines(source.split('\n').enumerate().map(|(i, line)| {
        (
            style(i, line),
            iced::widget::text(line)
                .selectable(true)
                .font(Font::new("Geist Mono"))
                .size(14)
                .line_height(22.75)
                .into(),
        )
    }))
    .font(Font::new("Geist Mono"))
}
const HIGHLIGHTED_LINES: &str = r##"export default {
  experimental: {
    reactCompiler: true,
    viewTransition: true,
  },
  images: {
    remotePatterns: [{ hostname: "avatars.githubusercontent.com" }],
  },
}"##;
```
