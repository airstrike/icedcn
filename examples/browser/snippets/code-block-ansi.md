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
}
impl Demo {
    pub fn new() -> Self {
        Self {
            copied: BTreeMap::new(),
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
        let id = "ansi";
        {
            let (listing, plain) = highlight::ansi(dark);
            listing
                .title(row![icon("terminal"), code_block::title("Terminal")].spacing(6))
                .actions(self.copy("ansi", &plain))
                .into()
        }
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
        "terminal" => r#"<path d="m4 17 6-5-6-5m8 12h8"/>"#,
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

mod highlight {
    //! Parse the static samples once; the theme resolves their colors at display time.
    use super::*;

    use iced::highlighter::{Parser, Settings};
    use iced::widget::text::Wrapping;
    use iced::{Code, Fill, Font, Shrink};
    use icedxdy::theme::tweak::Token;
    use icedxdy::widget::code_block::{CodeBlock, LineStyle};
    use icedxdy::widget::{rich_text, span};
    use icedxdy::{Color, Slot};
    use std::collections::HashMap;
    use std::ops::Range;
    use std::sync::LazyLock;

    fn rgb(hex: u32) -> Token {
        Color::from(iced::Color::from_rgb8(
            (hex >> 16) as u8,
            (hex >> 8) as u8,
            hex as u8,
        ))
        .into()
    }

    pub fn ansi(dark: bool) -> (CodeBlock<'static, Message>, String) {
        let mut stripped = Vec::new();
        let lines = ANSI
            .lines()
            .map(|line| {
                let mut color = Token::CodeForeground;
                let mut spans = Vec::new();
                let mut plain = String::new();
                let mut pieces = line.split("\x1b[");
                let prefix = pieces.next().unwrap_or_default();
                plain.push_str(prefix);
                spans.push(span(prefix));
                for piece in pieces {
                    if let Some((codes, value)) = piece.split_once('m') {
                        for code in codes.split(';') {
                            color = match code {
                                "2" => Token::MutedForeground,
                                "31" => rgb(if dark { 0xf97583 } else { 0xb31d28 }),
                                "32" => rgb(if dark { 0x85e89d } else { 0x22863a }),
                                "0" | "22" | "39" => Token::CodeForeground,
                                _ => color,
                            };
                        }
                        plain.push_str(value);
                        spans.push(span(value).color(color));
                    }
                }
                stripped.push(plain);
                (
                    LineStyle::Default,
                    Slot::from(
                        rich_text(spans)
                            .size(14)
                            .line_height(22.75)
                            .font(Font::new("Geist Mono")),
                    ),
                )
            })
            .collect::<Vec<_>>();
        (
            CodeBlock::lines(lines)
                .wrap(true)
                .font(Font::new("Geist Mono")),
            stripped.join("\n"),
        )
    }
}

const ANSI: &str = "$ pnpm test\n\x1b[32m✓\x1b[39m src/format.test.ts \x1b[2m(4 tests)\x1b[22m\n\x1b[32m✓\x1b[39m src/parse.test.ts \x1b[2m(7 tests)\x1b[22m\n\x1b[31m✗\x1b[39m src/stream.test.ts \x1b[2m(3 tests | 1 failed)\x1b[22m\n\n\x1b[31mAssertionError: expected 'hello' to be 'hello world'\x1b[39m\n\nTests  \x1b[31m1 failed\x1b[39m | \x1b[32m13 passed\x1b[39m (14)";
```
