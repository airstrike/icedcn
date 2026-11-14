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
        let mut output = column![].width(Fill).spacing(16);
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
            if !matches!(title, "Terminal" | ".cargo/config.toml") {
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
fn replay_icon() -> Slot<'static, Message> {
    svg(svg::Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"/><path d="M3 3v5h5"/></svg>"#))
        .width(14).height(14).style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) }).boxed().into()
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
    &[(5, 0x24292eff, 0xe1e4e8ff)],
    &[(1, 0x24292eff, 0xe1e4e8ff)],
    &[(8, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (13, 0x6f42c1ff, 0xb392f0ff), (15, 0x24292eff, 0xe1e4e8ff)],
    &[(8, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (16, 0x6f42c1ff, 0xb392f0ff), (18, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0xd73a49ff, 0xf97583ff), (3, 0x24292eff, 0xe1e4e8ff), (10, 0x6f42c1ff, 0xb392f0ff), (16, 0x24292eff, 0xe1e4e8ff), (17, 0xd73a49ff, 0xf97583ff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (22, 0x6f42c1ff, 0xb392f0ff), (29, 0x24292eff, 0xe1e4e8ff), (30, 0xd73a49ff, 0xf97583ff), (31, 0x24292eff, 0xe1e4e8ff), (32, 0xd73a49ff, 0xf97583ff), (35, 0x6f42c1ff, 0xb392f0ff), (37, 0x24292eff, 0xe1e4e8ff), (39, 0xd73a49ff, 0xf97583ff), (40, 0x24292eff, 0xe1e4e8ff), (44, 0x6f42c1ff, 0xb392f0ff), (46, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (22, 0x6f42c1ff, 0xb392f0ff), (24, 0x24292eff, 0xe1e4e8ff), (25, 0xd73a49ff, 0xf97583ff), (33, 0x6f42c1ff, 0xb392f0ff), (34, 0x24292eff, 0xe1e4e8ff), (35, 0xd73a49ff, 0xf97583ff), (40, 0x24292eff, 0xe1e4e8ff), (41, 0xd73a49ff, 0xf97583ff), (53, 0x6f42c1ff, 0xb392f0ff), (56, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0xd73a49ff, 0xf97583ff), (3, 0x24292eff, 0xe1e4e8ff), (9, 0x6f42c1ff, 0xb392f0ff), (11, 0x24292eff, 0xe1e4e8ff), (12, 0x6f42c1ff, 0xb392f0ff), (19, 0x24292eff, 0xe1e4e8ff), (20, 0xd73a49ff, 0xf97583ff), (21, 0x24292eff, 0xe1e4e8ff), (22, 0xd73a49ff, 0xf97583ff), (25, 0x6f42c1ff, 0xb392f0ff), (33, 0x24292eff, 0xe1e4e8ff), (34, 0xd73a49ff, 0xf97583ff), (35, 0x24292eff, 0xe1e4e8ff), (36, 0xd73a49ff, 0xf97583ff), (37, 0x24292eff, 0xe1e4e8ff), (38, 0xd73a49ff, 0xf97583ff), (39, 0x24292eff, 0xe1e4e8ff), (40, 0x6f42c1ff, 0xb392f0ff), (41, 0x24292eff, 0xe1e4e8ff), (44, 0x6f42c1ff, 0xb392f0ff), (47, 0x24292eff, 0xe1e4e8ff), (49, 0xd73a49ff, 0xf97583ff), (50, 0x24292eff, 0xe1e4e8ff), (53, 0x6f42c1ff, 0xb392f0ff), (54, 0x24292eff, 0xe1e4e8ff), (55, 0xd73a49ff, 0xf97583ff), (56, 0x24292eff, 0xe1e4e8ff), (57, 0x6f42c1ff, 0xb392f0ff), (58, 0x24292eff, 0xe1e4e8ff), (61, 0x6f42c1ff, 0xb392f0ff), (64, 0x24292eff, 0xe1e4e8ff)],
    &[(10, 0x24292eff, 0xe1e4e8ff)],
    &[(8, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (15, 0x6f42c1ff, 0xb392f0ff), (17, 0x24292eff, 0xe1e4e8ff)],
    &[(8, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (15, 0x6f42c1ff, 0xb392f0ff), (16, 0x24292eff, 0xe1e4e8ff), (17, 0xd73a49ff, 0xf97583ff), (22, 0x24292eff, 0xe1e4e8ff), (23, 0xd73a49ff, 0xf97583ff), (24, 0x24292eff, 0xe1e4e8ff), (31, 0x6f42c1ff, 0xb392f0ff), (46, 0x24292eff, 0xe1e4e8ff)],
    &[(1, 0x24292eff, 0xe1e4e8ff), (4, 0x6f42c1ff, 0xb392f0ff), (5, 0x24292eff, 0xe1e4e8ff)],
    &[(11, 0x24292eff, 0xe1e4e8ff), (27, 0x032f62ff, 0x9ecbffff)],
    &[(2, 0xd73a49ff, 0xf97583ff), (4, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0xd73a49ff, 0xf97583ff), (5, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0xd73a49ff, 0xf97583ff), (6, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0xd73a49ff, 0xf97583ff), (7, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0xd73a49ff, 0xf97583ff), (3, 0x24292eff, 0xe1e4e8ff), (8, 0xd73a49ff, 0xf97583ff)],
    &[(2, 0xd73a49ff, 0xf97583ff), (9, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0xd73a49ff, 0xf97583ff), (3, 0x24292eff, 0xe1e4e8ff), (10, 0x6f42c1ff, 0xb392f0ff), (16, 0x24292eff, 0xe1e4e8ff), (17, 0xd73a49ff, 0xf97583ff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (20, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0xd73a49ff, 0xf97583ff), (3, 0x24292eff, 0xe1e4e8ff), (10, 0x6f42c1ff, 0xb392f0ff), (16, 0x24292eff, 0xe1e4e8ff), (17, 0xd73a49ff, 0xf97583ff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (21, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0xd73a49ff, 0xf97583ff), (3, 0x24292eff, 0xe1e4e8ff), (10, 0x6f42c1ff, 0xb392f0ff), (16, 0x24292eff, 0xe1e4e8ff), (17, 0xd73a49ff, 0xf97583ff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (22, 0x6f42c1ff, 0xb392f0ff), (29, 0x24292eff, 0xe1e4e8ff), (30, 0xd73a49ff, 0xf97583ff), (31, 0x24292eff, 0xe1e4e8ff), (32, 0xd73a49ff, 0xf97583ff), (33, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0xd73a49ff, 0xf97583ff), (3, 0x24292eff, 0xe1e4e8ff), (10, 0x6f42c1ff, 0xb392f0ff), (16, 0x24292eff, 0xe1e4e8ff), (17, 0xd73a49ff, 0xf97583ff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (22, 0x6f42c1ff, 0xb392f0ff), (29, 0x24292eff, 0xe1e4e8ff), (30, 0xd73a49ff, 0xf97583ff), (31, 0x24292eff, 0xe1e4e8ff), (32, 0xd73a49ff, 0xf97583ff), (34, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0xd73a49ff, 0xf97583ff), (3, 0x24292eff, 0xe1e4e8ff), (10, 0x6f42c1ff, 0xb392f0ff), (16, 0x24292eff, 0xe1e4e8ff), (17, 0xd73a49ff, 0xf97583ff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (22, 0x6f42c1ff, 0xb392f0ff), (29, 0x24292eff, 0xe1e4e8ff), (30, 0xd73a49ff, 0xf97583ff), (31, 0x24292eff, 0xe1e4e8ff), (32, 0xd73a49ff, 0xf97583ff), (35, 0x6f42c1ff, 0xb392f0ff), (37, 0x24292eff, 0xe1e4e8ff), (39, 0xd73a49ff, 0xf97583ff), (41, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0xd73a49ff, 0xf97583ff), (3, 0x24292eff, 0xe1e4e8ff), (10, 0x6f42c1ff, 0xb392f0ff), (16, 0x24292eff, 0xe1e4e8ff), (17, 0xd73a49ff, 0xf97583ff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (22, 0x6f42c1ff, 0xb392f0ff), (29, 0x24292eff, 0xe1e4e8ff), (30, 0xd73a49ff, 0xf97583ff), (31, 0x24292eff, 0xe1e4e8ff), (32, 0xd73a49ff, 0xf97583ff), (35, 0x6f42c1ff, 0xb392f0ff), (37, 0x24292eff, 0xe1e4e8ff), (39, 0xd73a49ff, 0xf97583ff), (42, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0xd73a49ff, 0xf97583ff), (3, 0x24292eff, 0xe1e4e8ff), (10, 0x6f42c1ff, 0xb392f0ff), (16, 0x24292eff, 0xe1e4e8ff), (17, 0xd73a49ff, 0xf97583ff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (22, 0x6f42c1ff, 0xb392f0ff), (29, 0x24292eff, 0xe1e4e8ff), (30, 0xd73a49ff, 0xf97583ff), (31, 0x24292eff, 0xe1e4e8ff), (32, 0xd73a49ff, 0xf97583ff), (35, 0x6f42c1ff, 0xb392f0ff), (37, 0x24292eff, 0xe1e4e8ff), (39, 0xd73a49ff, 0xf97583ff), (43, 0x24292eff, 0xe1e4e8ff)],
    &[(6, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (14, 0x24292eff, 0xe1e4e8ff), (15, 0xd73a49ff, 0xf97583ff), (21, 0x24292eff, 0xe1e4e8ff), (22, 0xd73a49ff, 0xf97583ff), (23, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (14, 0x24292eff, 0xe1e4e8ff), (15, 0xd73a49ff, 0xf97583ff), (21, 0x24292eff, 0xe1e4e8ff), (22, 0xd73a49ff, 0xf97583ff), (24, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (14, 0x24292eff, 0xe1e4e8ff), (15, 0xd73a49ff, 0xf97583ff), (21, 0x24292eff, 0xe1e4e8ff), (22, 0xd73a49ff, 0xf97583ff), (25, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (14, 0x24292eff, 0xe1e4e8ff), (15, 0xd73a49ff, 0xf97583ff), (21, 0x24292eff, 0xe1e4e8ff), (22, 0xd73a49ff, 0xf97583ff), (26, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (14, 0x24292eff, 0xe1e4e8ff), (15, 0xd73a49ff, 0xf97583ff), (21, 0x24292eff, 0xe1e4e8ff), (22, 0xd73a49ff, 0xf97583ff), (26, 0x6f42c1ff, 0xb392f0ff), (28, 0x24292eff, 0xe1e4e8ff), (29, 0xd73a49ff, 0xf97583ff), (30, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (14, 0x24292eff, 0xe1e4e8ff), (15, 0xd73a49ff, 0xf97583ff), (21, 0x24292eff, 0xe1e4e8ff), (22, 0xd73a49ff, 0xf97583ff), (26, 0x6f42c1ff, 0xb392f0ff), (28, 0x24292eff, 0xe1e4e8ff), (29, 0xd73a49ff, 0xf97583ff), (31, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (14, 0x24292eff, 0xe1e4e8ff), (15, 0xd73a49ff, 0xf97583ff), (21, 0x24292eff, 0xe1e4e8ff), (22, 0xd73a49ff, 0xf97583ff), (26, 0x6f42c1ff, 0xb392f0ff), (28, 0x24292eff, 0xe1e4e8ff), (29, 0xd73a49ff, 0xf97583ff), (32, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (14, 0x24292eff, 0xe1e4e8ff), (15, 0xd73a49ff, 0xf97583ff), (21, 0x24292eff, 0xe1e4e8ff), (22, 0xd73a49ff, 0xf97583ff), (26, 0x6f42c1ff, 0xb392f0ff), (28, 0x24292eff, 0xe1e4e8ff), (29, 0xd73a49ff, 0xf97583ff), (33, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (14, 0x24292eff, 0xe1e4e8ff), (15, 0xd73a49ff, 0xf97583ff), (21, 0x24292eff, 0xe1e4e8ff), (22, 0xd73a49ff, 0xf97583ff), (26, 0x6f42c1ff, 0xb392f0ff), (28, 0x24292eff, 0xe1e4e8ff), (29, 0xd73a49ff, 0xf97583ff), (34, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (14, 0x24292eff, 0xe1e4e8ff), (15, 0xd73a49ff, 0xf97583ff), (21, 0x24292eff, 0xe1e4e8ff), (22, 0xd73a49ff, 0xf97583ff), (26, 0x6f42c1ff, 0xb392f0ff), (28, 0x24292eff, 0xe1e4e8ff), (29, 0xd73a49ff, 0xf97583ff), (35, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (14, 0x24292eff, 0xe1e4e8ff), (15, 0xd73a49ff, 0xf97583ff), (21, 0x24292eff, 0xe1e4e8ff), (22, 0xd73a49ff, 0xf97583ff), (26, 0x6f42c1ff, 0xb392f0ff), (28, 0x24292eff, 0xe1e4e8ff), (29, 0xd73a49ff, 0xf97583ff), (36, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (14, 0x24292eff, 0xe1e4e8ff), (15, 0xd73a49ff, 0xf97583ff), (21, 0x24292eff, 0xe1e4e8ff), (22, 0xd73a49ff, 0xf97583ff), (26, 0x6f42c1ff, 0xb392f0ff), (28, 0x24292eff, 0xe1e4e8ff), (29, 0xd73a49ff, 0xf97583ff), (37, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (14, 0x24292eff, 0xe1e4e8ff), (15, 0xd73a49ff, 0xf97583ff), (21, 0x24292eff, 0xe1e4e8ff), (22, 0xd73a49ff, 0xf97583ff), (26, 0x6f42c1ff, 0xb392f0ff), (28, 0x24292eff, 0xe1e4e8ff), (29, 0xd73a49ff, 0xf97583ff), (38, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (14, 0x24292eff, 0xe1e4e8ff), (15, 0xd73a49ff, 0xf97583ff), (21, 0x24292eff, 0xe1e4e8ff), (22, 0xd73a49ff, 0xf97583ff), (26, 0x6f42c1ff, 0xb392f0ff), (28, 0x24292eff, 0xe1e4e8ff), (29, 0xd73a49ff, 0xf97583ff), (39, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (14, 0x24292eff, 0xe1e4e8ff), (15, 0xd73a49ff, 0xf97583ff), (21, 0x24292eff, 0xe1e4e8ff), (22, 0xd73a49ff, 0xf97583ff), (26, 0x6f42c1ff, 0xb392f0ff), (28, 0x24292eff, 0xe1e4e8ff), (29, 0xd73a49ff, 0xf97583ff), (40, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (14, 0x24292eff, 0xe1e4e8ff), (15, 0xd73a49ff, 0xf97583ff), (21, 0x24292eff, 0xe1e4e8ff), (22, 0xd73a49ff, 0xf97583ff), (26, 0x6f42c1ff, 0xb392f0ff), (28, 0x24292eff, 0xe1e4e8ff), (29, 0xd73a49ff, 0xf97583ff), (41, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (11, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (12, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (13, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (14, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (15, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (16, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (17, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (18, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (19, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (20, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (21, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (22, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (22, 0x6f42c1ff, 0xb392f0ff), (24, 0x24292eff, 0xe1e4e8ff), (25, 0xd73a49ff, 0xf97583ff), (26, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (22, 0x6f42c1ff, 0xb392f0ff), (24, 0x24292eff, 0xe1e4e8ff), (25, 0xd73a49ff, 0xf97583ff), (27, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (22, 0x6f42c1ff, 0xb392f0ff), (24, 0x24292eff, 0xe1e4e8ff), (25, 0xd73a49ff, 0xf97583ff), (28, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (22, 0x6f42c1ff, 0xb392f0ff), (24, 0x24292eff, 0xe1e4e8ff), (25, 0xd73a49ff, 0xf97583ff), (29, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (22, 0x6f42c1ff, 0xb392f0ff), (24, 0x24292eff, 0xe1e4e8ff), (25, 0xd73a49ff, 0xf97583ff), (30, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (22, 0x6f42c1ff, 0xb392f0ff), (24, 0x24292eff, 0xe1e4e8ff), (25, 0xd73a49ff, 0xf97583ff), (31, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (22, 0x6f42c1ff, 0xb392f0ff), (24, 0x24292eff, 0xe1e4e8ff), (25, 0xd73a49ff, 0xf97583ff), (32, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (22, 0x6f42c1ff, 0xb392f0ff), (24, 0x24292eff, 0xe1e4e8ff), (25, 0xd73a49ff, 0xf97583ff), (33, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (14, 0x24292eff, 0xe1e4e8ff), (15, 0xd73a49ff, 0xf97583ff), (21, 0x24292eff, 0xe1e4e8ff), (22, 0xd73a49ff, 0xf97583ff), (26, 0x6f42c1ff, 0xb392f0ff), (28, 0x24292eff, 0xe1e4e8ff), (29, 0xd73a49ff, 0xf97583ff), (41, 0x6f42c1ff, 0xb392f0ff), (44, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (22, 0x6f42c1ff, 0xb392f0ff), (24, 0x24292eff, 0xe1e4e8ff), (25, 0xd73a49ff, 0xf97583ff), (33, 0x6f42c1ff, 0xb392f0ff), (34, 0x24292eff, 0xe1e4e8ff), (35, 0xd73a49ff, 0xf97583ff), (41, 0x24292eff, 0xe1e4e8ff)],
    &[(5, 0x6f42c1ff, 0xb392f0ff), (6, 0x24292eff, 0xe1e4e8ff), (10, 0x032f62ff, 0x9ecbffff), (11, 0x24292eff, 0xe1e4e8ff), (12, 0x032f62ff, 0x9ecbffff)],
    &[(5, 0x6f42c1ff, 0xb392f0ff), (6, 0x24292eff, 0xe1e4e8ff), (10, 0x032f62ff, 0x9ecbffff), (11, 0x24292eff, 0xe1e4e8ff), (16, 0x005cc5ff, 0x79b8ffff), (17, 0x24292eff, 0xe1e4e8ff), (23, 0x032f62ff, 0x9ecbffff)],
    &[(2, 0xd73a49ff, 0xf97583ff), (8, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0xd73a49ff, 0xf97583ff), (3, 0x24292eff, 0xe1e4e8ff), (9, 0x6f42c1ff, 0xb392f0ff), (10, 0x24292eff, 0xe1e4e8ff), (11, 0x032f62ff, 0x9ecbffff)],
    &[(2, 0xd73a49ff, 0xf97583ff), (3, 0x24292eff, 0xe1e4e8ff), (9, 0x6f42c1ff, 0xb392f0ff), (11, 0x24292eff, 0xe1e4e8ff), (12, 0x6f42c1ff, 0xb392f0ff), (19, 0x24292eff, 0xe1e4e8ff), (20, 0xd73a49ff, 0xf97583ff), (21, 0x24292eff, 0xe1e4e8ff), (22, 0xd73a49ff, 0xf97583ff), (23, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0xd73a49ff, 0xf97583ff), (3, 0x24292eff, 0xe1e4e8ff), (9, 0x6f42c1ff, 0xb392f0ff), (11, 0x24292eff, 0xe1e4e8ff), (12, 0x6f42c1ff, 0xb392f0ff), (19, 0x24292eff, 0xe1e4e8ff), (20, 0xd73a49ff, 0xf97583ff), (21, 0x24292eff, 0xe1e4e8ff), (22, 0xd73a49ff, 0xf97583ff), (24, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0xd73a49ff, 0xf97583ff), (3, 0x24292eff, 0xe1e4e8ff), (9, 0x6f42c1ff, 0xb392f0ff), (11, 0x24292eff, 0xe1e4e8ff), (12, 0x6f42c1ff, 0xb392f0ff), (19, 0x24292eff, 0xe1e4e8ff), (20, 0xd73a49ff, 0xf97583ff), (21, 0x24292eff, 0xe1e4e8ff), (22, 0xd73a49ff, 0xf97583ff), (25, 0x6f42c1ff, 0xb392f0ff), (33, 0x24292eff, 0xe1e4e8ff), (34, 0xd73a49ff, 0xf97583ff), (35, 0x24292eff, 0xe1e4e8ff), (36, 0xd73a49ff, 0xf97583ff), (37, 0x24292eff, 0xe1e4e8ff), (38, 0xd73a49ff, 0xf97583ff), (39, 0x032f62ff, 0x9ecbffff)],
    &[(2, 0xd73a49ff, 0xf97583ff), (3, 0x24292eff, 0xe1e4e8ff), (9, 0x6f42c1ff, 0xb392f0ff), (11, 0x24292eff, 0xe1e4e8ff), (12, 0x6f42c1ff, 0xb392f0ff), (19, 0x24292eff, 0xe1e4e8ff), (20, 0xd73a49ff, 0xf97583ff), (21, 0x24292eff, 0xe1e4e8ff), (22, 0xd73a49ff, 0xf97583ff), (25, 0x6f42c1ff, 0xb392f0ff), (33, 0x24292eff, 0xe1e4e8ff), (34, 0xd73a49ff, 0xf97583ff), (35, 0x24292eff, 0xe1e4e8ff), (36, 0xd73a49ff, 0xf97583ff), (37, 0x24292eff, 0xe1e4e8ff), (38, 0xd73a49ff, 0xf97583ff), (39, 0x24292eff, 0xe1e4e8ff), (40, 0x6f42c1ff, 0xb392f0ff), (42, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0xd73a49ff, 0xf97583ff), (3, 0x24292eff, 0xe1e4e8ff), (9, 0x6f42c1ff, 0xb392f0ff), (11, 0x24292eff, 0xe1e4e8ff), (12, 0x6f42c1ff, 0xb392f0ff), (19, 0x24292eff, 0xe1e4e8ff), (20, 0xd73a49ff, 0xf97583ff), (21, 0x24292eff, 0xe1e4e8ff), (22, 0xd73a49ff, 0xf97583ff), (25, 0x6f42c1ff, 0xb392f0ff), (33, 0x24292eff, 0xe1e4e8ff), (34, 0xd73a49ff, 0xf97583ff), (35, 0x24292eff, 0xe1e4e8ff), (36, 0xd73a49ff, 0xf97583ff), (37, 0x24292eff, 0xe1e4e8ff), (38, 0xd73a49ff, 0xf97583ff), (39, 0x24292eff, 0xe1e4e8ff), (40, 0x6f42c1ff, 0xb392f0ff), (43, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0xd73a49ff, 0xf97583ff), (3, 0x24292eff, 0xe1e4e8ff), (9, 0x6f42c1ff, 0xb392f0ff), (11, 0x24292eff, 0xe1e4e8ff), (12, 0x6f42c1ff, 0xb392f0ff), (19, 0x24292eff, 0xe1e4e8ff), (20, 0xd73a49ff, 0xf97583ff), (21, 0x24292eff, 0xe1e4e8ff), (22, 0xd73a49ff, 0xf97583ff), (25, 0x6f42c1ff, 0xb392f0ff), (33, 0x24292eff, 0xe1e4e8ff), (34, 0xd73a49ff, 0xf97583ff), (35, 0x24292eff, 0xe1e4e8ff), (36, 0xd73a49ff, 0xf97583ff), (37, 0x24292eff, 0xe1e4e8ff), (38, 0xd73a49ff, 0xf97583ff), (39, 0x24292eff, 0xe1e4e8ff), (40, 0x6f42c1ff, 0xb392f0ff), (41, 0x24292eff, 0xe1e4e8ff), (44, 0x6f42c1ff, 0xb392f0ff), (47, 0x24292eff, 0xe1e4e8ff), (49, 0xd73a49ff, 0xf97583ff), (50, 0x24292eff, 0xe1e4e8ff), (53, 0x6f42c1ff, 0xb392f0ff), (54, 0x24292eff, 0xe1e4e8ff), (55, 0xd73a49ff, 0xf97583ff), (56, 0x24292eff, 0xe1e4e8ff), (57, 0x6f42c1ff, 0xb392f0ff), (59, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0xd73a49ff, 0xf97583ff), (3, 0x24292eff, 0xe1e4e8ff), (9, 0x6f42c1ff, 0xb392f0ff), (11, 0x24292eff, 0xe1e4e8ff), (12, 0x6f42c1ff, 0xb392f0ff), (19, 0x24292eff, 0xe1e4e8ff), (20, 0xd73a49ff, 0xf97583ff), (21, 0x24292eff, 0xe1e4e8ff), (22, 0xd73a49ff, 0xf97583ff), (25, 0x6f42c1ff, 0xb392f0ff), (33, 0x24292eff, 0xe1e4e8ff), (34, 0xd73a49ff, 0xf97583ff), (35, 0x24292eff, 0xe1e4e8ff), (36, 0xd73a49ff, 0xf97583ff), (37, 0x24292eff, 0xe1e4e8ff), (38, 0xd73a49ff, 0xf97583ff), (39, 0x24292eff, 0xe1e4e8ff), (40, 0x6f42c1ff, 0xb392f0ff), (41, 0x24292eff, 0xe1e4e8ff), (44, 0x6f42c1ff, 0xb392f0ff), (47, 0x24292eff, 0xe1e4e8ff), (49, 0xd73a49ff, 0xf97583ff), (50, 0x24292eff, 0xe1e4e8ff), (53, 0x6f42c1ff, 0xb392f0ff), (54, 0x24292eff, 0xe1e4e8ff), (55, 0xd73a49ff, 0xf97583ff), (56, 0x24292eff, 0xe1e4e8ff), (57, 0x6f42c1ff, 0xb392f0ff), (60, 0x24292eff, 0xe1e4e8ff)],
    &[(10, 0x24292eff, 0xe1e4e8ff), (11, 0xd73a49ff, 0xf97583ff), (12, 0x24292eff, 0xe1e4e8ff)],
    &[(10, 0x24292eff, 0xe1e4e8ff), (11, 0xd73a49ff, 0xf97583ff), (13, 0x24292eff, 0xe1e4e8ff)],
    &[(10, 0x24292eff, 0xe1e4e8ff), (11, 0xd73a49ff, 0xf97583ff), (14, 0x24292eff, 0xe1e4e8ff)],
    &[(10, 0x24292eff, 0xe1e4e8ff), (11, 0xd73a49ff, 0xf97583ff), (15, 0x24292eff, 0xe1e4e8ff)],
    &[(10, 0x24292eff, 0xe1e4e8ff), (11, 0xd73a49ff, 0xf97583ff), (15, 0x6f42c1ff, 0xb392f0ff), (17, 0x24292eff, 0xe1e4e8ff), (18, 0xd73a49ff, 0xf97583ff), (19, 0x24292eff, 0xe1e4e8ff)],
    &[(10, 0x24292eff, 0xe1e4e8ff), (11, 0xd73a49ff, 0xf97583ff), (15, 0x6f42c1ff, 0xb392f0ff), (17, 0x24292eff, 0xe1e4e8ff), (18, 0xd73a49ff, 0xf97583ff), (20, 0x24292eff, 0xe1e4e8ff)],
    &[(10, 0x24292eff, 0xe1e4e8ff), (11, 0xd73a49ff, 0xf97583ff), (15, 0x6f42c1ff, 0xb392f0ff), (17, 0x24292eff, 0xe1e4e8ff), (18, 0xd73a49ff, 0xf97583ff), (21, 0x24292eff, 0xe1e4e8ff)],
    &[(10, 0x24292eff, 0xe1e4e8ff), (11, 0xd73a49ff, 0xf97583ff), (15, 0x6f42c1ff, 0xb392f0ff), (17, 0x24292eff, 0xe1e4e8ff), (18, 0xd73a49ff, 0xf97583ff), (22, 0x24292eff, 0xe1e4e8ff)],
    &[(10, 0x24292eff, 0xe1e4e8ff), (11, 0xd73a49ff, 0xf97583ff), (15, 0x6f42c1ff, 0xb392f0ff), (17, 0x24292eff, 0xe1e4e8ff), (18, 0xd73a49ff, 0xf97583ff), (23, 0x24292eff, 0xe1e4e8ff)],
    &[(10, 0x24292eff, 0xe1e4e8ff), (11, 0xd73a49ff, 0xf97583ff), (15, 0x6f42c1ff, 0xb392f0ff), (17, 0x24292eff, 0xe1e4e8ff), (18, 0xd73a49ff, 0xf97583ff), (24, 0x24292eff, 0xe1e4e8ff)],
    &[(8, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff)],
    &[(8, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (11, 0x24292eff, 0xe1e4e8ff)],
    &[(8, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (12, 0x24292eff, 0xe1e4e8ff)],
    &[(8, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (13, 0x24292eff, 0xe1e4e8ff)],
    &[(8, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (14, 0x24292eff, 0xe1e4e8ff)],
    &[(8, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (15, 0x24292eff, 0xe1e4e8ff)],
    &[(8, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (15, 0x6f42c1ff, 0xb392f0ff), (16, 0x24292eff, 0xe1e4e8ff), (17, 0xd73a49ff, 0xf97583ff), (22, 0x24292eff, 0xe1e4e8ff), (23, 0xd73a49ff, 0xf97583ff), (29, 0x24292eff, 0xe1e4e8ff), (30, 0xd73a49ff, 0xf97583ff), (31, 0x24292eff, 0xe1e4e8ff)],
    &[(8, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (15, 0x6f42c1ff, 0xb392f0ff), (16, 0x24292eff, 0xe1e4e8ff), (17, 0xd73a49ff, 0xf97583ff), (22, 0x24292eff, 0xe1e4e8ff), (23, 0xd73a49ff, 0xf97583ff), (29, 0x24292eff, 0xe1e4e8ff), (30, 0xd73a49ff, 0xf97583ff), (32, 0x24292eff, 0xe1e4e8ff)],
    &[(8, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (15, 0x6f42c1ff, 0xb392f0ff), (16, 0x24292eff, 0xe1e4e8ff), (17, 0xd73a49ff, 0xf97583ff), (22, 0x24292eff, 0xe1e4e8ff), (23, 0xd73a49ff, 0xf97583ff), (29, 0x24292eff, 0xe1e4e8ff), (30, 0xd73a49ff, 0xf97583ff), (33, 0x24292eff, 0xe1e4e8ff)],
    &[(8, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (15, 0x6f42c1ff, 0xb392f0ff), (16, 0x24292eff, 0xe1e4e8ff), (17, 0xd73a49ff, 0xf97583ff), (22, 0x24292eff, 0xe1e4e8ff), (23, 0xd73a49ff, 0xf97583ff), (29, 0x24292eff, 0xe1e4e8ff), (30, 0xd73a49ff, 0xf97583ff), (34, 0x24292eff, 0xe1e4e8ff)],
    &[(8, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (15, 0x6f42c1ff, 0xb392f0ff), (16, 0x24292eff, 0xe1e4e8ff), (17, 0xd73a49ff, 0xf97583ff), (22, 0x24292eff, 0xe1e4e8ff), (23, 0xd73a49ff, 0xf97583ff), (29, 0x24292eff, 0xe1e4e8ff), (30, 0xd73a49ff, 0xf97583ff), (35, 0x24292eff, 0xe1e4e8ff)],
    &[(8, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (15, 0x6f42c1ff, 0xb392f0ff), (16, 0x24292eff, 0xe1e4e8ff), (17, 0xd73a49ff, 0xf97583ff), (22, 0x24292eff, 0xe1e4e8ff), (23, 0xd73a49ff, 0xf97583ff), (29, 0x24292eff, 0xe1e4e8ff), (30, 0xd73a49ff, 0xf97583ff), (36, 0x24292eff, 0xe1e4e8ff)],
    &[(8, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (15, 0x6f42c1ff, 0xb392f0ff), (16, 0x24292eff, 0xe1e4e8ff), (17, 0xd73a49ff, 0xf97583ff), (22, 0x24292eff, 0xe1e4e8ff), (23, 0xd73a49ff, 0xf97583ff), (29, 0x24292eff, 0xe1e4e8ff), (30, 0xd73a49ff, 0xf97583ff), (37, 0x24292eff, 0xe1e4e8ff)],
    &[(8, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (15, 0x6f42c1ff, 0xb392f0ff), (16, 0x24292eff, 0xe1e4e8ff), (17, 0xd73a49ff, 0xf97583ff), (22, 0x24292eff, 0xe1e4e8ff), (23, 0xd73a49ff, 0xf97583ff), (29, 0x24292eff, 0xe1e4e8ff), (30, 0xd73a49ff, 0xf97583ff), (38, 0x24292eff, 0xe1e4e8ff)],
    &[(8, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (15, 0x6f42c1ff, 0xb392f0ff), (16, 0x24292eff, 0xe1e4e8ff), (17, 0xd73a49ff, 0xf97583ff), (22, 0x24292eff, 0xe1e4e8ff), (23, 0xd73a49ff, 0xf97583ff), (25, 0x24292eff, 0xe1e4e8ff)],
    &[(8, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (15, 0x6f42c1ff, 0xb392f0ff), (16, 0x24292eff, 0xe1e4e8ff), (17, 0xd73a49ff, 0xf97583ff), (22, 0x24292eff, 0xe1e4e8ff), (23, 0xd73a49ff, 0xf97583ff), (26, 0x24292eff, 0xe1e4e8ff)],
    &[(8, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (15, 0x6f42c1ff, 0xb392f0ff), (16, 0x24292eff, 0xe1e4e8ff), (17, 0xd73a49ff, 0xf97583ff), (22, 0x24292eff, 0xe1e4e8ff), (23, 0xd73a49ff, 0xf97583ff), (27, 0x24292eff, 0xe1e4e8ff)],
    &[(8, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (15, 0x6f42c1ff, 0xb392f0ff), (16, 0x24292eff, 0xe1e4e8ff), (17, 0xd73a49ff, 0xf97583ff), (22, 0x24292eff, 0xe1e4e8ff), (23, 0xd73a49ff, 0xf97583ff), (28, 0x24292eff, 0xe1e4e8ff)],
    &[(8, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (15, 0x6f42c1ff, 0xb392f0ff), (16, 0x24292eff, 0xe1e4e8ff), (17, 0xd73a49ff, 0xf97583ff), (22, 0x24292eff, 0xe1e4e8ff), (23, 0xd73a49ff, 0xf97583ff), (24, 0x24292eff, 0xe1e4e8ff), (29, 0xd73a49ff, 0xf97583ff)],
    &[(8, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (15, 0x6f42c1ff, 0xb392f0ff), (16, 0x24292eff, 0xe1e4e8ff), (17, 0xd73a49ff, 0xf97583ff), (22, 0x24292eff, 0xe1e4e8ff), (23, 0xd73a49ff, 0xf97583ff), (30, 0x24292eff, 0xe1e4e8ff)],
    &[(8, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (15, 0x6f42c1ff, 0xb392f0ff), (16, 0x24292eff, 0xe1e4e8ff), (17, 0xd73a49ff, 0xf97583ff), (22, 0x24292eff, 0xe1e4e8ff), (23, 0xd73a49ff, 0xf97583ff), (31, 0x24292eff, 0xe1e4e8ff)],
    &[(8, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (16, 0x24292eff, 0xe1e4e8ff)],
    &[(10, 0x24292eff, 0xe1e4e8ff), (11, 0xd73a49ff, 0xf97583ff), (15, 0x6f42c1ff, 0xb392f0ff), (17, 0x24292eff, 0xe1e4e8ff), (18, 0xd73a49ff, 0xf97583ff), (24, 0x6f42c1ff, 0xb392f0ff), (26, 0x24292eff, 0xe1e4e8ff)],
    &[(8, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (15, 0x6f42c1ff, 0xb392f0ff), (16, 0x24292eff, 0xe1e4e8ff), (17, 0xd73a49ff, 0xf97583ff), (22, 0x24292eff, 0xe1e4e8ff), (23, 0xd73a49ff, 0xf97583ff), (29, 0x24292eff, 0xe1e4e8ff), (30, 0xd73a49ff, 0xf97583ff), (38, 0x6f42c1ff, 0xb392f0ff), (46, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (22, 0x6f42c1ff, 0xb392f0ff), (24, 0x24292eff, 0xe1e4e8ff), (25, 0xd73a49ff, 0xf97583ff), (33, 0x6f42c1ff, 0xb392f0ff), (34, 0x24292eff, 0xe1e4e8ff), (35, 0xd73a49ff, 0xf97583ff), (40, 0x24292eff, 0xe1e4e8ff), (41, 0xd73a49ff, 0xf97583ff), (42, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (22, 0x6f42c1ff, 0xb392f0ff), (24, 0x24292eff, 0xe1e4e8ff), (25, 0xd73a49ff, 0xf97583ff), (33, 0x6f42c1ff, 0xb392f0ff), (34, 0x24292eff, 0xe1e4e8ff), (35, 0xd73a49ff, 0xf97583ff), (40, 0x24292eff, 0xe1e4e8ff), (41, 0xd73a49ff, 0xf97583ff), (43, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (22, 0x6f42c1ff, 0xb392f0ff), (24, 0x24292eff, 0xe1e4e8ff), (25, 0xd73a49ff, 0xf97583ff), (33, 0x6f42c1ff, 0xb392f0ff), (34, 0x24292eff, 0xe1e4e8ff), (35, 0xd73a49ff, 0xf97583ff), (40, 0x24292eff, 0xe1e4e8ff), (41, 0xd73a49ff, 0xf97583ff), (44, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (22, 0x6f42c1ff, 0xb392f0ff), (24, 0x24292eff, 0xe1e4e8ff), (25, 0xd73a49ff, 0xf97583ff), (33, 0x6f42c1ff, 0xb392f0ff), (34, 0x24292eff, 0xe1e4e8ff), (35, 0xd73a49ff, 0xf97583ff), (40, 0x24292eff, 0xe1e4e8ff), (41, 0xd73a49ff, 0xf97583ff), (45, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (22, 0x6f42c1ff, 0xb392f0ff), (24, 0x24292eff, 0xe1e4e8ff), (25, 0xd73a49ff, 0xf97583ff), (33, 0x6f42c1ff, 0xb392f0ff), (34, 0x24292eff, 0xe1e4e8ff), (35, 0xd73a49ff, 0xf97583ff), (40, 0x24292eff, 0xe1e4e8ff), (41, 0xd73a49ff, 0xf97583ff), (46, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (22, 0x6f42c1ff, 0xb392f0ff), (24, 0x24292eff, 0xe1e4e8ff), (25, 0xd73a49ff, 0xf97583ff), (33, 0x6f42c1ff, 0xb392f0ff), (34, 0x24292eff, 0xe1e4e8ff), (35, 0xd73a49ff, 0xf97583ff), (40, 0x24292eff, 0xe1e4e8ff), (41, 0xd73a49ff, 0xf97583ff), (47, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (22, 0x6f42c1ff, 0xb392f0ff), (24, 0x24292eff, 0xe1e4e8ff), (25, 0xd73a49ff, 0xf97583ff), (33, 0x6f42c1ff, 0xb392f0ff), (34, 0x24292eff, 0xe1e4e8ff), (35, 0xd73a49ff, 0xf97583ff), (40, 0x24292eff, 0xe1e4e8ff), (41, 0xd73a49ff, 0xf97583ff), (48, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (22, 0x6f42c1ff, 0xb392f0ff), (24, 0x24292eff, 0xe1e4e8ff), (25, 0xd73a49ff, 0xf97583ff), (33, 0x6f42c1ff, 0xb392f0ff), (34, 0x24292eff, 0xe1e4e8ff), (35, 0xd73a49ff, 0xf97583ff), (40, 0x24292eff, 0xe1e4e8ff), (41, 0xd73a49ff, 0xf97583ff), (49, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (22, 0x6f42c1ff, 0xb392f0ff), (24, 0x24292eff, 0xe1e4e8ff), (25, 0xd73a49ff, 0xf97583ff), (33, 0x6f42c1ff, 0xb392f0ff), (34, 0x24292eff, 0xe1e4e8ff), (35, 0xd73a49ff, 0xf97583ff), (40, 0x24292eff, 0xe1e4e8ff), (41, 0xd73a49ff, 0xf97583ff), (50, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (22, 0x6f42c1ff, 0xb392f0ff), (24, 0x24292eff, 0xe1e4e8ff), (25, 0xd73a49ff, 0xf97583ff), (33, 0x6f42c1ff, 0xb392f0ff), (34, 0x24292eff, 0xe1e4e8ff), (35, 0xd73a49ff, 0xf97583ff), (40, 0x24292eff, 0xe1e4e8ff), (41, 0xd73a49ff, 0xf97583ff), (51, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (22, 0x6f42c1ff, 0xb392f0ff), (24, 0x24292eff, 0xe1e4e8ff), (25, 0xd73a49ff, 0xf97583ff), (33, 0x6f42c1ff, 0xb392f0ff), (34, 0x24292eff, 0xe1e4e8ff), (35, 0xd73a49ff, 0xf97583ff), (40, 0x24292eff, 0xe1e4e8ff), (41, 0xd73a49ff, 0xf97583ff), (52, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (22, 0x6f42c1ff, 0xb392f0ff), (24, 0x24292eff, 0xe1e4e8ff), (25, 0xd73a49ff, 0xf97583ff), (33, 0x6f42c1ff, 0xb392f0ff), (34, 0x24292eff, 0xe1e4e8ff), (35, 0xd73a49ff, 0xf97583ff), (40, 0x24292eff, 0xe1e4e8ff), (41, 0xd73a49ff, 0xf97583ff), (53, 0x24292eff, 0xe1e4e8ff)],
    &[(1, 0xb31d28ff, 0xfdaeb7ff)],
    &[(2, 0xb31d28ff, 0xfdaeb7ff)],
    &[(3, 0xb31d28ff, 0xfdaeb7ff)],
    &[(4, 0xb31d28ff, 0xfdaeb7ff)],
    &[(5, 0xb31d28ff, 0xfdaeb7ff)],
    &[(6, 0xb31d28ff, 0xfdaeb7ff)],
    &[(7, 0xb31d28ff, 0xfdaeb7ff)],
    &[(8, 0xb31d28ff, 0xfdaeb7ff)],
    &[(8, 0xb31d28ff, 0xfdaeb7ff), (9, 0x24292eff, 0xe1e4e8ff)],
];
    #[rustfmt::skip]
static FIXTURES: &[Fixture] = &[
    Fixture { source: STREAMING_0, rows: &[5, 64, 65, 2], tails: &[(1, 2), (4, 13), (5, 14), (6, 15), (7, 16), (8, 17), (9, 18), (10, 19), (20, 20), (21, 21), (33, 22), (34, 23), (41, 24), (42, 25), (43, 26), (52, 1), (53, 27), (70, 28), (71, 29), (72, 30), (73, 31), (77, 32), (78, 33), (79, 34), (80, 35), (81, 36), (82, 37), (83, 38), (84, 39), (85, 40), (86, 41), (87, 42), (88, 43), (103, 44), (104, 45), (105, 46), (106, 47), (107, 48), (108, 49), (109, 50), (110, 51), (111, 52), (112, 53), (113, 54), (114, 55), (118, 56), (119, 57), (120, 58), (121, 59), (122, 60), (123, 61), (124, 62), (125, 63)] },
    Fixture { source: STREAMING_1, rows: &[67], tails: &[(12, 66)] },
    Fixture { source: STREAMING_2, rows: &[7, 109, 110, 10, 4, 2], tails: &[(2, 2), (5, 13), (6, 14), (7, 15), (8, 16), (9, 68), (10, 18), (12, 69), (24, 70), (25, 71), (40, 72), (43, 73), (44, 74), (60, 75), (61, 76), (79, 77), (80, 78), (81, 79), (82, 80), (86, 81), (87, 82), (88, 83), (89, 84), (90, 85), (91, 86), (105, 87), (106, 88), (107, 89), (108, 90), (109, 91), (110, 92), (126, 93), (127, 94), (128, 95), (129, 96), (130, 97), (131, 98), (132, 99), (133, 100), (153, 87), (154, 88), (155, 89), (156, 90), (157, 91), (158, 92), (168, 101), (169, 102), (170, 103), (171, 104), (172, 105), (173, 106), (174, 107), (201, 87), (202, 88), (203, 89), (204, 90), (205, 91), (206, 92), (207, 108)] },
    Fixture { source: STREAMING_3, rows: &[5, 6, 2, 0, 7, 8, 3, 9, 10, 4, 2], tails: &[(1, 2), (4, 13), (5, 14), (6, 15), (7, 16), (8, 17), (9, 18), (10, 19), (20, 20), (21, 21), (33, 22), (34, 23), (41, 24), (42, 25), (43, 26), (58, 44), (59, 45), (60, 46), (61, 47), (62, 48), (63, 49), (64, 50), (65, 51), (66, 52), (67, 53), (68, 54), (69, 55), (73, 56), (74, 57), (75, 58), (76, 59), (77, 60), (78, 61), (79, 62), (80, 63), (89, 111), (90, 112), (91, 113), (92, 114), (93, 115), (94, 116), (95, 117), (96, 118), (97, 119), (98, 120), (99, 121), (100, 122), (108, 2), (111, 13), (112, 14), (113, 15), (114, 16), (115, 68), (116, 18), (118, 69), (130, 70), (131, 71), (146, 72), (149, 73), (150, 74), (166, 75), (167, 76), (193, 87), (194, 88), (195, 89), (196, 90), (209, 87), (210, 88), (211, 89), (212, 90), (213, 91), (214, 92), (227, 87), (228, 88), (229, 89), (230, 90), (231, 91), (232, 92), (242, 101), (243, 102), (244, 103), (245, 104), (246, 105), (247, 106), (248, 107), (274, 87), (275, 88), (276, 89), (277, 90), (278, 91), (279, 92), (280, 108)] },
    Fixture { source: STREAMING_4, rows: &[11, 12], tails: &[(1, 123), (2, 124), (3, 125), (4, 126), (7, 123), (8, 124), (9, 125), (10, 126), (11, 127), (12, 128), (13, 129), (14, 130), (15, 131)] },
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

    pub const STREAMS: &[(&str, &str, &str)] = &[
        ("src/matcher.rs", "rust", STREAMING_0),
        ("Terminal", "sh", STREAMING_1),
        ("src/search.rs", "rust", STREAMING_2),
        ("src/search.rs", "rust", STREAMING_3),
        (".cargo/config.toml", "toml", STREAMING_4),
    ];
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
            let line = if source == STREAMING_2 {
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

const STREAMING_0: &str = r##"fn matches(query: &str, title: &str) -> bool {
    let query = query.trim().to_lowercase();
    title.to_lowercase().contains(&query)
}"##;

const STREAMING_1: &str = r##"cargo test --lib search"##;

const STREAMING_2: &str = r##" fn search<'a>(query: &str, titles: &[&'a str]) -> Vec<&'a str> {
     titles.iter().copied()
-        .filter(|title| title.contains(query))
+        .filter(|title| matches(query, title))
         .collect()
 }"##;

const STREAMING_3: &str = r##"fn matches(query: &str, title: &str) -> bool {
    title.to_lowercase().contains(&query.to_lowercase())
}

fn search<'a>(query: &str, titles: &[&'a str]) -> Vec<&'a str> {
    titles
        .iter()
        .copied()
        .filter(|title| matches(query, title))
        .collect()
}"##;

const STREAMING_4: &str = r##"[env]
RUST_LOG = "notebook=debug""##;
```
