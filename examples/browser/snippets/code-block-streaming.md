```rust
use highlight::{STREAMS, listing};
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
    streaming: BTreeMap<&'static str, Instant>,
}
impl Demo {
    pub fn new() -> Self {
        Self {
            copied: BTreeMap::new(),
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
                .retain(|id, at| visible_bytes(*at) < total_length(id)),
        }

        Task::none()
    }
    pub fn subscription(&self) -> Subscription<Message> {
        if self.streaming.is_empty() {
            Subscription::none()
        } else {
            time::every(Duration::from_millis(16)).map(|_| Message::Tick)
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
        let id = "streaming";
        self.streaming(dark)
    }
    fn replay(&self, id: &'static str) -> Slot<'static, Message> {
        button![replay_icon(), "Replay"]
            .style(button::Outline)
            .size(Size::Sm)
            .disabled(self.streaming.contains_key(id))
            .on_press(self.event(Message::Replay(id)))
            .into()
    }
    fn streaming(&self, dark: bool) -> Slot<'static, Message> {
        let mut remaining = self
            .streaming
            .get("streaming")
            .map(|s| visible_bytes(*s))
            .unwrap_or(usize::MAX);
        let mut output = column![].spacing(16);
        for &(title, _, source) in STREAMS {
            if remaining == 0 {
                break;
            }
            let count = remaining.min(source.len());
            let mut block = listing(dark, source, count, false, |index, line| {
                if source == STREAMING_2 {
                    diff_style(line)
                } else if source == STREAMING_3 && (5..=8).contains(&index) {
                    LineStyle::Highlighted
                } else {
                    LineStyle::Default
                }
            });
            if title != "Terminal" {
                let copied = self.copied.contains_key(title);
                block = block.title(code_block::title(title)).actions(tooltip(
                    button(if copied { icon("check") } else { icon("copy") })
                        .square()
                        .size(Size::Xs)
                        .style(button::Ghost)
                        .disabled(count < source.len())
                        .on_press(self.event(Message::Copy(
                            title,
                            if source == STREAMING_2 {
                                source
                                    .lines()
                                    .map(|s| s.get(1..).unwrap_or(""))
                                    .collect::<Vec<_>>()
                                    .join("\n")
                            } else {
                                source.to_owned()
                            },
                        ))),
                    if copied { "Copied" } else { "Copy" },
                ));
            }
            if !matches!(title, "Terminal" | ".env.local") {
                block = block.start_line(1);
            }
            output = output.push(block.into());
            remaining = remaining.saturating_sub(count);
        }
        output.push(self.replay("streaming")).into()
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
fn total_length(id: &str) -> usize {
    if id == "markdown" {
        0
    } else {
        STREAMS.iter().map(|(_, _, s)| s.len()).sum()
    }
}
fn visible_bytes(start: Instant) -> usize {
    (start.elapsed().as_millis() as usize / 16) * 3
}
fn replay_icon() -> Slot<'static, Message> {
    svg(svg::Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"/><path d="M3 3v5h5"/></svg>"#))
        .width(14).height(14).style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) }).boxed().into()
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

    pub const STREAMS: &[(&str, &str, &str)] = &[
        ("hooks/use-debounce.ts", "ts", STREAMING_0),
        ("Terminal", "sh", STREAMING_1),
        ("components/search.tsx", "tsx", STREAMING_2),
        ("app/api/chat/route.ts", "ts", STREAMING_3),
        (".env.local", "sh", STREAMING_4),
    ];
    type Runs = Vec<Vec<(Range<usize>, Code)>>;
    static TOKENS: LazyLock<HashMap<&'static str, Runs>> = LazyLock::new(|| {
        let sources: Vec<(&str, &str)> = vec![
            (STREAMING_0, "ts"),
            (STREAMING_1, "sh"),
            (STREAMING_2, "tsx"),
            (STREAMING_3, "ts"),
            (STREAMING_4, "sh"),
        ];
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
                            .parse_line(if source == STREAMING_2 {
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
            let line = if source == STREAMING_2 {
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

const STREAMING_0: &str = r##"export function useDebounce<T>(value: T, delay = 300) {
  const [debounced, setDebounced] = React.useState(value)

  React.useEffect(() => {
    const timeout = setTimeout(() => setDebounced(value), delay)
    return () => clearTimeout(timeout)
  }, [value, delay])

  return debounced
}"##;

const STREAMING_1: &str = r##"pnpm dlx shadcn@latest add code-block"##;

const STREAMING_2: &str = r##" export function Search() {
   const [query, setQuery] = React.useState("")
-  const results = useSearch(query)
+  const debounced = useDebounce(query, 300)
+  const results = useSearch(debounced)

   return <SearchResults results={results} />
 }"##;

const STREAMING_3: &str = r##"import { streamText } from "ai"

export async function POST(request: Request) {
  const { messages } = await request.json()

  const result = streamText({
    model: "anthropic/claude-sonnet-4.5",
    messages,
  })

  return result.toUIMessageStreamResponse()
}"##;

const STREAMING_4: &str = r##"AI_GATEWAY_API_KEY=your-api-key"##;
```
