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
    Wrap(&'static str),
    Copy(&'static str, String),
    Copied(&'static str, Instant),
    ResetCopy(&'static str, Instant),
    Save(&'static str, &'static str),
    Saved(&'static str, Result<bool, String>),
}

pub struct Demo {
    copied: BTreeMap<&'static str, Instant>,
    save_status: BTreeMap<&'static str, String>,
    wraps: BTreeMap<&'static str, bool>,
}
impl Demo {
    pub fn new() -> Self {
        Self {
            copied: BTreeMap::new(),
            save_status: BTreeMap::new(),
            wraps: BTreeMap::new(),
        }
    }
    fn event(&self, event: Message) -> Message {
        event
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Noop => {}
            Message::Wrap(id) => {
                let wrap = self.wraps.entry(id).or_insert(id == "actions");
                *wrap = !*wrap;
            }
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
            Message::Save(name, value) => {
                return Task::perform(
                    async move {
                        let file = rfd::AsyncFileDialog::new()
                            .set_file_name(name)
                            .save_file()
                            .await;
                        match file {
                            Some(file) => file
                                .write(value.as_bytes())
                                .await
                                .map(|()| true)
                                .map_err(|e| e.to_string()),
                            None => Ok(false),
                        }
                    },
                    move |result| Message::Saved(name, result),
                );
            }
            Message::Saved(name, Ok(true)) => {
                self.save_status.insert(name, format!("Saved {name}"));
            }
            Message::Saved(_, Ok(false)) => {}
            Message::Saved(name, Err(error)) => {
                self.save_status
                    .insert(name, format!("Could not save {name}: {error}"));
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
            .max_width(512)
            .into()
    }
    fn content(&self, dark: bool) -> Slot<'_, Message> {
        let normal = |_: usize, _: &str| LineStyle::Default;
        let id = "demo";
        {
            let wrap = *self.wraps.get("demo").unwrap_or(&false);
            let block = listing(dark, DEMO, usize::MAX, wrap, |i, _| {
                if (5..=9).contains(&i) {
                    LineStyle::Highlighted
                } else {
                    LineStyle::Default
                }
            })
            .start_line(1);
            self.header("demo", "app/api/chat/route.ts", DEMO, "route.ts", block)
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
    fn header(
        &self,
        id: &'static str,
        title: &'static str,
        source: &'static str,
        filename: &'static str,
        block: CodeBlock<'static, Message>,
    ) -> Slot<'static, Message> {
        let wrap = action(
            icon("wrap_text"),
            "Wrap lines",
            self.event(Message::Wrap(id)),
        );
        let save = action(
            icon("download"),
            "Download",
            self.event(Message::Save(filename, source)),
        );
        let mut block = block
            .title(
                row![
                    if id == "actions" {
                        icon("terminal")
                    } else {
                        icon("file_code")
                    },
                    code_block::title(title)
                ]
                .spacing(6),
            )
            .actions(row![wrap, save, self.copy(id, source)].spacing(4));
        if let Some(status) = self.save_status.get(filename) {
            block = block.footer(text::muted(status.clone()));
        }
        block.into()
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
        "download" => {
            r#"<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4M7 10l5 5 5-5M12 15V3"/>"#
        }
        "file_code" => {
            r#"<path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8zM14 2v6h6m-10 5-2 2 2 2m4-4 2 2-2 2"/>"#
        }
        "terminal" => r#"<path d="m4 17 6-5-6-5m8 12h8"/>"#,
        "wrap_text" => r#"<path d="M3 6h18M3 12h15a3 3 0 0 1 0 6h-4m2-2-2 2 2 2M3 18h7"/>"#,
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

mod tokens {
    //! Fixed recipe tokens generated with pinned Shiki 3.23.0, JavaScript regex engine,
    //! github-light/github-dark. Partial final lines retain streaming grammar behavior.
    //! This fixture data does not implement a general-purpose highlighter.
    use super::*;

    pub type Run = (usize, u32, u32);
    struct Fixture {
        source: &'static str,
        rows: &'static [usize],
        tails: &'static [(usize, usize)],
    }
    #[rustfmt::skip]
static ROWS: &[&[Run]] = &[
    &[(6, 0xd73a49ff, 0xf97583ff), (22, 0x24292eff, 0xe1e4e8ff), (26, 0xd73a49ff, 0xf97583ff), (27, 0x24292eff, 0xe1e4e8ff), (31, 0x032f62ff, 0x9ecbffff)],
    &[],
    &[(6, 0xd73a49ff, 0xf97583ff), (7, 0x24292eff, 0xe1e4e8ff), (12, 0xd73a49ff, 0xf97583ff), (13, 0x24292eff, 0xe1e4e8ff), (21, 0xd73a49ff, 0xf97583ff), (22, 0x24292eff, 0xe1e4e8ff), (26, 0x6f42c1ff, 0xb392f0ff), (27, 0x24292eff, 0xe1e4e8ff), (34, 0xe36209ff, 0xffab70ff), (35, 0xd73a49ff, 0xf97583ff), (36, 0x24292eff, 0xe1e4e8ff), (43, 0x6f42c1ff, 0xb392f0ff), (46, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff), (18, 0x005cc5ff, 0x79b8ffff), (21, 0x24292eff, 0xe1e4e8ff), (22, 0xd73a49ff, 0xf97583ff), (23, 0x24292eff, 0xe1e4e8ff), (28, 0xd73a49ff, 0xf97583ff), (37, 0x24292eff, 0xe1e4e8ff), (41, 0x6f42c1ff, 0xb392f0ff), (43, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (8, 0x24292eff, 0xe1e4e8ff), (14, 0x005cc5ff, 0x79b8ffff), (15, 0x24292eff, 0xe1e4e8ff), (16, 0xd73a49ff, 0xf97583ff), (17, 0x24292eff, 0xe1e4e8ff), (27, 0x6f42c1ff, 0xb392f0ff), (29, 0x24292eff, 0xe1e4e8ff)],
    &[(11, 0x24292eff, 0xe1e4e8ff), (40, 0x032f62ff, 0x9ecbffff), (41, 0x24292eff, 0xe1e4e8ff)],
    &[(12, 0x24292eff, 0xe1e4e8ff), (73, 0x032f62ff, 0x9ecbffff), (74, 0x24292eff, 0xe1e4e8ff)],
    &[(13, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (8, 0xd73a49ff, 0xf97583ff), (16, 0x24292eff, 0xe1e4e8ff), (41, 0x6f42c1ff, 0xb392f0ff), (43, 0x24292eff, 0xe1e4e8ff)],
    &[(1, 0x24292eff, 0xe1e4e8ff)],
];
    #[rustfmt::skip]
static FIXTURES: &[Fixture] = &[
    Fixture { source: DEMO, rows: &[0, 1, 2, 3, 1, 4, 5, 6, 7, 8, 1, 9, 10], tails: &[] },
];
    pub fn row(source: &str, index: usize, visible: usize, last: bool) -> Option<&'static [Run]> {
        let fixture = FIXTURES
            .iter()
            .rev()
            .find(|fixture| fixture.source == source)?;
        if last
            && let Ok(i) = fixture
                .tails
                .binary_search_by_key(&visible, |(end, _)| *end)
        {
            return Some(ROWS[fixture.tails[i].1]);
        }
        fixture.rows.get(index).map(|row| ROWS[*row])
    }
}

mod highlight {
    //! Render retained pinned Shiki runs for these fixed fixtures; no parsing in view.
    use super::tokens;
    use super::*;

    use iced::widget::text::{LineHeight, Wrapping};
    use iced::{Fill, Font, Shrink};
    use icedxdy::theme::tweak::Token;
    use icedxdy::widget::code_block::{CodeBlock, LineStyle};
    use icedxdy::widget::{rich_text, span};
    use icedxdy::{Color, Slot};

    fn rgb(hex: u32) -> Token {
        Color::from(iced::Color::from_rgba8(
            (hex >> 24) as u8,
            (hex >> 16) as u8,
            (hex >> 8) as u8,
            (hex & 255) as f32 / 255.,
        ))
        .into()
    }

    /// Render a retained prefix, including the grammar of an unfinished final line.
    pub fn listing<Message: 'static>(
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
        let mut end = visible.min(source.len());
        while !source.is_char_boundary(end) {
            end -= 1;
        }
        let last = source[..end].bytes().filter(|b| *b == b'\n').count();
        let lines = source[..end].split('\n').enumerate().map(|(i, line)| {
            let original = line;
            let line = if false {
                line.get(1..).unwrap_or("")
            } else {
                line
            };
            let mut spans = Vec::new();
            if let Some(runs) = tokens::row(source, i, end, i == last) {
                let mut pos = 0;
                for &(end, light, night) in runs {
                    let start = pos;
                    let end = end.min(line.len());
                    if start > pos {
                        spans.push(span(&line[pos..start]));
                    }
                    if end > start {
                        spans.push(span(&line[start..end]).color(rgb(if dark {
                            night
                        } else {
                            light
                        })));
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
                .line_height(LineHeight::Absolute(22.75.into()))
                .wrapping(wrapping)
                .width(if wrap { Fill } else { Shrink });
            (styles(i, original), Slot::from(text))
        });
        CodeBlock::lines(lines)
            .wrap(wrap)
            .font(Font::new("Geist Mono"))
    }
}

const DEMO: &str = r##"import { streamText } from "ai"

export async function POST(request: Request) {
  const { messages } = await request.json()

  const result = streamText({
    model: "anthropic/claude-sonnet-4.5",
    system: "You are a helpful assistant. Answer in one short paragraph.",
    messages,
  })

  return result.toUIMessageStreamResponse()
}"##;
```
