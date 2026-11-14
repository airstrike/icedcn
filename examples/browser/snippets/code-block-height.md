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
    Copy(&'static str, String),
    Copied(&'static str, Instant),
    ResetCopy(&'static str, Instant),
}

pub struct Demo {
    copied: BTreeMap<&'static str, Instant>,
    logs: String,
}
impl Demo {
    pub fn new() -> Self {
        Self {
            copied: BTreeMap::new(),
            logs: (0..40)
                .map(|i| {
                    format!(
                        "[{:02}] GET /api/chat 200 in {}ms",
                        i + 1,
                        12 + (i * 7) % 40
                    )
                })
                .collect::<Vec<_>>()
                .join("\n"),
        }
    }
    fn event(&self, event: Message) -> Message {
        event
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Noop => {}
            Message::Copy(id, value) => {
                return clipboard::write(value).then(move |result| {
                    if result.is_ok() {
                        Task::done(Message::Copied(id, Instant::now()))
                    } else {
                        Task::none()
                    }
                });
            }
            Message::Copied(id, at) => {
                self.copied.insert(id, at);
                return Task::perform(tokio::time::sleep(Duration::from_secs(2)), move |_| {
                    Message::ResetCopy(id, at)
                });
            }
            Message::ResetCopy(id, at) => {
                if self.copied.get(id) == Some(&at) {
                    self.copied.remove(id);
                }
            }
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
        let id = "height";
        terminal(&self.logs)
            .max_height(192.)
            .title("Logs")
            .actions(self.copy("height", &self.logs))
            .into()
    }
    fn copy(&self, id: &'static str, source: &str) -> Slot<'static, Message> {
        let copied = self.copied.contains_key(id);
        let glyph = if copied { icon("check") } else { icon("copy") };
        action(
            glyph,
            if copied { "Copied" } else { "Copy" },
            self.event(Message::Copy(id, source.to_owned())),
        )
    }
}

fn terminal(source: &str) -> CodeBlock<'_, Message> {
    code_block(source).font(Font::new("Geist Mono"))
}
fn action(
    glyph: impl Into<Slot<'static, Message>>,
    label: &'static str,
    event: Message,
) -> Slot<'static, Message> {
    tooltip(
        button(glyph)
            .square()
            .style(button::Ghost)
            .size(Size::Xs)
            .on_press(event),
        label,
    )
    .into()
}
fn icon(name: &str) -> Slot<'static, Message> {
    let path = match name {
        "check" => r#"<path d="m20 6-11 11-5-5"/>"#,
        "copy" => {
            r#"<rect width="14" height="14" x="8" y="8" rx="2"/><path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/>"#
        }
        _ => "",
    };
    let source = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">{path}</svg>"#
    );
    svg(svg::Handle::from_memory(source.into_bytes()))
        .width(14)
        .height(14)
        .style(|theme: &Theme, _| svg::Style {
            color: Some(theme.neutral.foreground.into()),
        })
        .boxed()
        .into()
}
```
