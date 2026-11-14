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
    &[(13, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (8, 0xd73a49ff, 0xf97583ff), (16, 0x24292eff, 0xe1e4e8ff), (41, 0x6f42c1ff, 0xb392f0ff), (43, 0x24292eff, 0xe1e4e8ff)],
    &[(1, 0x24292eff, 0xe1e4e8ff)],
    &[(3, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (9, 0x24292eff, 0xe1e4e8ff), (14, 0x005cc5ff, 0x79b8ffff), (16, 0x24292eff, 0xe1e4e8ff), (24, 0x005cc5ff, 0x79b8ffff), (26, 0x24292eff, 0xe1e4e8ff), (27, 0xd73a49ff, 0xf97583ff), (34, 0x24292eff, 0xe1e4e8ff), (42, 0x6f42c1ff, 0xb392f0ff), (43, 0x24292eff, 0xe1e4e8ff), (45, 0x032f62ff, 0x9ecbffff), (46, 0x24292eff, 0xe1e4e8ff)],
    &[(12, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff)],
    &[(10, 0x24292eff, 0xe1e4e8ff)],
    &[(7, 0x24292eff, 0xe1e4e8ff)],
    &[(18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (31, 0x032f62ff, 0x9ecbffff)],
    &[(2, 0x24292eff, 0xe1e4e8ff)],
    &[(5, 0x24292eff, 0xe1e4e8ff)],
    &[(6, 0xd73a49ff, 0xf97583ff), (8, 0x24292eff, 0xe1e4e8ff)],
    &[(6, 0xd73a49ff, 0xf97583ff), (9, 0x24292eff, 0xe1e4e8ff)],
    &[(6, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff)],
    &[(6, 0xd73a49ff, 0xf97583ff), (11, 0x24292eff, 0xe1e4e8ff)],
    &[(6, 0xd73a49ff, 0xf97583ff), (12, 0x24292eff, 0xe1e4e8ff)],
    &[(6, 0xd73a49ff, 0xf97583ff), (13, 0x24292eff, 0xe1e4e8ff)],
    &[(6, 0xd73a49ff, 0xf97583ff), (14, 0x24292eff, 0xe1e4e8ff)],
    &[(6, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (9, 0x24292eff, 0xe1e4e8ff), (18, 0x005cc5ff, 0x79b8ffff), (20, 0x24292eff, 0xe1e4e8ff), (32, 0x005cc5ff, 0x79b8ffff), (34, 0x24292eff, 0xe1e4e8ff), (35, 0xd73a49ff, 0xf97583ff), (36, 0x24292eff, 0xe1e4e8ff), (37, 0x005cc5ff, 0x79b8ffff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (9, 0x24292eff, 0xe1e4e8ff), (18, 0x005cc5ff, 0x79b8ffff), (20, 0x24292eff, 0xe1e4e8ff), (32, 0x005cc5ff, 0x79b8ffff), (34, 0x24292eff, 0xe1e4e8ff), (35, 0xd73a49ff, 0xf97583ff), (43, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (9, 0x24292eff, 0xe1e4e8ff), (18, 0x005cc5ff, 0x79b8ffff), (20, 0x24292eff, 0xe1e4e8ff), (32, 0x005cc5ff, 0x79b8ffff), (34, 0x24292eff, 0xe1e4e8ff), (35, 0xd73a49ff, 0xf97583ff), (44, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (9, 0x24292eff, 0xe1e4e8ff), (18, 0x005cc5ff, 0x79b8ffff), (20, 0x24292eff, 0xe1e4e8ff), (32, 0x005cc5ff, 0x79b8ffff), (34, 0x24292eff, 0xe1e4e8ff), (35, 0xd73a49ff, 0xf97583ff), (45, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (9, 0x24292eff, 0xe1e4e8ff), (18, 0x005cc5ff, 0x79b8ffff), (20, 0x24292eff, 0xe1e4e8ff), (32, 0x005cc5ff, 0x79b8ffff), (34, 0x24292eff, 0xe1e4e8ff), (35, 0xd73a49ff, 0xf97583ff), (46, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (9, 0x24292eff, 0xe1e4e8ff), (18, 0x005cc5ff, 0x79b8ffff), (20, 0x24292eff, 0xe1e4e8ff), (32, 0x005cc5ff, 0x79b8ffff), (34, 0x24292eff, 0xe1e4e8ff), (35, 0xd73a49ff, 0xf97583ff), (47, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (9, 0x24292eff, 0xe1e4e8ff), (18, 0x005cc5ff, 0x79b8ffff), (20, 0x24292eff, 0xe1e4e8ff), (32, 0x005cc5ff, 0x79b8ffff), (34, 0x24292eff, 0xe1e4e8ff), (35, 0xd73a49ff, 0xf97583ff), (48, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (9, 0x24292eff, 0xe1e4e8ff), (18, 0x005cc5ff, 0x79b8ffff), (20, 0x24292eff, 0xe1e4e8ff), (32, 0x005cc5ff, 0x79b8ffff), (34, 0x24292eff, 0xe1e4e8ff), (35, 0xd73a49ff, 0xf97583ff), (49, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (9, 0x24292eff, 0xe1e4e8ff), (18, 0x005cc5ff, 0x79b8ffff), (20, 0x24292eff, 0xe1e4e8ff), (32, 0x005cc5ff, 0x79b8ffff), (34, 0x24292eff, 0xe1e4e8ff), (35, 0xd73a49ff, 0xf97583ff), (50, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (3, 0x005cc5ff, 0x79b8ffff)],
    &[(11, 0x24292eff, 0xe1e4e8ff)],
    &[(14, 0x24292eff, 0xe1e4e8ff)],
    &[(15, 0x24292eff, 0xe1e4e8ff)],
    &[(16, 0x24292eff, 0xe1e4e8ff)],
    &[(17, 0x24292eff, 0xe1e4e8ff)],
    &[(8, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff), (17, 0x005cc5ff, 0x79b8ffff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (21, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff), (17, 0x005cc5ff, 0x79b8ffff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (22, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff), (17, 0x005cc5ff, 0x79b8ffff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (23, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff), (17, 0x005cc5ff, 0x79b8ffff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (24, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff), (17, 0x005cc5ff, 0x79b8ffff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (25, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff), (17, 0x005cc5ff, 0x79b8ffff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (26, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff), (17, 0x005cc5ff, 0x79b8ffff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (27, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff), (17, 0x005cc5ff, 0x79b8ffff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (28, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff), (17, 0x005cc5ff, 0x79b8ffff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (29, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff), (17, 0x005cc5ff, 0x79b8ffff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (30, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff), (17, 0x005cc5ff, 0x79b8ffff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (20, 0x24292eff, 0xe1e4e8ff), (30, 0x6f42c1ff, 0xb392f0ff), (34, 0x24292eff, 0xe1e4e8ff), (36, 0xd73a49ff, 0xf97583ff), (38, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff), (17, 0x005cc5ff, 0x79b8ffff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (20, 0x24292eff, 0xe1e4e8ff), (30, 0x6f42c1ff, 0xb392f0ff), (34, 0x24292eff, 0xe1e4e8ff), (36, 0xd73a49ff, 0xf97583ff), (39, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff), (17, 0x005cc5ff, 0x79b8ffff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (20, 0x24292eff, 0xe1e4e8ff), (30, 0x6f42c1ff, 0xb392f0ff), (34, 0x24292eff, 0xe1e4e8ff), (36, 0xd73a49ff, 0xf97583ff), (40, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff), (17, 0x005cc5ff, 0x79b8ffff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (20, 0x24292eff, 0xe1e4e8ff), (30, 0x6f42c1ff, 0xb392f0ff), (34, 0x24292eff, 0xe1e4e8ff), (36, 0xd73a49ff, 0xf97583ff), (41, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff), (17, 0x005cc5ff, 0x79b8ffff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (20, 0x24292eff, 0xe1e4e8ff), (30, 0x6f42c1ff, 0xb392f0ff), (34, 0x24292eff, 0xe1e4e8ff), (36, 0xd73a49ff, 0xf97583ff), (42, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff), (17, 0x005cc5ff, 0x79b8ffff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (20, 0x24292eff, 0xe1e4e8ff), (30, 0x6f42c1ff, 0xb392f0ff), (34, 0x24292eff, 0xe1e4e8ff), (36, 0xd73a49ff, 0xf97583ff), (43, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff), (17, 0x005cc5ff, 0x79b8ffff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (20, 0x24292eff, 0xe1e4e8ff), (30, 0x6f42c1ff, 0xb392f0ff), (34, 0x24292eff, 0xe1e4e8ff), (36, 0xd73a49ff, 0xf97583ff), (44, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff), (17, 0x005cc5ff, 0x79b8ffff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (20, 0x24292eff, 0xe1e4e8ff), (30, 0x6f42c1ff, 0xb392f0ff), (34, 0x24292eff, 0xe1e4e8ff), (36, 0xd73a49ff, 0xf97583ff), (45, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff), (17, 0x005cc5ff, 0x79b8ffff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (20, 0x24292eff, 0xe1e4e8ff), (30, 0x6f42c1ff, 0xb392f0ff), (34, 0x24292eff, 0xe1e4e8ff), (36, 0xd73a49ff, 0xf97583ff), (46, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff), (17, 0x005cc5ff, 0x79b8ffff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (20, 0x24292eff, 0xe1e4e8ff), (30, 0x6f42c1ff, 0xb392f0ff), (34, 0x24292eff, 0xe1e4e8ff), (36, 0xd73a49ff, 0xf97583ff), (47, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff), (17, 0x005cc5ff, 0x79b8ffff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (20, 0x24292eff, 0xe1e4e8ff), (30, 0x6f42c1ff, 0xb392f0ff), (34, 0x24292eff, 0xe1e4e8ff), (36, 0xd73a49ff, 0xf97583ff), (48, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff), (17, 0x005cc5ff, 0x79b8ffff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (20, 0x24292eff, 0xe1e4e8ff), (30, 0x6f42c1ff, 0xb392f0ff), (34, 0x24292eff, 0xe1e4e8ff), (36, 0xd73a49ff, 0xf97583ff), (49, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (14, 0x24292eff, 0xe1e4e8ff), (16, 0xd73a49ff, 0xf97583ff), (18, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (14, 0x24292eff, 0xe1e4e8ff), (16, 0xd73a49ff, 0xf97583ff), (19, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (14, 0x24292eff, 0xe1e4e8ff), (16, 0xd73a49ff, 0xf97583ff), (20, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (14, 0x24292eff, 0xe1e4e8ff), (16, 0xd73a49ff, 0xf97583ff), (21, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (14, 0x24292eff, 0xe1e4e8ff), (16, 0xd73a49ff, 0xf97583ff), (22, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (14, 0x24292eff, 0xe1e4e8ff), (16, 0xd73a49ff, 0xf97583ff), (23, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (14, 0x24292eff, 0xe1e4e8ff), (16, 0xd73a49ff, 0xf97583ff), (24, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (14, 0x24292eff, 0xe1e4e8ff), (16, 0xd73a49ff, 0xf97583ff), (25, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (14, 0x24292eff, 0xe1e4e8ff), (16, 0xd73a49ff, 0xf97583ff), (26, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (14, 0x24292eff, 0xe1e4e8ff), (16, 0xd73a49ff, 0xf97583ff), (27, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (14, 0x24292eff, 0xe1e4e8ff), (16, 0xd73a49ff, 0xf97583ff), (28, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (14, 0x24292eff, 0xe1e4e8ff), (16, 0xd73a49ff, 0xf97583ff), (29, 0x24292eff, 0xe1e4e8ff)],
    &[(6, 0xd73a49ff, 0xf97583ff), (7, 0x24292eff, 0xe1e4e8ff), (15, 0xd73a49ff, 0xf97583ff), (16, 0x24292eff, 0xe1e4e8ff), (27, 0x6f42c1ff, 0xb392f0ff), (28, 0x24292eff, 0xe1e4e8ff), (29, 0x6f42c1ff, 0xb392f0ff), (31, 0x24292eff, 0xe1e4e8ff), (36, 0xe36209ff, 0xffab70ff), (37, 0xd73a49ff, 0xf97583ff), (38, 0x24292eff, 0xe1e4e8ff), (39, 0x6f42c1ff, 0xb392f0ff), (41, 0x24292eff, 0xe1e4e8ff), (46, 0xe36209ff, 0xffab70ff), (47, 0x24292eff, 0xe1e4e8ff), (48, 0xd73a49ff, 0xf97583ff), (49, 0x24292eff, 0xe1e4e8ff), (52, 0x005cc5ff, 0x79b8ffff), (55, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (9, 0x24292eff, 0xe1e4e8ff), (18, 0x005cc5ff, 0x79b8ffff), (20, 0x24292eff, 0xe1e4e8ff), (32, 0x005cc5ff, 0x79b8ffff), (34, 0x24292eff, 0xe1e4e8ff), (35, 0xd73a49ff, 0xf97583ff), (42, 0x24292eff, 0xe1e4e8ff), (50, 0x6f42c1ff, 0xb392f0ff), (57, 0x24292eff, 0xe1e4e8ff)],
    &[(8, 0x24292eff, 0xe1e4e8ff), (17, 0x6f42c1ff, 0xb392f0ff), (21, 0x24292eff, 0xe1e4e8ff), (23, 0xd73a49ff, 0xf97583ff), (25, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (9, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff), (17, 0x005cc5ff, 0x79b8ffff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (20, 0x24292eff, 0xe1e4e8ff), (30, 0x6f42c1ff, 0xb392f0ff), (34, 0x24292eff, 0xe1e4e8ff), (36, 0xd73a49ff, 0xf97583ff), (37, 0x24292eff, 0xe1e4e8ff), (49, 0x6f42c1ff, 0xb392f0ff), (64, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff), (10, 0xd73a49ff, 0xf97583ff), (14, 0x24292eff, 0xe1e4e8ff), (16, 0xd73a49ff, 0xf97583ff), (17, 0x24292eff, 0xe1e4e8ff), (29, 0x6f42c1ff, 0xb392f0ff), (38, 0x24292eff, 0xe1e4e8ff)],
    &[(20, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (8, 0xd73a49ff, 0xf97583ff), (18, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x6f42c1ff, 0xb392f0ff), (5, 0x24292eff, 0xe1e4e8ff), (8, 0x032f62ff, 0x9ecbffff), (9, 0x24292eff, 0xe1e4e8ff), (22, 0x032f62ff, 0x9ecbffff), (23, 0x24292eff, 0xe1e4e8ff), (26, 0x032f62ff, 0x9ecbffff), (27, 0x24292eff, 0xe1e4e8ff), (37, 0x032f62ff, 0x9ecbffff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (9, 0x24292eff, 0xe1e4e8ff), (14, 0x005cc5ff, 0x79b8ffff), (16, 0x24292eff, 0xe1e4e8ff), (24, 0x005cc5ff, 0x79b8ffff), (26, 0x24292eff, 0xe1e4e8ff), (27, 0xd73a49ff, 0xf97583ff), (28, 0x24292eff, 0xe1e4e8ff), (29, 0x005cc5ff, 0x79b8ffff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (9, 0x24292eff, 0xe1e4e8ff), (14, 0x005cc5ff, 0x79b8ffff), (16, 0x24292eff, 0xe1e4e8ff), (24, 0x005cc5ff, 0x79b8ffff), (26, 0x24292eff, 0xe1e4e8ff), (27, 0xd73a49ff, 0xf97583ff), (35, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (9, 0x24292eff, 0xe1e4e8ff), (14, 0x005cc5ff, 0x79b8ffff), (16, 0x24292eff, 0xe1e4e8ff), (24, 0x005cc5ff, 0x79b8ffff), (26, 0x24292eff, 0xe1e4e8ff), (27, 0xd73a49ff, 0xf97583ff), (36, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (9, 0x24292eff, 0xe1e4e8ff), (14, 0x005cc5ff, 0x79b8ffff), (16, 0x24292eff, 0xe1e4e8ff), (24, 0x005cc5ff, 0x79b8ffff), (26, 0x24292eff, 0xe1e4e8ff), (27, 0xd73a49ff, 0xf97583ff), (37, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (9, 0x24292eff, 0xe1e4e8ff), (14, 0x005cc5ff, 0x79b8ffff), (16, 0x24292eff, 0xe1e4e8ff), (24, 0x005cc5ff, 0x79b8ffff), (26, 0x24292eff, 0xe1e4e8ff), (27, 0xd73a49ff, 0xf97583ff), (38, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (9, 0x24292eff, 0xe1e4e8ff), (14, 0x005cc5ff, 0x79b8ffff), (16, 0x24292eff, 0xe1e4e8ff), (24, 0x005cc5ff, 0x79b8ffff), (26, 0x24292eff, 0xe1e4e8ff), (27, 0xd73a49ff, 0xf97583ff), (39, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (9, 0x24292eff, 0xe1e4e8ff), (14, 0x005cc5ff, 0x79b8ffff), (16, 0x24292eff, 0xe1e4e8ff), (24, 0x005cc5ff, 0x79b8ffff), (26, 0x24292eff, 0xe1e4e8ff), (27, 0xd73a49ff, 0xf97583ff), (40, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (9, 0x24292eff, 0xe1e4e8ff), (14, 0x005cc5ff, 0x79b8ffff), (16, 0x24292eff, 0xe1e4e8ff), (24, 0x005cc5ff, 0x79b8ffff), (26, 0x24292eff, 0xe1e4e8ff), (27, 0xd73a49ff, 0xf97583ff), (41, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (9, 0x24292eff, 0xe1e4e8ff), (14, 0x005cc5ff, 0x79b8ffff), (16, 0x24292eff, 0xe1e4e8ff), (24, 0x005cc5ff, 0x79b8ffff), (26, 0x24292eff, 0xe1e4e8ff), (27, 0xd73a49ff, 0xf97583ff), (42, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (8, 0x24292eff, 0xe1e4e8ff), (15, 0x005cc5ff, 0x79b8ffff), (16, 0x24292eff, 0xe1e4e8ff), (17, 0xd73a49ff, 0xf97583ff), (19, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (8, 0x24292eff, 0xe1e4e8ff), (15, 0x005cc5ff, 0x79b8ffff), (16, 0x24292eff, 0xe1e4e8ff), (17, 0xd73a49ff, 0xf97583ff), (20, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (8, 0x24292eff, 0xe1e4e8ff), (15, 0x005cc5ff, 0x79b8ffff), (16, 0x24292eff, 0xe1e4e8ff), (17, 0xd73a49ff, 0xf97583ff), (21, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (8, 0x24292eff, 0xe1e4e8ff), (15, 0x005cc5ff, 0x79b8ffff), (16, 0x24292eff, 0xe1e4e8ff), (17, 0xd73a49ff, 0xf97583ff), (22, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (8, 0x24292eff, 0xe1e4e8ff), (15, 0x005cc5ff, 0x79b8ffff), (16, 0x24292eff, 0xe1e4e8ff), (17, 0xd73a49ff, 0xf97583ff), (23, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (8, 0x24292eff, 0xe1e4e8ff), (15, 0x005cc5ff, 0x79b8ffff), (16, 0x24292eff, 0xe1e4e8ff), (17, 0xd73a49ff, 0xf97583ff), (24, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (8, 0x24292eff, 0xe1e4e8ff), (15, 0x005cc5ff, 0x79b8ffff), (16, 0x24292eff, 0xe1e4e8ff), (17, 0xd73a49ff, 0xf97583ff), (25, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (8, 0x24292eff, 0xe1e4e8ff), (15, 0x005cc5ff, 0x79b8ffff), (16, 0x24292eff, 0xe1e4e8ff), (17, 0xd73a49ff, 0xf97583ff), (26, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (8, 0x24292eff, 0xe1e4e8ff), (15, 0x005cc5ff, 0x79b8ffff), (16, 0x24292eff, 0xe1e4e8ff), (17, 0xd73a49ff, 0xf97583ff), (27, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (8, 0x24292eff, 0xe1e4e8ff), (17, 0x005cc5ff, 0x79b8ffff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (21, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (8, 0x24292eff, 0xe1e4e8ff), (17, 0x005cc5ff, 0x79b8ffff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (22, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (8, 0x24292eff, 0xe1e4e8ff), (17, 0x005cc5ff, 0x79b8ffff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (23, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (8, 0x24292eff, 0xe1e4e8ff), (17, 0x005cc5ff, 0x79b8ffff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (24, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (8, 0x24292eff, 0xe1e4e8ff), (17, 0x005cc5ff, 0x79b8ffff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (25, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (8, 0x24292eff, 0xe1e4e8ff), (17, 0x005cc5ff, 0x79b8ffff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (26, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (8, 0x24292eff, 0xe1e4e8ff), (17, 0x005cc5ff, 0x79b8ffff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (27, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (8, 0x24292eff, 0xe1e4e8ff), (17, 0x005cc5ff, 0x79b8ffff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (28, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (8, 0x24292eff, 0xe1e4e8ff), (17, 0x005cc5ff, 0x79b8ffff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (29, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (8, 0x24292eff, 0xe1e4e8ff), (17, 0x005cc5ff, 0x79b8ffff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (30, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (8, 0x24292eff, 0xe1e4e8ff), (17, 0x005cc5ff, 0x79b8ffff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (31, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (8, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff), (23, 0x005cc5ff, 0x79b8ffff), (24, 0x24292eff, 0xe1e4e8ff), (31, 0x6f42c1ff, 0xb392f0ff), (32, 0xd73a49ff, 0xf97583ff), (42, 0x24292eff, 0xe1e4e8ff), (43, 0xb31d28ff, 0xfdaeb7ff)],
    &[(6, 0xd73a49ff, 0xf97583ff), (7, 0x24292eff, 0xe1e4e8ff), (15, 0xd73a49ff, 0xf97583ff), (16, 0x24292eff, 0xe1e4e8ff), (22, 0x6f42c1ff, 0xb392f0ff), (26, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (8, 0x24292eff, 0xe1e4e8ff), (15, 0x005cc5ff, 0x79b8ffff), (16, 0x24292eff, 0xe1e4e8ff), (17, 0xd73a49ff, 0xf97583ff), (18, 0x24292eff, 0xe1e4e8ff), (27, 0x6f42c1ff, 0xb392f0ff), (34, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (8, 0x24292eff, 0xe1e4e8ff), (17, 0x005cc5ff, 0x79b8ffff), (18, 0x24292eff, 0xe1e4e8ff), (19, 0xd73a49ff, 0xf97583ff), (20, 0x24292eff, 0xe1e4e8ff), (31, 0x6f42c1ff, 0xb392f0ff), (39, 0x24292eff, 0xe1e4e8ff), (42, 0x005cc5ff, 0x79b8ffff), (43, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (8, 0x24292eff, 0xe1e4e8ff), (15, 0x005cc5ff, 0x79b8ffff), (16, 0x24292eff, 0xe1e4e8ff), (17, 0xd73a49ff, 0xf97583ff), (18, 0x24292eff, 0xe1e4e8ff), (27, 0x6f42c1ff, 0xb392f0ff), (38, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (8, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff), (23, 0x005cc5ff, 0x79b8ffff), (24, 0x24292eff, 0xe1e4e8ff), (31, 0x6f42c1ff, 0xb392f0ff), (32, 0xd73a49ff, 0xf97583ff), (44, 0x24292eff, 0xe1e4e8ff)],
    &[(6, 0xd73a49ff, 0xf97583ff), (23, 0x24292eff, 0xe1e4e8ff)],
    &[(6, 0xd73a49ff, 0xf97583ff), (24, 0x24292eff, 0xe1e4e8ff)],
    &[(6, 0xd73a49ff, 0xf97583ff), (25, 0x24292eff, 0xe1e4e8ff)],
    &[(6, 0xd73a49ff, 0xf97583ff), (22, 0x24292eff, 0xe1e4e8ff), (26, 0xd73a49ff, 0xf97583ff), (27, 0x24292eff, 0xe1e4e8ff), (28, 0x032f62ff, 0x9ecbffff), (29, 0xb31d28ff, 0xfdaeb7ff)],
    &[(6, 0xd73a49ff, 0xf97583ff), (22, 0x24292eff, 0xe1e4e8ff), (26, 0xd73a49ff, 0xf97583ff), (27, 0x24292eff, 0xe1e4e8ff), (29, 0x032f62ff, 0x9ecbffff), (30, 0xb31d28ff, 0xfdaeb7ff)],
    &[(6, 0xd73a49ff, 0xf97583ff), (15, 0x24292eff, 0xe1e4e8ff)],
    &[(6, 0xd73a49ff, 0xf97583ff), (16, 0x24292eff, 0xe1e4e8ff)],
    &[(6, 0xd73a49ff, 0xf97583ff), (17, 0x24292eff, 0xe1e4e8ff)],
    &[(6, 0xd73a49ff, 0xf97583ff), (18, 0x24292eff, 0xe1e4e8ff)],
    &[(6, 0xd73a49ff, 0xf97583ff), (19, 0x24292eff, 0xe1e4e8ff)],
    &[(6, 0xd73a49ff, 0xf97583ff), (20, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff), (18, 0x005cc5ff, 0x79b8ffff), (21, 0x24292eff, 0xe1e4e8ff), (22, 0xd73a49ff, 0xf97583ff), (24, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff), (18, 0x005cc5ff, 0x79b8ffff), (21, 0x24292eff, 0xe1e4e8ff), (22, 0xd73a49ff, 0xf97583ff), (25, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff), (18, 0x005cc5ff, 0x79b8ffff), (21, 0x24292eff, 0xe1e4e8ff), (22, 0xd73a49ff, 0xf97583ff), (26, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff), (18, 0x005cc5ff, 0x79b8ffff), (21, 0x24292eff, 0xe1e4e8ff), (22, 0xd73a49ff, 0xf97583ff), (27, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff), (18, 0x005cc5ff, 0x79b8ffff), (21, 0x24292eff, 0xe1e4e8ff), (22, 0xd73a49ff, 0xf97583ff), (23, 0x24292eff, 0xe1e4e8ff), (28, 0xd73a49ff, 0xf97583ff), (38, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff), (18, 0x005cc5ff, 0x79b8ffff), (21, 0x24292eff, 0xe1e4e8ff), (22, 0xd73a49ff, 0xf97583ff), (23, 0x24292eff, 0xe1e4e8ff), (28, 0xd73a49ff, 0xf97583ff), (39, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff), (18, 0x005cc5ff, 0x79b8ffff), (21, 0x24292eff, 0xe1e4e8ff), (22, 0xd73a49ff, 0xf97583ff), (23, 0x24292eff, 0xe1e4e8ff), (28, 0xd73a49ff, 0xf97583ff), (40, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (10, 0x24292eff, 0xe1e4e8ff), (18, 0x005cc5ff, 0x79b8ffff), (21, 0x24292eff, 0xe1e4e8ff), (22, 0xd73a49ff, 0xf97583ff), (23, 0x24292eff, 0xe1e4e8ff), (28, 0xd73a49ff, 0xf97583ff), (41, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (8, 0x24292eff, 0xe1e4e8ff), (14, 0x005cc5ff, 0x79b8ffff), (15, 0x24292eff, 0xe1e4e8ff), (16, 0xd73a49ff, 0xf97583ff), (18, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (8, 0x24292eff, 0xe1e4e8ff), (14, 0x005cc5ff, 0x79b8ffff), (15, 0x24292eff, 0xe1e4e8ff), (16, 0xd73a49ff, 0xf97583ff), (19, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (8, 0x24292eff, 0xe1e4e8ff), (14, 0x005cc5ff, 0x79b8ffff), (15, 0x24292eff, 0xe1e4e8ff), (16, 0xd73a49ff, 0xf97583ff), (20, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (8, 0x24292eff, 0xe1e4e8ff), (14, 0x005cc5ff, 0x79b8ffff), (15, 0x24292eff, 0xe1e4e8ff), (16, 0xd73a49ff, 0xf97583ff), (21, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (8, 0x24292eff, 0xe1e4e8ff), (14, 0x005cc5ff, 0x79b8ffff), (15, 0x24292eff, 0xe1e4e8ff), (16, 0xd73a49ff, 0xf97583ff), (22, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (8, 0x24292eff, 0xe1e4e8ff), (14, 0x005cc5ff, 0x79b8ffff), (15, 0x24292eff, 0xe1e4e8ff), (16, 0xd73a49ff, 0xf97583ff), (23, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (8, 0x24292eff, 0xe1e4e8ff), (14, 0x005cc5ff, 0x79b8ffff), (15, 0x24292eff, 0xe1e4e8ff), (16, 0xd73a49ff, 0xf97583ff), (24, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (8, 0x24292eff, 0xe1e4e8ff), (14, 0x005cc5ff, 0x79b8ffff), (15, 0x24292eff, 0xe1e4e8ff), (16, 0xd73a49ff, 0xf97583ff), (25, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (8, 0x24292eff, 0xe1e4e8ff), (14, 0x005cc5ff, 0x79b8ffff), (15, 0x24292eff, 0xe1e4e8ff), (16, 0xd73a49ff, 0xf97583ff), (26, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (7, 0xd73a49ff, 0xf97583ff), (8, 0x24292eff, 0xe1e4e8ff), (14, 0x005cc5ff, 0x79b8ffff), (15, 0x24292eff, 0xe1e4e8ff), (16, 0xd73a49ff, 0xf97583ff), (27, 0x24292eff, 0xe1e4e8ff)],
    &[(11, 0x24292eff, 0xe1e4e8ff), (12, 0x032f62ff, 0x9ecbffff), (13, 0xb31d28ff, 0xfdaeb7ff)],
    &[(11, 0x24292eff, 0xe1e4e8ff), (13, 0x032f62ff, 0x9ecbffff), (14, 0xb31d28ff, 0xfdaeb7ff)],
    &[(11, 0x24292eff, 0xe1e4e8ff), (14, 0x032f62ff, 0x9ecbffff), (15, 0xb31d28ff, 0xfdaeb7ff)],
    &[(11, 0x24292eff, 0xe1e4e8ff), (15, 0x032f62ff, 0x9ecbffff), (16, 0xb31d28ff, 0xfdaeb7ff)],
    &[(11, 0x24292eff, 0xe1e4e8ff), (16, 0x032f62ff, 0x9ecbffff), (17, 0xb31d28ff, 0xfdaeb7ff)],
    &[(11, 0x24292eff, 0xe1e4e8ff), (17, 0x032f62ff, 0x9ecbffff), (18, 0xb31d28ff, 0xfdaeb7ff)],
    &[(11, 0x24292eff, 0xe1e4e8ff), (18, 0x032f62ff, 0x9ecbffff), (19, 0xb31d28ff, 0xfdaeb7ff)],
    &[(11, 0x24292eff, 0xe1e4e8ff), (19, 0x032f62ff, 0x9ecbffff), (20, 0xb31d28ff, 0xfdaeb7ff)],
    &[(11, 0x24292eff, 0xe1e4e8ff), (20, 0x032f62ff, 0x9ecbffff), (21, 0xb31d28ff, 0xfdaeb7ff)],
    &[(11, 0x24292eff, 0xe1e4e8ff), (21, 0x032f62ff, 0x9ecbffff), (22, 0xb31d28ff, 0xfdaeb7ff)],
    &[(11, 0x24292eff, 0xe1e4e8ff), (22, 0x032f62ff, 0x9ecbffff), (23, 0xb31d28ff, 0xfdaeb7ff)],
    &[(11, 0x24292eff, 0xe1e4e8ff), (23, 0x032f62ff, 0x9ecbffff), (24, 0xb31d28ff, 0xfdaeb7ff)],
    &[(11, 0x24292eff, 0xe1e4e8ff), (24, 0x032f62ff, 0x9ecbffff), (25, 0xb31d28ff, 0xfdaeb7ff)],
    &[(11, 0x24292eff, 0xe1e4e8ff), (25, 0x032f62ff, 0x9ecbffff), (26, 0xb31d28ff, 0xfdaeb7ff)],
    &[(11, 0x24292eff, 0xe1e4e8ff), (26, 0x032f62ff, 0x9ecbffff), (27, 0xb31d28ff, 0xfdaeb7ff)],
    &[(11, 0x24292eff, 0xe1e4e8ff), (27, 0x032f62ff, 0x9ecbffff), (28, 0xb31d28ff, 0xfdaeb7ff)],
    &[(11, 0x24292eff, 0xe1e4e8ff), (28, 0x032f62ff, 0x9ecbffff), (29, 0xb31d28ff, 0xfdaeb7ff)],
    &[(11, 0x24292eff, 0xe1e4e8ff), (29, 0x032f62ff, 0x9ecbffff), (30, 0xb31d28ff, 0xfdaeb7ff)],
    &[(11, 0x24292eff, 0xe1e4e8ff), (30, 0x032f62ff, 0x9ecbffff), (31, 0xb31d28ff, 0xfdaeb7ff)],
    &[(11, 0x24292eff, 0xe1e4e8ff), (31, 0x032f62ff, 0x9ecbffff), (32, 0xb31d28ff, 0xfdaeb7ff)],
    &[(11, 0x24292eff, 0xe1e4e8ff), (32, 0x032f62ff, 0x9ecbffff), (33, 0xb31d28ff, 0xfdaeb7ff)],
    &[(11, 0x24292eff, 0xe1e4e8ff), (33, 0x032f62ff, 0x9ecbffff), (34, 0xb31d28ff, 0xfdaeb7ff)],
    &[(11, 0x24292eff, 0xe1e4e8ff), (34, 0x032f62ff, 0x9ecbffff), (35, 0xb31d28ff, 0xfdaeb7ff)],
    &[(11, 0x24292eff, 0xe1e4e8ff), (35, 0x032f62ff, 0x9ecbffff), (36, 0xb31d28ff, 0xfdaeb7ff)],
    &[(11, 0x24292eff, 0xe1e4e8ff), (36, 0x032f62ff, 0x9ecbffff), (37, 0xb31d28ff, 0xfdaeb7ff)],
    &[(11, 0x24292eff, 0xe1e4e8ff), (37, 0x032f62ff, 0x9ecbffff), (38, 0xb31d28ff, 0xfdaeb7ff)],
    &[(11, 0x24292eff, 0xe1e4e8ff), (38, 0x032f62ff, 0x9ecbffff), (39, 0xb31d28ff, 0xfdaeb7ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (8, 0xd73a49ff, 0xf97583ff), (17, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (8, 0xd73a49ff, 0xf97583ff), (19, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (8, 0xd73a49ff, 0xf97583ff), (20, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (8, 0xd73a49ff, 0xf97583ff), (21, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (8, 0xd73a49ff, 0xf97583ff), (22, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (8, 0xd73a49ff, 0xf97583ff), (23, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (8, 0xd73a49ff, 0xf97583ff), (24, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (8, 0xd73a49ff, 0xf97583ff), (25, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (8, 0xd73a49ff, 0xf97583ff), (26, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (8, 0xd73a49ff, 0xf97583ff), (27, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (8, 0xd73a49ff, 0xf97583ff), (28, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (8, 0xd73a49ff, 0xf97583ff), (29, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (8, 0xd73a49ff, 0xf97583ff), (30, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (8, 0xd73a49ff, 0xf97583ff), (31, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (8, 0xd73a49ff, 0xf97583ff), (32, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (8, 0xd73a49ff, 0xf97583ff), (33, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (8, 0xd73a49ff, 0xf97583ff), (34, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (8, 0xd73a49ff, 0xf97583ff), (35, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (8, 0xd73a49ff, 0xf97583ff), (36, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (8, 0xd73a49ff, 0xf97583ff), (37, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (8, 0xd73a49ff, 0xf97583ff), (38, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (8, 0xd73a49ff, 0xf97583ff), (39, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (8, 0xd73a49ff, 0xf97583ff), (40, 0x24292eff, 0xe1e4e8ff)],
    &[(2, 0x24292eff, 0xe1e4e8ff), (8, 0xd73a49ff, 0xf97583ff), (41, 0x24292eff, 0xe1e4e8ff)],
    &[(1, 0x6f42c1ff, 0xb392f0ff)],
    &[(2, 0x6f42c1ff, 0xb392f0ff)],
    &[(3, 0x6f42c1ff, 0xb392f0ff)],
    &[(4, 0x6f42c1ff, 0xb392f0ff)],
    &[(5, 0x6f42c1ff, 0xb392f0ff)],
    &[(6, 0x6f42c1ff, 0xb392f0ff)],
    &[(7, 0x6f42c1ff, 0xb392f0ff)],
    &[(8, 0x6f42c1ff, 0xb392f0ff)],
    &[(9, 0x6f42c1ff, 0xb392f0ff)],
    &[(10, 0x6f42c1ff, 0xb392f0ff)],
    &[(11, 0x6f42c1ff, 0xb392f0ff)],
    &[(12, 0x6f42c1ff, 0xb392f0ff)],
    &[(13, 0x6f42c1ff, 0xb392f0ff)],
    &[(14, 0x6f42c1ff, 0xb392f0ff)],
    &[(15, 0x6f42c1ff, 0xb392f0ff)],
    &[(16, 0x6f42c1ff, 0xb392f0ff)],
    &[(17, 0x6f42c1ff, 0xb392f0ff)],
    &[(18, 0x6f42c1ff, 0xb392f0ff)],
];
    #[rustfmt::skip]
static FIXTURES: &[Fixture] = &[
    Fixture { source: STREAMING_0, rows: &[77, 78, 1, 79, 80, 81, 82, 1, 83, 9], tails: &[(1, 9), (2, 17), (3, 10), (4, 7), (5, 18), (8, 19), (9, 20), (10, 21), (11, 22), (12, 23), (13, 24), (14, 25), (59, 10), (60, 7), (61, 18), (62, 26), (93, 27), (99, 28), (100, 29), (101, 30), (102, 31), (103, 32), (104, 33), (105, 34), (106, 35), (118, 36), (124, 13), (125, 14), (126, 37), (127, 12), (128, 6), (129, 38), (130, 39), (131, 40), (132, 41), (146, 18), (147, 26), (148, 15), (149, 42), (162, 43), (163, 44), (164, 45), (165, 46), (166, 47), (167, 48), (168, 49), (169, 50), (170, 51), (171, 52), (179, 53), (180, 54), (181, 55), (182, 56), (183, 57), (184, 58), (185, 59), (186, 60), (187, 61), (188, 62), (189, 63), (190, 64), (211, 18), (212, 26), (213, 15), (214, 42), (215, 13), (224, 65), (225, 66), (226, 67), (227, 68), (228, 69), (229, 70), (230, 71), (231, 72), (232, 73), (233, 74), (234, 75), (235, 76), (270, 10), (271, 7), (272, 18), (273, 26), (274, 15)] },
    Fixture { source: STREAMING_1, rows: &[84], tails: &[] },
    Fixture { source: STREAMING_2, rows: &[115, 11, 116, 117, 118, 1, 119, 9], tails: &[(2, 9), (3, 17), (4, 10), (5, 7), (6, 18), (9, 19), (10, 20), (11, 21), (12, 22), (13, 23), (14, 24), (15, 25), (32, 10), (33, 7), (34, 18), (35, 26), (58, 85), (64, 86), (65, 87), (66, 88), (67, 89), (68, 90), (69, 91), (70, 92), (71, 93), (80, 10), (81, 7), (82, 18), (83, 26), (96, 94), (97, 95), (98, 96), (99, 97), (100, 98), (101, 99), (102, 100), (103, 101), (104, 102), (116, 10), (117, 7), (118, 18), (119, 26), (134, 103), (135, 104), (136, 105), (137, 106), (138, 107), (139, 108), (140, 109), (141, 110), (142, 111), (143, 112), (144, 113), (161, 10), (162, 7), (163, 18), (164, 26), (177, 94), (178, 95), (179, 96), (180, 97), (181, 98), (182, 99), (183, 100), (184, 101), (185, 102), (202, 10), (203, 7), (204, 18), (205, 26), (206, 15), (242, 114)] },
    Fixture { source: STREAMING_3, rows: &[0, 1, 2, 3, 1, 4, 5, 6, 7, 1, 8, 9], tails: &[(1, 9), (2, 17), (3, 10), (4, 7), (5, 18), (23, 120), (24, 121), (25, 122), (29, 123), (30, 124), (34, 9), (35, 17), (36, 10), (37, 7), (38, 18), (41, 19), (42, 20), (43, 21), (44, 22), (45, 23), (46, 24), (47, 25), (48, 125), (49, 126), (50, 127), (51, 128), (52, 129), (53, 130), (83, 10), (84, 7), (85, 18), (86, 26), (104, 131), (105, 132), (106, 133), (107, 134), (118, 135), (119, 136), (120, 137), (121, 138), (128, 10), (129, 7), (130, 18), (131, 26), (143, 139), (144, 140), (145, 141), (146, 142), (147, 143), (148, 144), (149, 145), (150, 146), (151, 147), (152, 148), (168, 149), (169, 150), (170, 151), (171, 152), (172, 153), (173, 154), (174, 155), (175, 156), (176, 157), (177, 158), (178, 159), (179, 160), (180, 161), (181, 162), (182, 163), (183, 164), (184, 165), (185, 166), (186, 167), (187, 168), (188, 169), (189, 170), (190, 171), (191, 172), (192, 173), (193, 174), (194, 175), (220, 10), (221, 7), (222, 18), (223, 26), (224, 15), (234, 176), (235, 83), (236, 177), (237, 178), (238, 179), (239, 180), (240, 181), (241, 182), (242, 183), (243, 184), (244, 185), (245, 186), (246, 187), (247, 188), (248, 189), (249, 190), (250, 191), (251, 192), (252, 193), (253, 194), (254, 195), (255, 196), (256, 197), (257, 198), (258, 199)] },
    Fixture { source: STREAMING_4, rows: &[16], tails: &[(1, 200), (2, 201), (3, 202), (4, 203), (5, 204), (6, 205), (7, 206), (8, 207), (9, 208), (10, 209), (11, 210), (12, 211), (13, 212), (14, 213), (15, 214), (16, 215), (17, 216), (18, 217)] },
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
        ("hooks/use-debounce.ts", "ts", STREAMING_0),
        ("Terminal", "sh", STREAMING_1),
        ("components/search.tsx", "tsx", STREAMING_2),
        ("app/api/chat/route.ts", "ts", STREAMING_3),
        (".env.local", "sh", STREAMING_4),
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
