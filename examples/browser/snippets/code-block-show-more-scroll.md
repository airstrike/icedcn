```rust
use highlight::listing;
use iced::alignment::Horizontal;
use iced::widget::svg;
use iced::{Element, Fill, Font, Subscription, Task, Widget, clipboard, time};
use icedxdy::core::Border;
use icedxdy::widget::animation::cubic_bezier;
use icedxdy::widget::code_block::LineStyle;
use icedxdy::widget::{
    CodeBlock, bubble, button, code_block, column, container, markdown, message, row, tabs, text,
    tooltip, trace,
};
use icedxdy::{Size, Slot, Theme, radius, theme};
use std::collections::BTreeMap;
use std::f32::consts::PI;
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub enum Message {
    Noop,
    Expand(&'static str),
    Tick,
    Copy(&'static str, String),
    Copied(&'static str, Instant),
    ResetCopy(&'static str, Instant),
}

pub struct Demo {
    copied: BTreeMap<&'static str, Instant>,
    expanded: BTreeMap<&'static str, bool>,
    turn: Turn,
}
impl Demo {
    pub fn new() -> Self {
        Self {
            copied: BTreeMap::new(),
            expanded: BTreeMap::new(),
            turn: Turn::default(),
        }
    }
    fn event(&self, event: Message) -> Message {
        event
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Noop => {}
            Message::Expand(id) => {
                let expanded = self.expanded.entry(id).or_default();
                *expanded = !*expanded;
                self.turn
                    .retarget(if *expanded { 1. } else { 0. }, Instant::now());
            }
            Message::Tick => self.turn.advance(Instant::now()),
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
        if self.turn.start.is_some() {
            time::every(Duration::from_millis(16)).map(|_| Message::Tick)
        } else {
            Subscription::none()
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
        let id = "show-more-scroll";
        {
            let id = if id == "show-more" {
                "show-more"
            } else {
                "show-more-scroll"
            };
            let expanded = *self.expanded.get(id).unwrap_or(&false);
            let mut block = listing(dark, SHOW_MORE, usize::MAX, false, normal)
                .title(code_block::title("components/chat.tsx"))
                .start_line(1)
                .actions(self.copy(id, SHOW_MORE))
                .footer(
                    container(
                        button(
                            row![
                                chevron(self.turn.value),
                                if expanded { "Show less" } else { "Show more" }
                            ]
                            .spacing(4),
                        )
                        .size(Size::Xs)
                        .style(button::Ghost)
                        .on_press(self.event(Message::Expand(id))),
                    )
                    .center_x(Fill),
                );
            if !expanded {
                block = block.max_height(256.).collapsed(true);
            } else if id == "show-more-scroll" {
                block = block.max_height(384.);
            }
            block.into()
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

#[derive(Default)]
struct Turn {
    value: f32,
    from: f32,
    to: f32,
    start: Option<Instant>,
}
impl Turn {
    fn retarget(&mut self, to: f32, now: Instant) {
        self.advance(now);
        self.from = self.value;
        self.to = to;
        self.start = Some(now);
    }
    fn advance(&mut self, now: Instant) {
        if let Some(start) = self.start {
            let p = (now.saturating_duration_since(start).as_secs_f32() / 0.15).min(1.);
            self.value = self.from + (self.to - self.from) * cubic_bezier(0.4, 0., 0.2, 1., p);
            if p == 1. {
                self.start = None;
                self.value = self.to;
            }
        }
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
fn chevron(turn: f32) -> Slot<'static, Message> {
    svg(svg::Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m6 9 6 6 6-6"/></svg>"#))
        .width(14).height(14).rotation(turn * PI)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) }).boxed().into()
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
    &[],
    &[(1, 0x24292eff, 0xe1e4e8ff)],
    &[(3, 0x24292eff, 0xe1e4e8ff)],
    &[(12, 0x032f62ff, 0x9ecbffff)],
    &[(6, 0xd73a49ff, 0xf97583ff), (7, 0x24292eff, 0xe1e4e8ff), (8, 0x005cc5ff, 0x79b8ffff), (9, 0x24292eff, 0xe1e4e8ff), (11, 0xd73a49ff, 0xf97583ff), (18, 0x24292eff, 0xe1e4e8ff), (22, 0xd73a49ff, 0xf97583ff), (23, 0x24292eff, 0xe1e4e8ff), (30, 0x032f62ff, 0x9ecbffff)],
    &[(6, 0xd73a49ff, 0xf97583ff), (19, 0x24292eff, 0xe1e4e8ff), (23, 0xd73a49ff, 0xf97583ff), (24, 0x24292eff, 0xe1e4e8ff), (39, 0x032f62ff, 0x9ecbffff)],
    &[(6, 0xd73a49ff, 0xf97583ff), (18, 0x24292eff, 0xe1e4e8ff), (22, 0xd73a49ff, 0xf97583ff), (23, 0x24292eff, 0xe1e4e8ff), (47, 0x032f62ff, 0x9ecbffff)],
    &[(6, 0xd73a49ff, 0xf97583ff), (17, 0x24292eff, 0xe1e4e8ff), (21, 0xd73a49ff, 0xf97583ff), (22, 0x24292eff, 0xe1e4e8ff), (45, 0x032f62ff, 0x9ecbffff)],
    &[(6, 0xd73a49ff, 0xf97583ff), (7, 0x24292eff, 0xe1e4e8ff), (15, 0xd73a49ff, 0xf97583ff), (16, 0x24292eff, 0xe1e4e8ff), (20, 0x6f42c1ff, 0xb392f0ff), (24, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (9, 0x24292eff, 0xe1e4e8ff), (14, 0x005cc5ff, 0x79b8ffff), (16, 0x24292eff, 0xe1e4e8ff), (24, 0x005cc5ff, 0x79b8ffff), (26, 0x24292eff, 0xe1e4e8ff), (27, 0xd73a49ff, 0xf97583ff), (34, 0x24292eff, 0xe1e4e8ff), (42, 0x6f42c1ff, 0xb392f0ff), (43, 0x24292eff, 0xe1e4e8ff), (45, 0x032f62ff, 0x9ecbffff), (46, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff), (18, 0x005cc5ff, 0x79b8ffff), (20, 0x24292eff, 0xe1e4e8ff), (31, 0x005cc5ff, 0x79b8ffff), (33, 0x24292eff, 0xe1e4e8ff), (39, 0x005cc5ff, 0x79b8ffff), (42, 0x24292eff, 0xe1e4e8ff), (43, 0xd73a49ff, 0xf97583ff), (44, 0x24292eff, 0xe1e4e8ff), (51, 0x6f42c1ff, 0xb392f0ff), (53, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (8, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff)],
    &[(5, 0x24292eff, 0xe1e4e8ff), (8, 0x22863aff, 0x85e89dff), (9, 0x24292eff, 0xe1e4e8ff), (18, 0x6f42c1ff, 0xb392f0ff), (19, 0xd73a49ff, 0xf97583ff), (40, 0x032f62ff, 0x9ecbffff), (41, 0x24292eff, 0xe1e4e8ff)],
    &[(16, 0x24292eff, 0xe1e4e8ff), (19, 0x6f42c1ff, 0xb392f0ff), (21, 0x24292eff, 0xe1e4e8ff), (28, 0xe36209ff, 0xffab70ff), (30, 0x24292eff, 0xe1e4e8ff), (32, 0xd73a49ff, 0xf97583ff), (34, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff), (12, 0x22863aff, 0x85e89dff), (13, 0x24292eff, 0xe1e4e8ff), (16, 0x6f42c1ff, 0xb392f0ff), (17, 0xd73a49ff, 0xf97583ff), (30, 0x24292eff, 0xe1e4e8ff)],
    &[(25, 0x24292eff, 0xe1e4e8ff), (28, 0x6f42c1ff, 0xb392f0ff), (30, 0x24292eff, 0xe1e4e8ff), (34, 0xe36209ff, 0xffab70ff), (36, 0x24292eff, 0xe1e4e8ff), (41, 0xe36209ff, 0xffab70ff), (43, 0x24292eff, 0xe1e4e8ff), (45, 0xd73a49ff, 0xf97583ff)],
    &[(22, 0x24292eff, 0xe1e4e8ff), (25, 0xd73a49ff, 0xf97583ff), (26, 0x24292eff, 0xe1e4e8ff), (32, 0x032f62ff, 0x9ecbffff), (33, 0x24292eff, 0xe1e4e8ff), (34, 0xd73a49ff, 0xf97583ff), (36, 0x24292eff, 0xe1e4e8ff), (37, 0x22863aff, 0x85e89dff), (38, 0x24292eff, 0xe1e4e8ff), (41, 0x6f42c1ff, 0xb392f0ff), (42, 0xd73a49ff, 0xf97583ff), (63, 0x24292eff, 0xe1e4e8ff), (64, 0x22863aff, 0x85e89dff), (66, 0x24292eff, 0xe1e4e8ff), (67, 0xd73a49ff, 0xf97583ff), (68, 0x24292eff, 0xe1e4e8ff), (72, 0x005cc5ff, 0x79b8ffff)],
    &[(12, 0x24292eff, 0xe1e4e8ff)],
    &[(10, 0x24292eff, 0xe1e4e8ff), (13, 0x22863aff, 0x85e89dff), (14, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff)],
    &[(7, 0x24292eff, 0xe1e4e8ff), (11, 0x22863aff, 0x85e89dff)],
    &[(8, 0x24292eff, 0xe1e4e8ff), (16, 0x6f42c1ff, 0xb392f0ff), (17, 0xd73a49ff, 0xf97583ff), (19, 0x24292eff, 0xe1e4e8ff), (24, 0xe36209ff, 0xffab70ff), (26, 0x24292eff, 0xe1e4e8ff), (28, 0xd73a49ff, 0xf97583ff), (30, 0x24292eff, 0xe1e4e8ff)],
    &[(16, 0x24292eff, 0xe1e4e8ff), (30, 0x6f42c1ff, 0xb392f0ff), (32, 0x24292eff, 0xe1e4e8ff)],
    &[(10, 0x24292eff, 0xe1e4e8ff), (21, 0x6f42c1ff, 0xb392f0ff), (38, 0x24292eff, 0xe1e4e8ff)],
    &[(10, 0x24292eff, 0xe1e4e8ff), (18, 0x6f42c1ff, 0xb392f0ff), (19, 0x24292eff, 0xe1e4e8ff), (21, 0x032f62ff, 0x9ecbffff), (22, 0x24292eff, 0xe1e4e8ff)],
    &[(10, 0x24292eff, 0xe1e4e8ff)],
    &[(7, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff), (14, 0x005cc5ff, 0x79b8ffff), (15, 0x24292eff, 0xe1e4e8ff), (20, 0x6f42c1ff, 0xb392f0ff), (21, 0xd73a49ff, 0xf97583ff), (29, 0x24292eff, 0xe1e4e8ff), (37, 0x6f42c1ff, 0xb392f0ff), (38, 0xd73a49ff, 0xf97583ff), (40, 0x24292eff, 0xe1e4e8ff), (45, 0xe36209ff, 0xffab70ff), (47, 0x24292eff, 0xe1e4e8ff), (49, 0xd73a49ff, 0xf97583ff), (50, 0x24292eff, 0xe1e4e8ff), (58, 0x6f42c1ff, 0xb392f0ff), (82, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff), (15, 0x005cc5ff, 0x79b8ffff), (16, 0x24292eff, 0xe1e4e8ff), (24, 0x6f42c1ff, 0xb392f0ff), (25, 0xd73a49ff, 0xf97583ff), (33, 0x24292eff, 0xe1e4e8ff), (36, 0xd73a49ff, 0xf97583ff), (37, 0x24292eff, 0xe1e4e8ff), (44, 0x032f62ff, 0x9ecbffff), (52, 0x24292eff, 0xe1e4e8ff), (58, 0x005cc5ff, 0x79b8ffff), (59, 0x24292eff, 0xe1e4e8ff)],
    &[(8, 0x24292eff, 0xe1e4e8ff), (12, 0x22863aff, 0x85e89dff), (13, 0x24292eff, 0xe1e4e8ff)],
    &[(6, 0x24292eff, 0xe1e4e8ff), (9, 0x22863aff, 0x85e89dff), (10, 0x24292eff, 0xe1e4e8ff)],
];
    #[rustfmt::skip]
static FIXTURES: &[Fixture] = &[
    Fixture { source: SHOW_MORE, rows: &[3, 0, 4, 5, 0, 6, 7, 0, 8, 9, 10, 0, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 2, 1], tails: &[] },
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

const SHOW_MORE: &str = r##""use client"

import * as React from "react"
import { useChat } from "@ai-sdk/react"

import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"

export function Chat() {
  const [input, setInput] = React.useState("")
  const { messages, sendMessage, status } = useChat()

  return (
    <div className="flex flex-col gap-4">
      {messages.map((message) => (
        <div key={message.id}>
          {message.parts.map((part, index) =>
            part.type === "text" ? <p key={index}>{part.text}</p> : null
          )}
        </div>
      ))}
      <form
        onSubmit={(event) => {
          event.preventDefault()
          sendMessage({ text: input })
          setInput("")
        }}
      >
        <Input value={input} onChange={(event) => setInput(event.target.value)} />
        <Button disabled={status !== "ready"}>Send</Button>
      </form>
    </div>
  )
}"##;
```
