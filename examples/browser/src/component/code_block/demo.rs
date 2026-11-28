use iced::time::{Duration, Instant};
use std::collections::BTreeMap;

use iced::alignment::Horizontal;
use iced::{Element, Fill, Widget};
use icedcn::widget::code_block;
use icedcn::widget::code_block::{LineStyle, ansi};
use icedcn::widget::{bubble, button, column, container, markdown, message, tabs, text, trace};
use icedcn::{Destructive, Size, Theme, View};

use super::samples::*;
use crate::{icon, page};
use iced::widget::{Action, Component, component};
use iced::{mouse, window};

#[derive(Clone, Debug)]
pub enum Message {
    Wrap(&'static str),
    Save(&'static str, &'static str),
    Replay(&'static str),
    Idle,
    Tick(Instant),
}

pub struct State {
    progress: BTreeMap<&'static str, usize>,
    last_tick: Instant,
    wraps: BTreeMap<&'static str, bool>,
    streaming: BTreeMap<&'static str, Instant>,
    logs: String,
    markdown_before: Vec<markdown::Item>,
    markdown_after: Vec<markdown::Item>,
    typeset_before: Vec<markdown::Item>,
    typeset_after: Vec<markdown::Item>,
}

impl State {
    fn new() -> Self {
        Self {
            progress: BTreeMap::new(),
            last_tick: Instant::now(),
            wraps: BTreeMap::new(),
            streaming: BTreeMap::new(),
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
            Message::Save(name, value) => return Some(super::Event::Save { name, value }),
            Message::Replay(id) => {
                self.streaming.insert(id, Instant::now());
                self.progress.insert(id, 0);
                if id == "markdown" {
                    (self.markdown_before, self.markdown_after) = markdown_parts(0);
                }
            }
            Message::Tick(now) => self.advance(now),
            Message::Idle => {}
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
    fn wrap(&self, id: &'static str) -> bool {
        *self.wraps.get(id).unwrap_or(&(id == "actions"))
    }
    fn toolbar(
        &self,
        id: &'static str,
        source: &'static str,
        filename: &'static str,
    ) -> code_block::Actions<'static, Message> {
        code_block::actions![
            button(icon::wrap_text())
                .style(button::Ghost)
                .square()
                .pressed(self.wrap(id))
                .on_press(Message::Wrap(id)),
            button(icon::download())
                .style(button::Ghost)
                .square()
                .on_press(Message::Save(filename, source)),
            code_block::copy_button(source),
        ]
    }
    fn replay(&self, id: &'static str) -> View<'static, Message> {
        button![icon::rotate_ccw(), "Replay"]
            .style(button::Outline)
            .size(Size::Sm)
            .disabled(self.streaming.contains_key(id))
            .on_press(Message::Replay(id))
            .into()
    }
    fn streaming(&self) -> View<'static, Message> {
        let mut remaining = self
            .streaming
            .get("streaming")
            .and_then(|_| self.progress.get("streaming").copied())
            .unwrap_or(usize::MAX);
        let mut output = column![].width(Fill).spacing(16);
        for stream in STREAMS {
            if remaining == 0 {
                break;
            }
            let count = remaining.min(stream.code.len());
            let code = &stream.code[..stream.code.floor_char_boundary(count)];
            output = output.push(streaming_block(stream, code));
            remaining = remaining.saturating_sub(count);
        }
        output.push(self.replay("streaming")).into()
    }
    fn markdown(&self, dark: bool) -> View<'static, Message> {
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
            reply = reply.push(markdown_code_block(
                code.trim_end_matches('\n'),
                "rust",
                !fenced.contains("```"),
            ));
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
    fn example(&self, id: &str, dark: bool) -> View<'static, Message> {
        match id {
            "demo" => code_block(
                code_block::content(DEMO)
                    .language("rust")
                    .line_numbers(true)
                    .wrap(self.wrap("demo"))
                    .line_style(6..=10, code_block::Highlighted),
            )
            .header(
                code_block::header(code_block::title("src/view.rs").icon(icon::file_code()))
                    .actions(self.toolbar("demo", DEMO, "view.rs")),
            )
            .into(),
            "basic" => code_block(BASIC).into(),
            "header" => code_block(HEADER)
                .header(
                    code_block::header(code_block::title("Terminal").icon(icon::terminal()))
                        .actions(code_block::copy_button(COMMAND)),
                )
                .into(),
            "actions" => code_block(code_block::content(ACTIONS).wrap(self.wrap("actions")))
                .header(
                    code_block::header(code_block::title("Terminal").icon(icon::terminal()))
                        .actions(self.toolbar("actions", ACTIONS, "clippy.log")),
                )
                .into(),
            "floating-actions" => code_block(FLOATING_ACTIONS)
                .actions(code_block::copy_button(FLOATING_ACTIONS))
                .into(),
            "line-numbers" => {
                code_block(code_block::content(LINE_NUMBERS).line_numbers(true)).into()
            }
            "start-line" => code_block(
                code_block::content(START_LINE)
                    .language("rust")
                    .line_numbers(true)
                    .start_line(995),
            )
            .header(code_block::header("src/notebooks.rs"))
            .into(),
            "wrap" => column![
                code_block(WRAP),
                code_block(code_block::content(WRAP).wrap(true)),
            ]
            .width(Fill)
            .spacing(16)
            .into(),
            "highlighted-lines" => code_block(
                code_block::content(HIGHLIGHTED_LINES)
                    .line_numbers(true)
                    .line_style(3..=4, code_block::Highlighted),
            )
            .into(),
            "diff" => code_block(
                DIFF.lines()
                    .map(|row| code_block::line(&row[1..]).style(diff_style(row)))
                    .collect::<Vec<_>>(),
            )
            .header(code_block::header("src/search.rs").actions(code_block::copy_button(DIFF)))
            .into(),
            "height" => code_block(code_block::content(self.logs.clone()).max_height(192))
                .header(
                    code_block::header("Logs").actions(code_block::copy_button(self.logs.clone())),
                )
                .into(),
            "show-more" => code_block::show_more(256, || {
                code_block(
                    code_block::content(SHOW_MORE)
                        .language("rust")
                        .line_numbers(true),
                )
                .header(
                    code_block::header("src/notebook.rs")
                        .actions(code_block::copy_button(SHOW_MORE)),
                )
            })
            .into(),
            "show-more-scroll" => code_block::show_more(256, || {
                code_block(
                    code_block::content(SHOW_MORE)
                        .language("rust")
                        .line_numbers(true)
                        .max_height(384),
                )
                .header(
                    code_block::header("src/notebook.rs")
                        .actions(code_block::copy_button(SHOW_MORE)),
                )
            })
            .into(),
            "size" => column![terminal(SIZE), terminal(SIZE).size(Size::Sm),]
                .width(Fill)
                .spacing(16)
                .into(),
            "destructive" => code_block(code_block::content(DESTRUCTIVE).wrap(true))
                .header(code_block::header("Error").actions(code_block::copy_button(DESTRUCTIVE)))
                .style(Destructive)
                .into(),
            "trace" => trace([
                tool_output("Ran cargo --version", TRACE, false),
                tool_output("Failed to run cargo run --bin notes", TRACE_ERROR, true),
            ])
            .into(),
            "tabs" => tabs(|t| {
                let mut body = column![t.list([
                    t.trigger("view", "view.rs"),
                    t.trigger("search", "search.rs"),
                    t.trigger("config", "config.toml"),
                ])]
                .spacing(8);
                for file in FILES {
                    body = body.push(
                        t.content(
                            file.id,
                            code_block(
                                code_block::content(file.code)
                                    .language(file.language)
                                    .line_numbers(true),
                            )
                            .header(
                                code_block::header(file.title)
                                    .actions(code_block::copy_button(file.code)),
                            ),
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
                        code_block(code_block::content(CHAT).language("rust")).header(
                            code_block::header("src/search.rs")
                                .actions(code_block::copy_button(CHAT))
                        ),
                        text::p("The result borrows the matching titles from the original list."),
                    ]
                    .spacing(12)
                ),
            ]
            .spacing(16)
            .into(),
            "typeset" => column![
                text::h3("Read a notebook index"),
                prose(&self.typeset_before, dark),
                code_block(
                    code_block::content(TYPESET)
                        .language("rust")
                        .line_numbers(true)
                )
                .header(
                    code_block::header(code_block::title("src/index.rs").icon(icon::file_code()))
                        .actions(code_block::copy_button(TYPESET))
                ),
                prose(&self.typeset_after, dark),
            ]
            .spacing(12)
            .into(),
            "editable" => code_block(
                code_block::content("")
                    .default_value(EDITABLE)
                    .language("rust")
                    .line_numbers(true),
            )
            .header(code_block::header(
                code_block::title("src/greet.rs").icon(icon::file_code()),
            ))
            .into(),
            "highlighting" => code_block(code_block::content(HIGHLIGHTING).language("rust"))
                .header(code_block::header("src/notebook.rs"))
                .into(),
            "languages" => {
                let mut samples = column![].width(Fill).spacing(16);
                for sample in SAMPLES {
                    samples = samples.push(
                        code_block(code_block::content(sample.code).language(sample.language))
                            .header(code_block::header(sample.title))
                            .into(),
                    );
                }
                samples.into()
            }
            "ansi" => code_block(code_block::content(ANSI).language("ansi").wrap(true))
                .header(
                    code_block::header(code_block::title("Terminal").icon(icon::terminal()))
                        .actions(code_block::copy_button(ansi::strip(ANSI))),
                )
                .into(),
            "streaming" => self.streaming(),
            "markdown" => self.markdown(dark),
            _ => unreachable!("unknown Code Block example {id}"),
        }
    }
}

fn terminal(output: &'static str) -> code_block::CodeBlock<'static, Message> {
    code_block(code_block::content(output).wrap(true)).header(
        code_block::header(code_block::title("Terminal").icon(icon::terminal()))
            .actions(code_block::copy_button(output)),
    )
}

fn diff_style(row: &str) -> LineStyle {
    match row.as_bytes().first() {
        Some(b'+') => code_block::Added,
        Some(b'-') => code_block::Removed,
        _ => LineStyle::Default,
    }
}

fn tool_output(title: &'static str, output: &'static str, error: bool) -> View<'static, Message> {
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
            let block =
                code_block(code_block::content(output).wrap(true).max_height(256)).size(Size::Sm);
            if error {
                block.style(Destructive).into()
            } else {
                block.into()
            }
        },
    )
    .default_open(true)
    .into()
}

fn streaming_block(stream: &Stream, code: &str) -> View<'static, Message> {
    let rows: Vec<&str> = code.split('\n').collect();
    // A diff row starts with +, - or a space. The marker is not part of the code.
    let source = if stream.diff {
        rows.iter()
            .map(|row| row.get(1..).unwrap_or(""))
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        code.to_owned()
    };
    let mut content = code_block::content(source.clone())
        .language(stream.language)
        .line_numbers(stream.line_numbers);
    for (index, row) in rows.iter().enumerate() {
        let style = if stream.diff {
            diff_style(row)
        } else if stream.highlight.contains(&index) {
            code_block::Highlighted
        } else {
            LineStyle::Default
        };
        content = content.line_style(index + 1, style);
    }
    match stream.title {
        Some(title) => {
            code_block(content)
                .header(code_block::header(title).actions(
                    code_block::copy_button(source).disabled(code.len() < stream.code.len()),
                ))
                .into()
        }
        None => code_block(content).into(),
    }
}

fn markdown_code_block(
    code: &str,
    language: &'static str,
    incomplete: bool,
) -> View<'static, Message> {
    code_block(code_block::content(code.to_owned()).language(language))
        .header(
            code_block::header(language)
                .actions(code_block::copy_button(code.to_owned()).disabled(incomplete)),
        )
        .into()
}

fn total_length(id: &str) -> usize {
    if id == "markdown" {
        MARKDOWN.len()
    } else {
        STREAMS.iter().map(|stream| stream.code.len()).sum()
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

fn prose(items: &[markdown::Item], dark: bool) -> View<'static, Message> {
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
struct Demo {
    id: String,
    dark: bool,
    page: usize,
}
pub fn show(id: &str, dark: bool, page: usize) -> Element<'static, crate::Message, Theme> {
    component(Demo {
        id: id.to_owned(),
        dark,
        page,
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
    fn listen(
        &self,
        state: &State,
        event: &iced::Event,
        _: iced::Rectangle,
        _: mouse::Cursor,
    ) -> Action<Message> {
        if let iced::Event::Window(window::Event::RedrawRequested(now)) = event
            && let Some(deadline) = state.deadline()
        {
            return if *now >= deadline {
                Action::publish(Message::Tick(*now))
                    .and_request_redraw_at(*now + Duration::from_millis(16))
            } else {
                Action::request_redraw_at(deadline)
            };
        }
        Action::none()
    }
}
#[cfg(test)]
#[path = "tests.rs"]
mod tests;
