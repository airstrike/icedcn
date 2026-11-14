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
        let id = "wrap";
        column![terminal(WRAP), terminal(WRAP).wrap(true)]
            .spacing(16)
            .into()
    }
}

fn terminal(source: &str) -> CodeBlock<'_, Message> {
    code_block(source).font(Font::new("Geist Mono"))
}
const WRAP: &str = r##"Error: The model "anthropic/claude-sonnet-4.5" returned an error: Request too large. The prompt is 212,431 tokens and the context window is 200,000 tokens. Remove some messages or summarize the conversation and try again."##;
```
