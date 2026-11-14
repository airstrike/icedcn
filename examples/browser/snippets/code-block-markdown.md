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
    Replay(&'static str),
    Tick,
}

pub struct Demo {
    copied: BTreeMap<&'static str, Instant>,
    markdown_after: Vec<markdown::Item>,
    markdown_before: Vec<markdown::Item>,
    streaming: BTreeMap<&'static str, Instant>,
}
impl Demo {
    pub fn new() -> Self {
        Self {
            copied: BTreeMap::new(),
            markdown_after: markdown_parts(usize::MAX).1,
            markdown_before: markdown_parts(usize::MAX).0,
            streaming: BTreeMap::new(),
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
            Message::Replay(id) => {
                self.streaming.insert(id, Instant::now());
            }
            Message::Tick => self
                .streaming
                .retain(|id, at| markdown_bytes(*at) < total_length(id)),
        }
        let end = self
            .streaming
            .get("markdown")
            .map(|s| markdown_bytes(*s))
            .unwrap_or(usize::MAX);
        (self.markdown_before, self.markdown_after) = markdown_parts(end);
        Task::none()
    }
    pub fn subscription(&self) -> Subscription<Message> {
        if self.streaming.is_empty() {
            Subscription::none()
        } else {
            time::every(Duration::from_millis(30)).map(|_| Message::Tick)
        }
    }
    pub fn view(&self, theme: &Theme) -> Element<'_, Message, Theme> {
        container(self.content(theme.neutral.is_dark))
            .width(Fill)
            .max_width(448)
            .into()
    }
    fn content(&self, dark: bool) -> Slot<'_, Message> {
        let normal = |_: usize, _: &str| LineStyle::Default;
        let id = "markdown";
        self.markdown(dark)
    }
    fn markdown(&self, dark: bool) -> Slot<'_, Message> {
        let end = self
            .streaming
            .get("markdown")
            .map(|s| markdown_bytes(*s))
            .unwrap_or(usize::MAX)
            .min(MARKDOWN.len());
        let body = &MARKDOWN[..end];
        let (_, fenced) = body.split_once("```tsx\n").unwrap_or((body, ""));
        let noop = self.event(Message::Tick);
        let settings = prose_settings(dark);
        let mut reply = column![
            markdown::view(&self.markdown_before, settings)
                .map(move |_| noop.clone())
                .boxed()
        ]
        .spacing(12);
        if !fenced.is_empty() {
            let (code, _) = fenced.split_once("```").unwrap_or((fenced, ""));
            let complete = fenced.contains("```");
            let copied = self.copied.contains_key("markdown");
            reply = reply.push(
                listing(
                    dark,
                    STREAMING_0,
                    code.trim_end_matches('\n').len(),
                    false,
                    |_, _| LineStyle::Default,
                )
                .title(code_block::title("tsx"))
                .actions(tooltip(
                    button(if copied { icon("check") } else { icon("copy") })
                        .square()
                        .size(Size::Xs)
                        .style(button::Ghost)
                        .disabled(!complete)
                        .on_press(self.event(Message::Copy("markdown", code.to_owned()))),
                    if copied { "Copied" } else { "Copy" },
                ))
                .into(),
            );
        }
        let noop = self.event(Message::Tick);
        reply = reply.push(
            markdown::view(&self.markdown_after, settings)
                .map(move |_| noop.clone())
                .boxed()
                .into(),
        );
        let mut items = column![
            message(
                bubble("How do I debounce a search input in React?").style(bubble::Variant::Muted)
            )
            .align(Horizontal::Right),
            message(reply),
        ]
        .spacing(16);
        if !self.streaming.contains_key("markdown") {
            items = items.push(
                button("Replay")
                    .size(Size::Sm)
                    .style(button::Outline)
                    .on_press(self.event(Message::Replay("markdown")))
                    .into(),
            );
        }
        container(items)
            .padding([24, 0])
            .min_height(448)
            .width(Fill)
            .into()
    }
}

fn prose_settings(dark: bool) -> markdown::Settings {
    let theme = if dark { Theme::dark() } else { Theme::light() };
    markdown::Settings::with_text_size(14, markdown::Style::from(theme.iced_theme()))
}
fn total_length(id: &str) -> usize {
    if id == "markdown" { MARKDOWN.len() } else { 0 }
}
fn markdown_parts(end: usize) -> (Vec<markdown::Item>, Vec<markdown::Item>) {
    let body = &MARKDOWN[..end.min(MARKDOWN.len())];
    let (before, fenced) = body.split_once("```tsx\n").unwrap_or((body, ""));
    let after = fenced
        .split_once("```")
        .map(|(_, after)| after)
        .unwrap_or("");
    (
        markdown::parse(before).collect(),
        markdown::parse(after).collect(),
    )
}
fn markdown_bytes(start: Instant) -> usize {
    markdown_prefix(start.elapsed().as_millis() as usize / 30)
}
fn markdown_prefix(deltas: usize) -> usize {
    use std::sync::LazyLock;
    static ENDS: LazyLock<Vec<usize>> = LazyLock::new(|| {
        let mut ends = Vec::new();
        let mut whitespace = false;
        for (index, ch) in MARKDOWN.char_indices() {
            if whitespace && !ch.is_whitespace() {
                ends.push(index);
            }
            whitespace = ch.is_whitespace();
        }
        ends.push(MARKDOWN.len());
        ends
    });
    if deltas == 0 {
        0
    } else {
        ENDS.get(deltas - 1).copied().unwrap_or(MARKDOWN.len())
    }
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
        let sources: Vec<(&str, &str)> = vec![(STREAMING_0, "ts")];
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
            let line = if false {
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

const MARKDOWN: &str = "Wrap the value in a `useDebounce` hook. It returns the latest value once it stops changing for `delay` milliseconds.\n\n```tsx\nexport function useDebounce<T>(value: T, delay = 300) {\n  const [debounced, setDebounced] = React.useState(value)\n\n  React.useEffect(() => {\n    const timeout = setTimeout(() => setDebounced(value), delay)\n    return () => clearTimeout(timeout)\n  }, [value, delay])\n\n  return debounced\n}\n```\n\nPass the debounced value to your fetch instead of the raw input.";

const STREAMING_0: &str = r##"export function useDebounce<T>(value: T, delay = 300) {
  const [debounced, setDebounced] = React.useState(value)

  React.useEffect(() => {
    const timeout = setTimeout(() => setDebounced(value), delay)
    return () => clearTimeout(timeout)
  }, [value, delay])

  return debounced
}"##;
```
