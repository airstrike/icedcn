```rust
use crate::icon;
use iced::{Fill, Subscription, time};
use icedcn::widget::{button, code_block, column};
use icedcn::{Size, Slot};
use std::time::{Duration, Instant};
#[derive(Clone, Debug)]
pub enum Message {
    Replay,
    Tick(Instant),
}
#[derive(Default)]
pub struct Demo {
    started: Option<Instant>,
    visible: usize,
}
impl Demo {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::Replay => {
                self.visible = 0;
                self.started = Some(Instant::now());
            }
            Message::Tick(now) => {
                if let Some(start) = self.started {
                    self.visible =
                        (now.saturating_duration_since(start).as_millis() as usize / 16) * 3;
                    if self.visible >= STREAMS.iter().map(|(_, _, s)| s.len()).sum::<usize>() {
                        self.started = None;
                    }
                }
            }
        }
    }
    pub fn subscription(&self) -> Subscription<Message> {
        if self.started.is_some() {
            time::every(Duration::from_millis(16)).map(Message::Tick)
        } else {
            Subscription::none()
        }
    }
    pub fn view(&self) -> Slot<'static, Message> {
        let mut remaining = if self.started.is_some() {
            self.visible
        } else {
            usize::MAX
        };
        let mut output = column![].spacing(16).width(Fill);
        for &(title, language, source) in STREAMS {
            if remaining == 0 {
                break;
            }
            let count = remaining.min(source.len());
            let visible = &source[..source.floor_char_boundary(count)];
            let mut block = code_block(visible).language(language).font("Geist Mono");
            if source == STREAMING_2 {
                block = block.diff();
            } else if source == STREAMING_3 {
                for line in 6..=9 {
                    block = block.line_style(line, code_block::LineStyle::Highlighted);
                }
            }
            if title != "Terminal" {
                let copy = if source == STREAMING_2 {
                    code_block::copy_button(code_block::diff_text(source))
                } else {
                    code_block::copy_button(source)
                };
                block = block
                    .title(title)
                    .actions(copy.disabled(count < source.len()));
            }
            if !matches!(title, "Terminal" | ".cargo/config.toml") {
                block = block.start_line(1);
            }
            output = output.push(block.into());
            remaining -= count;
        }
        output
            .push(
                button![icon::rotate_ccw().size(14).line_height(1.0), "Replay"]
                    .style(button::Outline)
                    .size(Size::Sm)
                    .disabled(self.started.is_some())
                    .on_press(Message::Replay)
                    .into(),
            )
            .into()
    }
}
const STREAMS: &[(&str, &str, &str)] = &[
    ("src/matcher.rs", "rust", STREAMING_0),
    ("Terminal", "sh", STREAMING_1),
    ("src/search.rs", "rust", STREAMING_2),
    ("src/search.rs", "rust", STREAMING_3),
    (".cargo/config.toml", "toml", STREAMING_4),
];

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
