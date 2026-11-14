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
        let id = "start-line";
        listing(dark, START_LINE, usize::MAX, false, normal)
            .title(code_block::title("src/notebooks.rs"))
            .start_line(995)
            .into()
    }
}

mod tokens {
    //! Fixed recipe tokens generated with pinned Shiki 3.23.0,
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
    &[],
    &[(1, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (14, 0x24292eff, 0xe1e4e8ff), (15, 0xd73a49ff, 0xf97583ff), (16, 0x24292eff, 0xe1e4e8ff), (20, 0x005cc5ff, 0x79b8ffff), (21, 0xd73a49ff, 0xf97583ff), (26, 0x24292eff, 0xe1e4e8ff), (27, 0xd73a49ff, 0xf97583ff), (31, 0x6f42c1ff, 0xb392f0ff), (33, 0x24292eff, 0xe1e4e8ff), (34, 0xd73a49ff, 0xf97583ff), (46, 0x6f42c1ff, 0xb392f0ff), (49, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (8, 0x005cc5ff, 0x79b8ffff), (9, 0xd73a49ff, 0xf97583ff), (18, 0x24292eff, 0xe1e4e8ff)],
    &[(8, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (13, 0x6f42c1ff, 0xb392f0ff), (15, 0x24292eff, 0xe1e4e8ff)],
    &[(8, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (15, 0x6f42c1ff, 0xb392f0ff), (16, 0x24292eff, 0xe1e4e8ff), (17, 0xd73a49ff, 0xf97583ff), (21, 0x24292eff, 0xe1e4e8ff), (22, 0xd73a49ff, 0xf97583ff), (27, 0x24292eff, 0xe1e4e8ff), (28, 0xd73a49ff, 0xf97583ff), (33, 0x24292eff, 0xe1e4e8ff), (34, 0xd73a49ff, 0xf97583ff), (46, 0x6f42c1ff, 0xb392f0ff), (48, 0x24292eff, 0xe1e4e8ff), (49, 0xd73a49ff, 0xf97583ff), (57, 0x6f42c1ff, 0xb392f0ff), (58, 0x24292eff, 0xe1e4e8ff), (59, 0xd73a49ff, 0xf97583ff), (66, 0x24292eff, 0xe1e4e8ff)],
    &[(8, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (12, 0x6f42c1ff, 0xb392f0ff), (13, 0x24292eff, 0xe1e4e8ff), (14, 0xd73a49ff, 0xf97583ff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (24, 0x24292eff, 0xe1e4e8ff), (25, 0xd73a49ff, 0xf97583ff), (30, 0x24292eff, 0xe1e4e8ff), (31, 0xd73a49ff, 0xf97583ff), (37, 0x6f42c1ff, 0xb392f0ff), (40, 0x24292eff, 0xe1e4e8ff)],
    &[(8, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (16, 0x6f42c1ff, 0xb392f0ff), (18, 0x24292eff, 0xe1e4e8ff)],
];
    #[rustfmt::skip]
static FIXTURES: &[Fixture] = &[
    Fixture { source: START_LINE, rows: &[2, 0, 3, 4, 5, 6, 7, 1], tails: &[] },
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

const START_LINE: &str = r##"    let query = self.query.trim().to_lowercase();

    self.notebooks
        .iter()
        .filter(|book| book.title.to_lowercase().contains(&query))
        .map(|book| book.title.as_str())
        .collect()
}"##;
```
