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

fn markdown_bytes(start: Instant) -> usize {
    markdown_prefix(start.elapsed().as_millis() as usize / 30)
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
fn prose_settings(dark: bool) -> markdown::Settings {
    let theme = if dark { Theme::dark() } else { Theme::light() };
    markdown::Settings::with_text_size(14, markdown::Style::from(theme.iced_theme()))
}
fn total_length(id: &str) -> usize {
    if id == "markdown" { MARKDOWN.len() } else { 0 }
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
    &[(13, 0x24292eff, 0xe1e4e8ff)],
    &[(4, 0x24292eff, 0xe1e4e8ff)],
    &[(1, 0x24292eff, 0xe1e4e8ff)],
    &[(3, 0x24292eff, 0xe1e4e8ff)],
    &[(12, 0x24292eff, 0xe1e4e8ff)],
    &[(9, 0x24292eff, 0xe1e4e8ff)],
    &[(10, 0x24292eff, 0xe1e4e8ff)],
    &[(7, 0x24292eff, 0xe1e4e8ff)],
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
];
    #[rustfmt::skip]
static FIXTURES: &[Fixture] = &[
    Fixture { source: STREAMING_0, rows: &[69, 70, 0, 71, 72, 73, 74, 0, 75, 3], tails: &[(1, 3), (2, 9), (3, 4), (4, 2), (5, 10), (8, 11), (9, 12), (10, 13), (11, 14), (12, 15), (13, 16), (14, 17), (59, 4), (60, 2), (61, 10), (62, 18), (93, 19), (99, 20), (100, 21), (101, 22), (102, 23), (103, 24), (104, 25), (105, 26), (106, 27), (118, 28), (124, 6), (125, 7), (126, 29), (127, 5), (128, 1), (129, 30), (130, 31), (131, 32), (132, 33), (146, 10), (147, 18), (148, 8), (149, 34), (162, 35), (163, 36), (164, 37), (165, 38), (166, 39), (167, 40), (168, 41), (169, 42), (170, 43), (171, 44), (179, 45), (180, 46), (181, 47), (182, 48), (183, 49), (184, 50), (185, 51), (186, 52), (187, 53), (188, 54), (189, 55), (190, 56), (211, 10), (212, 18), (213, 8), (214, 34), (215, 6), (224, 57), (225, 58), (226, 59), (227, 60), (228, 61), (229, 62), (230, 63), (231, 64), (232, 65), (233, 66), (234, 67), (235, 68), (270, 4), (271, 2), (272, 10), (273, 18), (274, 8)] },
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
