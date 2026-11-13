mod highlight;
mod samples;
mod sections;

use iced::{clipboard, time};
use std::collections::BTreeMap;
use std::time::{Duration, Instant};
use tokio::time::sleep;

use iced::widget::Id;
use iced::{Element, Fill, Font, Subscription, Task};
use icedxdy::theme::tweak::{Alpha, Bg, Fg, Token};
use icedxdy::widget::code_block::LineStyle;
use icedxdy::widget::{
    CodeBlock, button, code_block, collapsible, column, container, row, tabs, text, tooltip,
};
use icedxdy::{Size, Slot, Theme, theme};

use crate::{Message, font, icon, page};
use highlight::{STREAMS, listing};
use samples::*;
use sections::BLOCKS;

pub static META: page::Meta = page::Meta {
    title: "Code Block",
    description: "Displays code, terminal output and diffs with an optional header, line numbers and a copy button.",
};

#[derive(Clone, Debug)]
pub enum Event {
    Wrap(&'static str),
    Expand(&'static str),
    Copy(&'static str, String),
    Copied(&'static str, Instant),
    ResetCopy(&'static str, Instant),
    Replay(&'static str),
    Tick,
    Save(&'static str, &'static str),
    Saved(&'static str, Result<bool, String>),
}

pub struct CodeBlockPage {
    state: page::State,
    wraps: BTreeMap<&'static str, bool>,
    expanded: BTreeMap<&'static str, bool>,
    copied: BTreeMap<&'static str, Instant>,
    streaming: BTreeMap<&'static str, Instant>,
    save_status: BTreeMap<&'static str, String>,
    logs: String,
}

impl CodeBlockPage {
    pub fn new(index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(index, BLOCKS),
            wraps: BTreeMap::new(),
            expanded: BTreeMap::new(),
            copied: BTreeMap::new(),
            streaming: BTreeMap::new(),
            save_status: BTreeMap::new(),
            logs: (0..40)
                .map(|i| {
                    format!(
                        "[{:02}] GET /api/chat 200 in {}ms",
                        i + 1,
                        12 + (i * 7) % 40
                    )
                })
                .collect::<Vec<_>>()
                .join("\n"),
        })
    }
    fn event(&self, event: Event) -> Message {
        Message::Page(self.state.page_index, page::Message::CodeBlock(event))
    }
    fn update_demo(&mut self, event: Event) -> Task<Message> {
        let index = self.state.page_index;
        let send = move |event| Message::Page(index, page::Message::CodeBlock(event));
        match event {
            Event::Wrap(id) => {
                let wrap = self.wraps.entry(id).or_insert(id == "actions");
                *wrap = !*wrap;
            }
            Event::Expand(id) => {
                let expand = self.expanded.entry(id).or_default();
                *expand = !*expand;
            }
            Event::Copy(id, value) => {
                return clipboard::write(value).then(move |result| {
                    if result.is_ok() {
                        Task::done(send(Event::Copied(id, Instant::now())))
                    } else {
                        Task::none()
                    }
                });
            }
            Event::Copied(id, at) => {
                self.copied.insert(id, at);
                return Task::perform(sleep(Duration::from_secs(2)), move |_| {
                    send(Event::ResetCopy(id, at))
                });
            }
            Event::ResetCopy(id, at) => {
                if self.copied.get(id) == Some(&at) {
                    self.copied.remove(id);
                }
            }
            Event::Replay(id) => {
                self.streaming.insert(id, Instant::now());
            }
            Event::Tick => self
                .streaming
                .retain(|id, start| visible_bytes(*start) < total_length(id)),
            Event::Save(name, value) => {
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
                    move |result| send(Event::Saved(name, result)),
                );
            }
            Event::Saved(name, Ok(true)) => {
                self.save_status.insert(name, format!("Saved {name}"));
            }
            Event::Saved(_, Ok(false)) => {}
            Event::Saved(name, Err(error)) => {
                self.save_status
                    .insert(name, format!("Could not save {name}: {error}"));
            }
        }
        Task::none()
    }

    fn copy(&self, id: &'static str, source: &str) -> Slot<'static, Message> {
        let copied = self.copied.contains_key(id);
        let glyph = if copied { icon::check() } else { icon::copy() };
        action(
            glyph.size(14),
            if copied { "Copied" } else { "Copy" },
            self.event(Event::Copy(id, source.to_owned())),
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
            icon::wrap_text().size(14),
            "Wrap lines",
            self.event(Event::Wrap(id)),
        );
        let save = action(
            icon::download().size(14),
            "Download",
            self.event(Event::Save(filename, source)),
        );
        let mut block = block
            .title(row![icon::file_code().size(14), text::small(title)].spacing(6))
            .actions(row![wrap, save, self.copy(id, source)]);
        if let Some(status) = self.save_status.get(filename) {
            block = block.footer(text::muted(status.clone()));
        }
        block.into()
    }
    fn replay(&self, id: &'static str) -> Slot<'static, Message> {
        button!("Replay", icon::refresh_ccw().size(13))
            .style(button::Outline)
            .size(Size::Xs)
            .disabled(self.streaming.contains_key(id))
            .on_press(self.event(Event::Replay(id)))
            .into()
    }
    fn streaming(&self) -> Slot<'static, Message> {
        let mut remaining = self
            .streaming
            .get("streaming")
            .map(|s| visible_bytes(*s))
            .unwrap_or(usize::MAX);
        let mut output = column![].spacing(12);
        for &(title, _, source) in STREAMS {
            if remaining == 0 {
                break;
            }
            let count = remaining.min(source.len());
            let mut block = listing(source, count, true, |_, line| {
                if source == STREAMING_2 {
                    diff_style(line)
                } else {
                    LineStyle::Default
                }
            })
            .title(title);
            if !matches!(title, "Terminal" | ".env.local") {
                block = block.start_line(1);
            }
            output = output.push(block.into());
            remaining = remaining.saturating_sub(count);
        }
        output.push(self.replay("streaming")).into()
    }
    fn markdown(&self) -> Slot<'static, Message> {
        let end = self
            .streaming
            .get("markdown")
            .map(|s| visible_bytes(*s))
            .unwrap_or(usize::MAX)
            .min(MARKDOWN.len());
        let body = &MARKDOWN[..end];
        let (before, fenced) = body.split_once("```tsx\n").unwrap_or((body, ""));
        let mut items = column![
            text::label("How do I debounce a search input in React?"),
            text::p(before.trim().replace('`', ""))
        ]
        .spacing(12);
        if !fenced.is_empty() {
            let (code, after) = fenced.split_once("```").unwrap_or((fenced, ""));
            let mut block = code_block(code)
                .font(Font::new(font::GEIST_MONO_NAME))
                .title("tsx")
                .wrap(true);
            if !after.is_empty() {
                block = block.actions(self.copy("markdown", code));
            }
            items = items.push(block.into());
            if !after.trim().is_empty() {
                items = items.push(text::p(after.trim()).into());
            }
        }
        items.push(self.replay("markdown")).into()
    }
    fn example(&self, id: &str) -> Slot<'_, Message> {
        let normal = |_, _: &str| LineStyle::Default;
        match id {
            "demo" => {
                let wrap = *self.wraps.get("demo").unwrap_or(&false);
                let block = listing(DEMO, usize::MAX, wrap, |i, _| {
                    if (5..=9).contains(&i) {
                        LineStyle::Highlighted
                    } else {
                        LineStyle::Default
                    }
                })
                .start_line(1);
                self.header("demo", "app/api/chat/route.ts", DEMO, "route.ts", block)
            },
            "basic" => terminal(BASIC).into(),
            "header" => {
                terminal(HEADER)
                    .title("Terminal")
                    .actions(self.copy("header", "pnpm add ai @ai-sdk/react zod"))
                    .into()
            },
            "actions" => {
                let wrap = *self.wraps.get("actions").unwrap_or(&true);
                self.header(
                    "actions",
                    "Terminal",
                    ACTIONS,
                    "lint.log",
                    terminal(ACTIONS).wrap(wrap),
                )
            },
            "floating-actions" => {
                terminal(FLOATING_ACTIONS)
                    .actions(self.copy("floating-actions", FLOATING_ACTIONS))
                    .into()
            },
            "line-numbers" => terminal(LINE_NUMBERS).start_line(1).into(),
            "start-line" => {
                listing(START_LINE, usize::MAX, false, normal)
                    .title("app/dashboard/page.tsx")
                    .start_line(995)
                    .into()
            },
            "wrap" => {
                column![terminal(WRAP), terminal(WRAP).wrap(true)]
                    .spacing(16)
                    .into()
            },
            "highlighted-lines" => {
                listing(HIGHLIGHTED_LINES, usize::MAX, false, |i, _| {
                    if [2, 3].contains(&i) {
                        LineStyle::Highlighted
                    } else {
                        LineStyle::Default
                    }
                })
                .start_line(1)
                .into()
            },
            "diff" => {
                listing(DIFF, usize::MAX, false, |_, line| diff_style(line))
                    .title("components/greeting.tsx")
                    .actions(self.copy("diff", DIFF))
                    .into()
            },
            "height" => {
                terminal(&self.logs)
                    .height(160.)
                    .title("Logs")
                    .actions(self.copy("height", &self.logs))
                    .into()
            },
            "show-more" | "show-more-scroll" => {
                let id = if id == "show-more" {
                    "show-more"
                } else {
                    "show-more-scroll"
                };
                let expanded = *self.expanded.get(id).unwrap_or(&false);
                let end = if expanded {
                    SHOW_MORE.len()
                } else {
                    SHOW_MORE
                        .match_indices('\n')
                        .nth(9)
                        .map_or(SHOW_MORE.len(), |(i, _)| i)
                };
                let mut block = listing(SHOW_MORE, end, true, normal)
                    .title("components/chat.tsx")
                    .start_line(1)
                    .actions(self.copy(id, SHOW_MORE))
                    .footer(
                        container(
                            button(if expanded { "Show less" } else { "Show more" })
                                .size(Size::Xs)
                                .style(button::Ghost)
                                .on_press(self.event(Event::Expand(id))),
                        )
                        .center_x(Fill),
                    );
                if expanded && id == "show-more-scroll" {
                    block = block.height(320.);
                }
                block.into()
            },
            "size" => {
                column![
                    terminal(SIZE)
                        .title("Terminal")
                        .actions(self.copy("size-normal", SIZE)),
                    terminal(SIZE)
                        .title("Terminal")
                        .compact(true)
                        .actions(self.copy("size-small", SIZE))
                ]
                .spacing(16)
                .into()
            },
            "destructive" => {
                error_block(DESTRUCTIVE)
                    .title("Error")
                    .actions(self.copy("destructive", DESTRUCTIVE))
                    .into()
            },
            "trace" => {
                column![
                    trace("Ran node --version", TRACE, false),
                    trace("Failed to run pnpm dev", TRACE_ERROR, true)
                ]
                .spacing(6)
                .into()
            },
            "tabs" => {
                tabs(move |t| {
                    let mut body = column![t.list([
                        t.trigger("route", "route.ts"),
                        t.trigger("chat", "chat.tsx"),
                        t.trigger("env", ".env.local")
                    ])]
                    .spacing(8);
                    for (id, title, source) in [
                        ("route", "app/api/chat/route.ts", TABS_0),
                        ("chat", "components/chat.tsx", TABS_1),
                        ("env", ".env.local", TABS_2),
                    ] {
                        body = body.push(
                            t.content(
                                id,
                                listing(source, usize::MAX, false, normal)
                                    .start_line(1)
                                    .title(title)
                                    .actions(self.copy(id, source)),
                            ),
                        );
                    }
                    body.into()
                })
                .default_value("route")
                .into()
            },
            "chat" => {
                column![
                    text::label("How do I stream a response?"),
                    text::p("Create a route handler that returns the stream:"),
                    listing(CHAT, usize::MAX, false, normal)
                        .title("app/api/chat/route.ts")
                        .actions(self.copy("chat-sample", CHAT)),
                    text::p("The client reads it with useChat.")
                ]
                .spacing(12)
                .into()
            },
            "typeset" => {
                column![
                text::h3("Stream a response"),
                text::p("Create a route handler that calls streamText and returns the stream to the client."),
                listing(TYPESET,usize::MAX,false,normal).start_line(1).title("app/api/chat/route.ts").actions(self.copy("typeset",TYPESET)),
                text::p("The useChat hook reads the stream on the client and renders the answer while it is written.")
                ].spacing(12).into()
            },
            "highlighting" => {
                listing(HIGHLIGHTING, usize::MAX, true, normal)
                    .title("getWeather output")
                    .into()
            },
            "languages" => {
                let mut samples = column![].spacing(12);
                for (title, source) in [
                    ("app/globals.css", LANGUAGES_0),
                    ("main.py", LANGUAGES_1),
                    ("notes.txt", LANGUAGES_2),
                ] {
                    samples = samples.push(
                        listing(source, usize::MAX, true, normal)
                            .title(title)
                            .into(),
                    );
                }
                samples.into()
            },
            "ansi" => {
                let (listing, plain) = highlight::ansi();
                listing
                    .title("Terminal")
                    .actions(self.copy("ansi", &plain))
                    .into()
            },
            "streaming" => self.streaming(),
            "markdown" => self.markdown(),
            _ => unreachable!("unknown Code Block example {id}"),
        }
    }
}

fn terminal(source: &str) -> CodeBlock<'_, Message> {
    code_block(source).font(Font::new(font::GEIST_MONO_NAME))
}
fn error_block(source: &str) -> CodeBlock<'_, Message> {
    terminal(source).wrap(true).style(
        theme::container::CodeBlock + Bg(Token::Destructive) + Alpha(0.10) + Fg(Token::Destructive),
    )
}
fn trace(title: &'static str, source: &'static str, error: bool) -> Slot<'static, Message> {
    collapsible(move |c| {
        let code = if error {
            error_block(source)
        } else {
            terminal(source).wrap(true)
        };
        column![
            c.trigger(
                button![
                    icon::terminal().size(14),
                    title,
                    icon::chevron_down().size(12)
                ]
                .style(button::Ghost)
            ),
            c.content(code.compact(true))
        ]
        .into()
    })
    .into()
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
fn diff_style(line: &str) -> LineStyle {
    if line.starts_with('+') {
        LineStyle::Added
    } else if line.starts_with('-') {
        LineStyle::Removed
    } else {
        LineStyle::Default
    }
}
fn visible_bytes(start: Instant) -> usize {
    (start.elapsed().as_millis() as usize / 16) * 3
}
fn total_length(id: &str) -> usize {
    if id == "markdown" {
        MARKDOWN.len()
    } else {
        STREAMS.iter().map(|(_, _, s)| s.len()).sum()
    }
}

impl page::Page for CodeBlockPage {
    fn meta(&self) -> &'static page::Meta {
        &META
    }
    fn blocks(&self) -> &'static [page::Block] {
        BLOCKS
    }
    fn page_index(&self) -> usize {
        self.state.page_index
    }
    fn state(&self) -> &page::State {
        &self.state
    }
    fn scrollable_id(&self) -> Id {
        self.state.scrollable_id.clone()
    }
    fn init(&self) -> Task<Message> {
        self.state.init()
    }
    fn update(&mut self, event: page::Message) -> Task<Message> {
        match event {
            page::Message::CodeBlock(event) => self.update_demo(event),
            other => self.state.update(&other),
        }
    }
    fn reset_volatile(&mut self) {
        self.state.reset_volatile();
        self.streaming.clear();
    }
    fn subscription(&self) -> Subscription<page::Message> {
        if self.streaming.is_empty() {
            Subscription::none()
        } else {
            time::every(Duration::from_millis(32)).map(|_| page::Message::CodeBlock(Event::Tick))
        }
    }
    fn preview(&self, id: &str) -> Element<'_, Message, Theme> {
        container(self.example(id))
            .max_width(if id == "demo" { 512 } else { 448 })
            .width(Fill)
            .into()
    }
}
