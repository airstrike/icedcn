// The upstream RTL recipe is excluded by project policy; all 24 LTR recipes remain.
mod highlight;
mod samples;
mod sections;
#[cfg(test)]
mod tests;

use iced::{clipboard, time};
use std::collections::BTreeMap;
use std::time::{Duration, Instant};
use tokio::time::sleep;

use iced::alignment::Horizontal;
use iced::widget::{Id, svg};
use iced::{Element, Fill, Font, Subscription, Task, Widget};
use icedxdy::core::Border;
use icedxdy::widget::code_block::LineStyle;
use icedxdy::widget::{
    CodeBlock, bubble, button, code_block, column, container, markdown, message, row, tabs, text,
    tooltip, trace,
};
use icedxdy::{Size, Slot, Theme, radius, theme};

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
    markdown_before: Vec<markdown::Item>,
    markdown_after: Vec<markdown::Item>,
    typeset_before: Vec<markdown::Item>,
    typeset_after: Vec<markdown::Item>,
}

impl CodeBlockPage {
    pub fn new(index: usize) -> Box<dyn page::Page> {
        Box::new(Self::at(index))
    }
    fn at(index: usize) -> Self {
        Self {
            state: page::State::new(index, BLOCKS),
            wraps: BTreeMap::new(),
            expanded: BTreeMap::new(),
            copied: BTreeMap::new(),
            streaming: BTreeMap::new(),
            save_status: BTreeMap::new(),
            typeset_before: markdown::parse("Create a route handler that calls `streamText` and returns the stream to the client.").collect(),
            typeset_after: markdown::parse("The `useChat` hook reads the stream on the client and renders the answer while it is written.").collect(),
            markdown_before: markdown_parts(usize::MAX).0,
            markdown_after: markdown_parts(usize::MAX).1,
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
        }
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
            Event::Tick => self.streaming.retain(|id, start| {
                (if *id == "markdown" {
                    markdown_bytes(*start)
                } else {
                    visible_bytes(*start)
                }) < total_length(id)
            }),
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
        let end = self
            .streaming
            .get("markdown")
            .map(|start| markdown_bytes(*start))
            .unwrap_or(usize::MAX);
        (self.markdown_before, self.markdown_after) = markdown_parts(end);
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
            .title(
                row![
                    if id == "actions" {
                        icon::terminal().size(14)
                    } else {
                        icon::file_code().size(14)
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
    fn replay(&self, id: &'static str) -> Slot<'static, Message> {
        button![replay_icon(), "Replay"]
            .style(button::Outline)
            .size(Size::Sm)
            .disabled(self.streaming.contains_key(id))
            .on_press(self.event(Event::Replay(id)))
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
                    button(if copied {
                        icon::check().size(14)
                    } else {
                        icon::copy().size(14)
                    })
                    .square()
                    .size(Size::Xs)
                    .style(button::Ghost)
                    .disabled(count < source.len())
                    .on_press(self.event(Event::Copy(
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
    fn markdown(&self, dark: bool) -> Slot<'_, Message> {
        let end = self
            .streaming
            .get("markdown")
            .map(|s| markdown_bytes(*s))
            .unwrap_or(usize::MAX)
            .min(MARKDOWN.len());
        let body = &MARKDOWN[..end];
        let (_, fenced) = body.split_once("```tsx\n").unwrap_or((body, ""));
        let noop = self.event(Event::Tick);
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
                    button(if copied {
                        icon::check().size(14)
                    } else {
                        icon::copy().size(14)
                    })
                    .square()
                    .size(Size::Xs)
                    .style(button::Ghost)
                    .disabled(!complete)
                    .on_press(self.event(Event::Copy("markdown", code.to_owned()))),
                    if copied { "Copied" } else { "Copy" },
                ))
                .into(),
            );
        }
        let noop = self.event(Event::Tick);
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
                    .on_press(self.event(Event::Replay("markdown")))
                    .into(),
            );
        }
        container(items)
            .padding([24, 0])
            .min_height(448)
            .width(Fill)
            .into()
    }
    fn example(&self, id: &str, dark: bool) -> Slot<'_, Message> {
        let normal = |_, _: &str| LineStyle::Default;
        match id {
            "demo" => {
                let wrap = *self.wraps.get("demo").unwrap_or(&false);
                let block = listing(dark, DEMO, usize::MAX, wrap, |i, _| {
                    if (5..=9).contains(&i) {
                        LineStyle::Highlighted
                    } else {
                        LineStyle::Default
                    }
                })
                .start_line(1);
                self.header("demo", "app/api/chat/route.ts", DEMO, "route.ts", block)
            }
            "basic" => terminal(BASIC).into(),
            "header" => terminal(HEADER)
                .title(row![icon::terminal().size(14), code_block::title("Terminal")].spacing(6))
                .actions(self.copy("header", "pnpm add ai @ai-sdk/react zod"))
                .into(),
            "actions" => {
                let wrap = *self.wraps.get("actions").unwrap_or(&true);
                self.header(
                    "actions",
                    "Terminal",
                    ACTIONS,
                    "lint.log",
                    terminal(ACTIONS).wrap(wrap),
                )
            }
            "floating-actions" => terminal(FLOATING_ACTIONS)
                .actions(
                    container(self.copy("floating-actions", FLOATING_ACTIONS)).style(
                        |theme: &Theme| theme::container::Style {
                            background: Some(theme.neutral.base.into()),
                            border: Border {
                                width: 1.,
                                color: theme.neutral.foreground.alpha(0.1),
                                radius: radius::SM.into(),
                            },
                            ..Default::default()
                        },
                    ),
                )
                .into(),
            "line-numbers" => terminal(LINE_NUMBERS).start_line(1).into(),
            "start-line" => listing(dark, START_LINE, usize::MAX, false, normal)
                .title(code_block::title("app/dashboard/page.tsx"))
                .start_line(995)
                .into(),
            "wrap" => column![terminal(WRAP), terminal(WRAP).wrap(true)]
                .spacing(16)
                .into(),
            "highlighted-lines" => plain_lines(HIGHLIGHTED_LINES, |i, _| {
                if [2, 3].contains(&i) {
                    LineStyle::Highlighted
                } else {
                    LineStyle::Default
                }
            })
            .start_line(1)
            .into(),
            "diff" => listing(dark, DIFF, usize::MAX, false, |_, line| diff_style(line))
                .title(code_block::title("components/greeting.tsx"))
                .actions(self.copy("diff", DIFF))
                .into(),
            "height" => terminal(&self.logs)
                .max_height(192.)
                .title("Logs")
                .actions(self.copy("height", &self.logs))
                .into(),
            "show-more" | "show-more-scroll" => {
                let id = if id == "show-more" {
                    "show-more"
                } else {
                    "show-more-scroll"
                };
                let expanded = *self.expanded.get(id).unwrap_or(&false);
                let mut block = listing(dark, SHOW_MORE, usize::MAX, false, normal)
                    .title(code_block::title("components/chat.tsx"))
                    .start_line(1)
                    .actions(self.copy(id, SHOW_MORE))
                    .footer(
                        container(
                            button(
                                row![
                                    if expanded {
                                        icon::chevron_up().size(14)
                                    } else {
                                        icon::chevron_down().size(14)
                                    },
                                    if expanded { "Show less" } else { "Show more" }
                                ]
                                .spacing(4),
                            )
                            .size(Size::Xs)
                            .style(button::Ghost)
                            .on_press(self.event(Event::Expand(id))),
                        )
                        .center_x(Fill),
                    );
                if !expanded {
                    block = block.max_height(256.).collapsed(true);
                } else if id == "show-more-scroll" {
                    block = block.max_height(384.);
                }
                block.into()
            }
            "size" => column![
                terminal(SIZE)
                    .wrap(true)
                    .title(
                        row![icon::terminal().size(14), code_block::title("Terminal")].spacing(6)
                    )
                    .actions(self.copy("size-normal", SIZE)),
                terminal(SIZE)
                    .wrap(true)
                    .title(
                        row![icon::terminal().size(14), code_block::title("Terminal")].spacing(6)
                    )
                    .compact(true)
                    .actions(self.copy("size-small", SIZE))
            ]
            .spacing(16)
            .into(),
            "destructive" => error_block(DESTRUCTIVE)
                .title("Error")
                .actions(self.copy("destructive", DESTRUCTIVE))
                .into(),
            "trace" => column![
                trace_output("Ran node --version", TRACE, false),
                trace_output("Failed to run pnpm dev", TRACE_ERROR, true)
            ]
            .spacing(8)
            .into(),
            "tabs" => tabs(move |t| {
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
                            listing(dark, source, usize::MAX, false, normal)
                                .start_line(1)
                                .title(code_block::title(title))
                                .actions(self.copy(id, source)),
                        ),
                    );
                }
                body.into()
            })
            .default_value("route")
            .into(),
            "chat" => column![
                message(bubble("How do I stream a response?").style(bubble::Variant::Muted))
                    .align(Horizontal::Right),
                message(
                    column![
                        text::p("Create a route handler that returns the stream:"),
                        listing(dark, CHAT, usize::MAX, false, normal)
                            .title("app/api/chat/route.ts")
                            .actions(self.copy("chat-sample", CHAT)),
                        text::p("The client reads it with useChat.")
                    ]
                    .spacing(12)
                )
            ]
            .spacing(16)
            .into(),
            "typeset" => column![
                text::h3("Stream a response"),
                prose(&self.typeset_before, dark, self.event(Event::Tick)),
                listing(dark, TYPESET, usize::MAX, false, normal)
                    .start_line(1)
                    .title(
                        row![
                            icon::file_code().size(14),
                            code_block::title("app/api/chat/route.ts")
                        ]
                        .spacing(6)
                    )
                    .actions(self.copy("typeset", TYPESET)),
                prose(&self.typeset_after, dark, self.event(Event::Tick))
            ]
            .spacing(12)
            .into(),
            "highlighting" => listing(dark, HIGHLIGHTING, usize::MAX, false, normal)
                .title("getWeather output")
                .into(),
            "languages" => {
                let mut samples = column![].spacing(16);
                for (title, source) in [
                    ("app/globals.css", LANGUAGES_0),
                    ("main.py", LANGUAGES_1),
                    ("notes.txt", LANGUAGES_2),
                ] {
                    samples = samples.push(
                        listing(dark, source, usize::MAX, false, normal)
                            .title(code_block::title(title))
                            .into(),
                    );
                }
                samples.into()
            }
            "ansi" => {
                let (listing, plain) = highlight::ansi(dark);
                listing
                    .title(
                        row![icon::terminal().size(14), code_block::title("Terminal")].spacing(6),
                    )
                    .actions(self.copy("ansi", &plain))
                    .into()
            }
            "streaming" => self.streaming(dark),
            "markdown" => self.markdown(dark),
            _ => unreachable!("unknown Code Block example {id}"),
        }
    }
}

