```rust
use iced::alignment::Horizontal;
use iced::{Fill, Font, Subscription, Widget, time};
use icedxdy::{
    Size, Slot, Theme,
    widget::{bubble, button, code_block, column, container, markdown, message},
};
use std::time::{Duration, Instant};
#[derive(Clone, Debug)]
pub enum Message {
    Replay,
    Tick(Instant),
    Link,
}
pub struct Demo {
    started: Option<Instant>,
    end: usize,
    before: Vec<markdown::Item>,
    after: Vec<markdown::Item>,
}
impl Default for Demo {
    fn default() -> Self {
        let (before, after) = markdown_parts(MARKDOWN.len());
        Self {
            started: None,
            end: MARKDOWN.len(),
            before,
            after,
        }
    }
}
impl Demo {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::Replay => {
                self.started = Some(Instant::now());
                self.end = 0;
            }
            Message::Tick(now) => {
                let Some(start) = self.started else { return };
                self.end =
                    markdown_prefix(now.saturating_duration_since(start).as_millis() as usize / 30);
                if self.end == MARKDOWN.len() {
                    self.started = None;
                }
            }
            Message::Link => return,
        }
        (self.before, self.after) = markdown_parts(self.end);
    }
    pub fn subscription(&self) -> Subscription<Message> {
        if self.started.is_some() {
            time::every(Duration::from_millis(30)).map(Message::Tick)
        } else {
            Subscription::none()
        }
    }
    pub fn view(&self, theme: &Theme) -> Slot<'_, Message> {
        let settings =
            markdown::Settings::with_text_size(14, markdown::Style::from(theme.iced_theme()));
        let mut reply = column![
            markdown::view(&self.before, settings)
                .map(|_| Message::Link)
                .boxed()
        ]
        .width(Fill)
        .spacing(12);
        if let Some((_, fenced)) = MARKDOWN[..self.end].split_once("```rust\n")
            && !fenced.is_empty()
        {
            let (code, complete) = fenced
                .split_once("```")
                .map_or((fenced, false), |(code, _)| (code, true));
            reply = reply.push(
                code_block(code.trim_end_matches('\n'))
                    .language("rust")
                    .font(Font::new("Geist Mono"))
                    .title(code_block::title("rust"))
                    .actions(code_block::copy_button(code).disabled(!complete))
                    .into(),
            );
        }
        reply = reply.push(
            markdown::view(&self.after, settings)
                .map(|_| Message::Link)
                .boxed()
                .into(),
        );
        let mut items = column![
            message(bubble("How do I ignore case when searching?").style(bubble::Variant::Muted))
                .align(Horizontal::Right),
            message(reply)
        ]
        .width(Fill)
        .spacing(16);
        if self.started.is_none() {
            items = items.push(
                button("Replay")
                    .size(Size::Sm)
                    .style(button::Outline)
                    .on_press(Message::Replay)
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
fn markdown_parts(end: usize) -> (Vec<markdown::Item>, Vec<markdown::Item>) {
    let body = &MARKDOWN[..end.min(MARKDOWN.len())];
    let (before, fenced) = body.split_once("```rust\n").unwrap_or((body, ""));
    let after = fenced
        .split_once("```")
        .map(|(_, after)| after)
        .unwrap_or("");
    (
        markdown::parse(before).collect(),
        markdown::parse(after).collect(),
    )
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

const MARKDOWN: &str = "Normalize the query and title with `to_lowercase` before comparing them.\n\n```rust\nfn matches(query: &str, title: &str) -> bool {\n    let query = query.trim().to_lowercase();\n    title.to_lowercase().contains(&query)\n}\n```\n\nTrimming the query also ignores extra spaces at either end.";
```
