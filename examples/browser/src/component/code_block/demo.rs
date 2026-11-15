use iced::time::{Duration, Instant};
use std::collections::BTreeMap;

use iced::alignment::Horizontal;
use iced::widget::Id;
use iced::{Element, Fill, Widget};
use icedcn::widget::code_block::LineStyle;
use icedcn::widget::{
    CodeBlock, bubble, button, code_block, column, container, markdown, message, row, tabs, text,
    trace,
};
use icedcn::{Size, Slot, Theme};

use super::samples::*;
use crate::{icon, page};
use iced::widget::{Action, Component, component};
use iced::{mouse, window};

#[derive(Clone, Debug)]
pub enum Message {
    Wrap(&'static str),
    Replay(&'static str),
    Idle,
    Acknowledge,
    Tick(Instant),
    Save(&'static str, &'static str),
    Saved(&'static str, Result<bool, String>),
}

pub struct State {
    target: Id,
    receipts: Vec<Id>,
    acknowledgements: Vec<Id>,
    progress: BTreeMap<&'static str, usize>,
    last_tick: Instant,
    wraps: BTreeMap<&'static str, bool>,
    streaming: BTreeMap<&'static str, Instant>,
    save_status: BTreeMap<&'static str, String>,
    logs: String,
    markdown_before: Vec<markdown::Item>,
    markdown_after: Vec<markdown::Item>,
    typeset_before: Vec<markdown::Item>,
    typeset_after: Vec<markdown::Item>,
}

impl State {
    fn new() -> Self {
        Self {
            target: Id::unique(),
            receipts: Vec::new(),
            acknowledgements: Vec::new(),
            progress: BTreeMap::new(),
            last_tick: Instant::now(),
            wraps: BTreeMap::new(),
            streaming: BTreeMap::new(),
            save_status: BTreeMap::new(),
            typeset_before: markdown::parse("Read the notebook index with `read_to_string` and return one title per line.").collect(),
            typeset_after: markdown::parse("The caller receives an `io::Result` and can display a read error in the application.").collect(),
            markdown_before: markdown_parts(usize::MAX).0,
            markdown_after: markdown_parts(usize::MAX).1,
            logs: (0..40)
                .map(|i| {
                    format!(
                        "[{:02}] Indexed notebook in {}ms",
                        i + 1,
                        12 + (i * 7) % 40
                    )
                })
                .collect::<Vec<_>>()
                .join("\n"),
        }
    }
    fn update(&mut self, event: Message) -> Option<super::Event> {
        match event {
            Message::Wrap(id) => {
                let value = self.wraps.entry(id).or_insert(id == "actions");
                *value = !*value;
            }
            Message::Save(name, value) => {
                return Some(super::Event::Save {
                    target: self.target.clone(),
                    name,
                    value,
                });
            }
            Message::Saved(name, Ok(true)) => {
                self.save_status.insert(name, format!("Saved {name}"));
            }
            Message::Saved(_, Ok(false)) => {}
            Message::Saved(name, Err(error)) => {
                self.save_status
                    .insert(name, format!("Could not save {name}: {error}"));
            }
            Message::Replay(id) => {
                self.streaming.insert(id, Instant::now());
                self.progress.insert(id, 0);
                if id == "markdown" {
                    (self.markdown_before, self.markdown_after) = markdown_parts(0);
                }
            }
            Message::Tick(now) => self.advance(now),
            Message::Idle => {}
            Message::Acknowledge => {
                return Some(super::Event::Acknowledged(std::mem::take(
                    &mut self.acknowledgements,
                )));
            }
        }
        None
    }
    fn advance(&mut self, now: Instant) {
        self.last_tick = now;
        let previous = self.progress.get("markdown").copied();
        self.streaming.retain(|id, start| {
            let elapsed = now.saturating_duration_since(*start).as_millis() as usize;
            let count = if *id == "markdown" {
                markdown_prefix(elapsed / 30)
            } else {
                elapsed / 16 * 3
            };
            self.progress.insert(id, count.min(total_length(id)));
            count < total_length(id)
        });
        if previous != self.progress.get("markdown").copied() {
            let count = self.progress.get("markdown").copied().unwrap_or(usize::MAX);
            (self.markdown_before, self.markdown_after) = markdown_parts(count);
        }
    }
    fn deadline(&self) -> Option<Instant> {
        (!self.streaming.is_empty()).then(|| {
            self.last_tick
                + Duration::from_millis(if self.streaming.contains_key("markdown") {
                    30
                } else {
                    16
                })
        })
    }
    fn receive(&mut self, completion: &Completion) {
        if completion.target == self.target && !self.receipts.contains(&completion.serial) {
            self.receipts.push(completion.serial.clone());
            self.acknowledgements.push(completion.serial.clone());
            let _ = self.update(completion.message.clone());
        }
    }
    fn header(
        &self,
        id: &'static str,
        title: &'static str,
        source: &'static str,
        filename: &'static str,
        block: CodeBlock<'static, Message>,
    ) -> Slot<'static, Message> {
        let wrapped = *self.wraps.get(id).unwrap_or(&(id == "actions"));
        let wrap = code_block::action(icon::wrap_text().size(14), "Wrap lines")
            .active(wrapped)
            .on_press(Message::Wrap(id));
        let save = code_block::action(icon::download().size(14), "Download")
            .on_press(Message::Save(filename, source));
        let title = if id == "actions" {
            code_block::terminal_title(title)
        } else {
            row![icon::file_code().size(14), code_block::title(title)]
                .spacing(6)
                .into()
        };
        let mut block = block
            .title(title)
            .actions(row![wrap, save].spacing(4))
            .copyable(true);
        if let Some(status) = self.save_status.get(filename) {
            block = block.footer(text::muted(status.clone()));
        }
        block.into()
    }
    fn replay(&self, id: &'static str) -> Slot<'static, Message> {
        button![icon::rotate_ccw().size(14).line_height(1.0), "Replay"]
            .style(button::Outline)
            .size(Size::Sm)
            .disabled(self.streaming.contains_key(id))
            .on_press(Message::Replay(id))
            .into()
    }
    fn streaming(&self) -> Slot<'static, Message> {
        let mut remaining = self
            .streaming
            .get("streaming")
            .and_then(|_| self.progress.get("streaming").copied())
            .unwrap_or(usize::MAX);
        let mut output = column![].width(Fill).spacing(16);
        for &(title, language, source) in STREAMS {
            if remaining == 0 {
                break;
            }
            let count = remaining.min(source.len());
            let visible = &source[..source.floor_char_boundary(count)];
            let mut block = terminal(visible).language(language);
            if source == STREAMING_2 {
                block = block.diff();
            } else if source == STREAMING_3 {
                for line in 6..=9 {
                    block = block.line_style(line, LineStyle::Highlighted);
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
            remaining = remaining.saturating_sub(count);
        }
        output.push(self.replay("streaming")).into()
    }
    fn markdown(&self, dark: bool) -> Slot<'static, Message> {
        let end = self
            .streaming
            .get("markdown")
            .and_then(|_| self.progress.get("markdown").copied())
            .unwrap_or(usize::MAX)
            .min(MARKDOWN.len());
        let body = &MARKDOWN[..end];
        let (_, fenced) = body.split_once("```rust\n").unwrap_or((body, ""));
        let mut reply = column![prose(&self.markdown_before, dark)]
            .width(Fill)
            .spacing(12);
        if !fenced.is_empty() {
            let (code, _) = fenced.split_once("```").unwrap_or((fenced, ""));
            let complete = fenced.contains("```");
            reply = reply.push(
                terminal(code.trim_end_matches('\n'))
                    .language("rust")
                    .title("rust")
                    .actions(code_block::copy_button(code).disabled(!complete))
                    .into(),
            );
        }
        reply = reply.push(prose(&self.markdown_after, dark));
        let mut items = column![
            message(bubble("How do I ignore case when searching?").style(bubble::Variant::Muted))
                .align(Horizontal::Right),
            message(reply),
        ]
        .width(Fill)
        .spacing(16);
        if !self.streaming.contains_key("markdown") {
            items = items.push(
                button("Replay")
                    .size(Size::Sm)
                    .style(button::Outline)
                    .on_press(Message::Replay("markdown"))
                    .into(),
            );
        }
        container(items)
            .padding([24, 0])
            .min_height(448)
            .width(Fill)
            .into()
    }
    fn example(&self, id: &str, dark: bool) -> Slot<'static, Message> {
        match id {
            "demo" => {
                let wrap = *self.wraps.get("demo").unwrap_or(&false);
                let mut block = terminal(DEMO).language("rust").wrap(wrap).start_line(1);
                for line in 6..=10 {
                    block = block.line_style(line, LineStyle::Highlighted);
                }
                self.header("demo", "src/view.rs", DEMO, "view.rs", block)
            }
            "basic" => terminal(BASIC).into(),
            "header" => terminal(HEADER)
                .title(code_block::terminal_title("Terminal"))
                .actions(code_block::copy_button("cargo check -p icedcn_widget"))
                .into(),
            "actions" => {
                let wrap = *self.wraps.get("actions").unwrap_or(&true);
                self.header(
                    "actions",
                    "Terminal",
                    ACTIONS,
                    "clippy.log",
                    terminal(ACTIONS).wrap(wrap),
                )
            }
            "floating-actions" => terminal(FLOATING_ACTIONS).copyable(true).into(),
            "line-numbers" => terminal(LINE_NUMBERS).start_line(1).into(),
            "start-line" => terminal(START_LINE)
                .language("rust")
                .title("src/notebooks.rs")
                .start_line(995)
                .into(),
            "wrap" => column![terminal(WRAP), terminal(WRAP).wrap(true)]
                .spacing(16)
                .into(),
            "highlighted-lines" => terminal(HIGHLIGHTED_LINES)
                .line_style(3, LineStyle::Highlighted)
                .line_style(4, LineStyle::Highlighted)
                .start_line(1)
                .into(),
            "diff" => terminal(DIFF)
                .language("rust")
                .diff()
                .title("src/search.rs")
                .copyable(true)
                .into(),
            "height" => terminal(self.logs.clone())
                .max_height(192.)
                .title("Logs")
                .copyable(true)
                .into(),
            "show-more" | "show-more-scroll" => {
                let scroll = id == "show-more-scroll";
                code_block::show_more(256, move || {
                    let block = terminal(SHOW_MORE)
                        .language("rust")
                        .title("src/notebook.rs")
                        .start_line(1)
                        .copyable(true);
                    if scroll {
                        block.max_height(384.)
                    } else {
                        block
                    }
                })
                .into()
            }
            "size" => column![
                terminal(SIZE)
                    .wrap(true)
                    .title(code_block::terminal_title("Terminal"))
                    .copyable(true),
                terminal(SIZE)
                    .wrap(true)
                    .title(code_block::terminal_title("Terminal"))
                    .compact(true)
                    .copyable(true)
            ]
            .spacing(16)
            .into(),
            "destructive" => error_block(DESTRUCTIVE)
                .title("Error")
                .copyable(true)
                .into(),
            "trace" => column![
                trace_output("Ran cargo --version", TRACE, false),
                trace_output("Failed to run cargo run --bin notes", TRACE_ERROR, true)
            ]
            .spacing(8)
            .into(),
            "tabs" => tabs(move |t| {
                let mut body = column![t.list([
                    t.trigger("view", "view.rs"),
                    t.trigger("search", "search.rs"),
                    t.trigger("config", "config.toml")
                ])]
                .spacing(8);
                for (id, title, language, source) in [
                    ("view", "src/view.rs", "rust", TABS_0),
                    ("search", "src/search.rs", "rust", TABS_1),
                    ("config", ".cargo/config.toml", "toml", TABS_2),
                ] {
                    body = body.push(
                        t.content(
                            id,
                            terminal(source)
                                .language(language)
                                .start_line(1)
                                .title(title)
                                .copyable(true),
                        ),
                    );
                }
                body.into()
            })
            .default_value("view")
            .into(),
            "chat" => column![
                message(bubble("How do I filter notebook titles?").style(bubble::Variant::Muted))
                    .align(Horizontal::Right),
                message(
                    column![
                        text::p("Compare the query and each title in lowercase."),
                        terminal(CHAT)
                            .language("rust")
                            .title("src/search.rs")
                            .copyable(true),
                        text::p("The result borrows the matching titles from the original list.")
                    ]
                    .spacing(12)
                )
            ]
            .spacing(16)
            .into(),
            "typeset" => column![
                text::h3("Read a notebook index"),
                prose(&self.typeset_before, dark),
                terminal(TYPESET)
                    .language("rust")
                    .start_line(1)
                    .title(
                        row![
                            icon::file_code().size(14),
                            code_block::title("src/index.rs")
                        ]
                        .spacing(6)
                    )
                    .copyable(true),
                prose(&self.typeset_after, dark)
            ]
            .spacing(12)
            .into(),
            "highlighting" => terminal(HIGHLIGHTING)
                .language("rust")
                .title("src/notebook.rs")
                .into(),
            "languages" => {
                let mut samples = column![].spacing(16);
                for (title, language, source) in [
                    ("Cargo.toml", "toml", LANGUAGES_0),
                    ("src/main.rs", "rust", LANGUAGES_1),
                    ("notes.txt", "text", LANGUAGES_2),
                ] {
                    samples = samples.push(terminal(source).language(language).title(title).into());
                }
                samples.into()
            }
            "ansi" => {
                use code_block::ansi;
                use std::sync::LazyLock;
                static OUTPUT: LazyLock<ansi::Output> = LazyLock::new(|| ansi::parse(ANSI));
                OUTPUT
                    .view()
                    .font("Geist Mono")
                    .title(code_block::terminal_title("Terminal"))
                    .copyable(true)
                    .into()
            }
            "streaming" => self.streaming(),
            "markdown" => self.markdown(dark),
            _ => unreachable!("unknown Code Block example {id}"),
        }
    }
}

fn terminal<'a>(source: impl Into<std::borrow::Cow<'a, str>>) -> CodeBlock<'a, Message> {
    code_block(source).font("Geist Mono")
}
fn error_block(source: &str) -> CodeBlock<'_, Message> {
    terminal(source).wrap(true).style(code_block::Destructive)
}
fn trace_output(title: &'static str, source: &'static str, error: bool) -> Slot<'static, Message> {
    trace::disclosure(
        move || {
            trace::line(title)
                .icon(icon::terminal().size(14))
                .status(if error {
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

fn total_length(id: &str) -> usize {
    if id == "markdown" {
        MARKDOWN.len()
    } else {
        STREAMS.iter().map(|(_, _, s)| s.len()).sum()
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

fn prose(items: &[markdown::Item], dark: bool) -> Slot<'static, Message> {
    let mut output = column![].width(Fill).spacing(12);
    for item in items {
        if let markdown::Item::Paragraph(text) = item {
            output = output
                .push(markdown::paragraph(prose_settings(dark), text, |_| Message::Idle).into());
        }
    }
    output.into()
}
fn prose_settings(dark: bool) -> markdown::Settings {
    let theme = if dark { Theme::dark() } else { Theme::light() };
    markdown::Settings::with_text_size(14, markdown::Style::from(theme.iced_theme()))
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

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Debug)]
pub struct Completion {
    pub target: Id,
    pub serial: Id,
    pub message: Message,
}
struct Demo {
    id: String,
    dark: bool,
    page: usize,
    completions: Vec<Completion>,
}
pub fn show(
    id: &str,
    dark: bool,
    page: usize,
    completions: Vec<Completion>,
) -> Element<'static, crate::Message, Theme> {
    component(Demo {
        id: id.to_owned(),
        dark,
        page,
        completions,
    })
    .boxed()
}
impl Component<'static, crate::Message, Theme> for Demo {
    type State = State;
    type Event = Message;
    fn update(
        &self,
        state: &mut State,
        event: Message,
        _: &iced::Renderer,
    ) -> Option<crate::Message> {
        state
            .update(event)
            .map(|effect| crate::Message::Page(self.page, page::Message::CodeBlock(effect)))
    }
    fn view(&self, state: &State) -> impl Widget<Message, Theme> + 'static {
        state.example(&self.id, self.dark).compose()
    }
    fn diff(&mut self, state: &mut State) {
        state.receipts.retain(|serial| {
            self.completions
                .iter()
                .any(|completion| &completion.serial == serial)
        });
        for completion in &self.completions {
            state.receive(completion);
        }
    }
    fn listen(
        &self,
        state: &State,
        event: &iced::Event,
        _: iced::Rectangle,
        _: mouse::Cursor,
    ) -> Action<Message> {
        if let iced::Event::Window(window::Event::RedrawRequested(now)) = event {
            if !state.acknowledgements.is_empty() {
                return Action::publish(Message::Acknowledge)
                    .and_request_redraw_at(*now + Duration::from_millis(16));
            }
            if let Some(deadline) = state.deadline() {
                return if *now >= deadline {
                    Action::publish(Message::Tick(*now))
                        .and_request_redraw_at(*now + Duration::from_millis(16))
                } else {
                    Action::request_redraw_at(deadline)
                };
            }
        }
        Action::none()
    }
}
#[cfg(test)]
#[path = "tests.rs"]
mod tests;