fn terminal(source: &str) -> CodeBlock<'_, Message> {
    code_block(source).font(Font::new(font::GEIST_MONO_NAME))
}
fn error_block(source: &str) -> CodeBlock<'_, Message> {
    terminal(source).wrap(true).style(code_block::Destructive)
}
fn plain_lines(
    source: &'static str,
    style: impl Fn(usize, &str) -> LineStyle,
) -> CodeBlock<'static, Message> {
    CodeBlock::lines(source.split('\n').enumerate().map(|(i, line)| {
        (
            style(i, line),
            iced_widget::text(line)
                .selectable(true)
                .font(Font::new(font::GEIST_MONO_NAME))
                .size(14)
                .line_height(22.75)
                .into(),
        )
    }))
    .font(Font::new(font::GEIST_MONO_NAME))
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
            time::every(Duration::from_millis(
                if self.streaming.contains_key("streaming") {
                    16
                } else {
                    30
                },
            ))
            .map(|_| page::Message::CodeBlock(Event::Tick))
        }
    }
    fn backdrop(&self) -> page::Backdrop {
        page::Backdrop {
            min_height: 0.,
            padding: 40.into(),
            ..Default::default()
        }
    }
    fn docs_preview(&self, id: &str, theme: &Theme) -> Element<'_, Message, Theme> {
        container(self.example(id, theme.neutral.is_dark))
            .width(Fill)
            .max_width(if id == "demo" { 512 } else { 448 })
            .into()
    }
    fn preview(&self, id: &str) -> Element<'_, Message, Theme> {
        container(self.example(id, false))
            .width(Fill)
            .max_width(if id == "demo" { 512 } else { 448 })
            .into()
    }
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

fn prose(items: &[markdown::Item], dark: bool, noop: Message) -> Slot<'_, Message> {
    markdown::view(items, prose_settings(dark))
        .map(move |_| noop.clone())
        .boxed()
        .into()
}
fn prose_settings(dark: bool) -> markdown::Settings {
    let theme = if dark { Theme::dark() } else { Theme::light() };
    markdown::Settings::with_text_size(14, markdown::Style::from(theme.iced_theme()))
}

fn replay_icon() -> Slot<'static, Message> {
    svg(svg::Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"/><path d="M3 3v5h5"/></svg>"#))
        .width(14).height(14).style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) }).boxed().into()
}

fn markdown_bytes(start: Instant) -> usize {
    markdown_prefix(start.elapsed().as_millis() as usize / 30)
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
