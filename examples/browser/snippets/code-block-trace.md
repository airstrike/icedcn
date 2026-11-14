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
        let id = "trace";
        column![
            trace_output("Ran node --version", TRACE, false),
            trace_output("Failed to run pnpm dev", TRACE_ERROR, true)
        ]
        .spacing(8)
        .into()
    }
}

fn trace_output(title: &'static str, source: &'static str, error: bool) -> Slot<'static, Message> {
    trace::disclosure(
        move || {
            trace::line(title).icon(icon("terminal")).status(if error {
                trace::Status::Error
            } else {
                trace::Status::Complete
            })
        },
        move || {
            let code = if error {
                error_block(source)
            } else {
                terminal(source).wrap(true)
            };
            code.compact(true).max_height(256.).into()
        },
    )
    .default_open(true)
    .into()
}
fn error_block(source: &str) -> CodeBlock<'_, Message> {
    terminal(source).wrap(true).style(code_block::Destructive)
}
fn terminal(source: &str) -> CodeBlock<'_, Message> {
    code_block(source).font(Font::new("Geist Mono"))
}
fn icon(name: &str) -> Slot<'static, Message> {
    let path = match name {
        "terminal" => r#"<path d="m4 17 6-5-6-5m8 12h8"/>"#,
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

const TRACE: &str = r##"$ node --version
v24.18.0"##;

const TRACE_ERROR: &str = r##"$ pnpm dev
Error: listen EADDRINUSE: address already in use :::3000
    at Server.setupListenHandle [as _listen2] (node:net:1908:16)"##;
```
