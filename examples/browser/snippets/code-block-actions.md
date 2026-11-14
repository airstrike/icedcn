```rust
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
            .max_width(448)
            .into()
    }
    fn content(&self, dark: bool) -> Slot<'_, Message> {
        let normal = |_: usize, _: &str| LineStyle::Default;
        let id = "actions";
        {
            let wrap = *self.wraps.get("actions").unwrap_or(&true);
            self.header(
                "actions",
                "Terminal",
                ACTIONS,
                "clippy.log",
                terminal(ACTIONS).wrap(wrap),
            )
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

fn terminal(source: &str) -> CodeBlock<'_, Message> {
    code_block(source).font(Font::new("Geist Mono"))
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

const ACTIONS: &str = r##"$ cargo clippy -- -D warnings
    Checking notebook v0.1.0
error: writing `&String` instead of `&str` involves a new object where a slice will do
 --> src/search.rs:4:18
  |
4 | fn matches(query: &String, title: &str) -> bool {
  |                   ^^^^^^^ help: change this to: `&str`"##;
```
