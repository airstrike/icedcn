```rust
use highlight::listing;
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
        let id = "diff";
        listing(dark, DIFF, usize::MAX, false, |_, line| diff_style(line))
            .title(code_block::title("components/greeting.tsx"))
            .actions(self.copy("diff", DIFF))
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

fn diff_style(line: &str) -> LineStyle {
    if line.starts_with('+') {
        LineStyle::Added
    } else if line.starts_with('-') {
        LineStyle::Removed
    } else {
        LineStyle::Default
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

    type Runs = Vec<Vec<(Range<usize>, Code)>>;
    static TOKENS: LazyLock<HashMap<&'static str, Runs>> = LazyLock::new(|| {
        let sources: Vec<(&str, &str)> = vec![];
        sources
            .into_iter()
            .map(|(source, lang)| {
                let mut parser = Parser::new(&Settings {
                    token: lang.to_string(),
                });
                let lines = source
                    .split('\n')
                    .map(|line| {
                        parser
                            .parse_line(if false {
                                line.get(1..).unwrap_or("")
                            } else {
                                line
                            })
                            .collect()
                    })
                    .collect();
                (source, lines)
            })
            .collect()
    });

    // GitHub Light/Dark, matching the pinned Shiki theme choices. The existing
    // parser still owns grammar classification; no shared syntax-engine changes.
    fn color(class: Code, dark: bool) -> Token {
        let (light, night) = match class {
            Code::Comment => (0x6a737d, 0x6a737d),
            Code::String => (0x032f62, 0x9ecbff),
            Code::Keyword | Code::Invalid => (0xd73a49, 0xf97583),
            Code::Function => (0x6f42c1, 0xb392f0),
            Code::Constant | Code::Support => (0x005cc5, 0x79b8ff),
            Code::Type => (0x22863a, 0x85e89d),
            _ => (0x24292e, 0xe1e4e8),
        };
        rgb(if dark { night } else { light })
    }
    fn rgb(hex: u32) -> Token {
        Color::from(iced::Color::from_rgb8(
            (hex >> 16) as u8,
            (hex >> 8) as u8,
            hex as u8,
        ))
        .into()
    }

    /// Render a prefix so one cached parse can also serve a streamed listing.
    pub fn listing(
        dark: bool,
        source: &'static str,
        visible: usize,
        wrap: bool,
        styles: impl Fn(usize, &str) -> LineStyle,
    ) -> CodeBlock<'static, Message> {
        let wrapping = if wrap {
            Wrapping::WordOrGlyph
        } else {
            Wrapping::None
        };
        let tokens = TOKENS.get(source);
        let mut end = visible.min(source.len());
        while !source.is_char_boundary(end) {
            end -= 1;
        }
        let lines = source[..end].split('\n').enumerate().map(|(i, line)| {
            let original = line;
            let line = if source == DIFF {
                line.get(1..).unwrap_or("")
            } else {
                line
            };
            let mut spans = Vec::new();
            if let Some(runs) = tokens.and_then(|all| all.get(i)) {
                let mut pos = 0;
                for (range, class) in runs {
                    let start = range.start.min(line.len());
                    let end = range.end.min(line.len());
                    if start > pos {
                        spans.push(span(&line[pos..start]));
                    }
                    if end > start {
                        spans.push(span(&line[start..end]).color(color(*class, dark)));
                    }
                    pos = end;
                }
                if pos < line.len() {
                    spans.push(span(&line[pos..]));
                }
            } else {
                spans.push(span(line));
            }
            if line.is_empty() {
                spans = vec![span("")];
            }
            let text = rich_text(spans)
                .font(Font::new("Geist Mono"))
                .size(14)
                .line_height(22.75)
                .wrapping(wrapping)
                .width(if wrap { Fill } else { Shrink });
            (styles(i, original), Slot::from(text))
        });
        CodeBlock::lines(lines)
            .wrap(wrap)
            .font(Font::new("Geist Mono"))
    }
}

const DIFF: &str = r##" export function Greeting({ name }: { name: string }) {
-  return <p>Hello, {name}</p>
+  return <p className="text-sm text-muted-foreground">
+    Hello, {name}!
+  </p>
 }"##;
```
