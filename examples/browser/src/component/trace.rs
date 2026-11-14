//! Trace rows and their step, source, and transcript compositions.
mod replay;
use crate::{Message, icon, page};
use iced::widget::{
    Text,
    component::{Component, component},
};
use iced::{Fill, Task, Widget};
use icedxdy::theme::container::CodeBlock;
use icedxdy::theme::tweak::{Alpha, Bg, Fg, Token};
use icedxdy::widget::{
    badge, button, code_block, column, container, dialog, row, sheet, spinner, text, trace,
};
use icedxdy::{Slot, Theme};
use trace::{Line, Status};

type Glyph = fn() -> Text<'static, Theme>;

fn line<M: 'static>(title: &'static str, glyph: Glyph) -> Line<'static, M> {
    trace::line(title).icon(glyph().size(16))
}
fn output<M: 'static>(value: &'static str) -> Slot<'static, M> {
    code_block(value).compact(true).wrap(true).into()
}
fn command_output<M: 'static>(value: &'static str, error: bool) -> Slot<'static, M> {
    if error {
        code_block(value)
            .compact(true)
            .wrap(true)
            .style(CodeBlock + Bg(Token::Destructive) + Alpha(0.10) + Fg(Token::Destructive))
            .into()
    } else {
        output(value)
    }
}
fn result<M: Clone + 'static>(
    title: &'static str,
    glyph: Glyph,
    body: &'static str,
    status: Status,
    open: bool,
) -> Slot<'static, M> {
    trace::disclosure(
        move || line(title, glyph).status(status),
        move || command_output(body, status == Status::Error),
    )
    .default_open(open)
    .into()
}
fn source(title: &'static str, href: &'static str) -> Slot<'static, Message> {
    trace::action(
        line(title, icon::globe).trailing(icon::arrow_up_right().size(16)),
        Some(Message::OpenUrl(href)),
    )
}
fn local(title: &'static str, slug: &str, index: usize) -> Slot<'static, Message> {
    let block = BLOCKS
        .iter()
        .flat_map(|b| {
            std::iter::once(b).chain(match b {
                page::Block::Group { children, .. } => children.iter(),
                _ => [].iter(),
            })
        })
        .position(|b| matches!(b,page::Block::Example{id,..} if *id==slug))
        .unwrap_or(0);
    trace::action(
        line(title, icon::file_text).trailing(icon::chevron_right().size(16)),
        Some(Message::Page(index, page::Message::ScrollTo(block))),
    )
}
fn static_line() -> Slot<'static, Message> {
    line("Created branch fix/cargo-lockfile", icon::git_branch).into()
}
fn status() -> Slot<'static, Message> {
    trace([
        line("Ran cargo fetch", icon::circle_check).into(),
        line("Failed to run cargo clippy", icon::circle_alert)
            .status(Status::Error)
            .into(),
        trace::line("Running cargo build")
            .icon(spinner().size(16.))
            .status(Status::Running)
            .into(),
        line("Run cargo test", icon::circle)
            .status(Status::Pending)
            .into(),
    ])
    .into()
}
fn description() -> Slot<'static, Message> {
    trace([
        line(
            "Edited src/settings/notifications.rs",
            icon::file_pen,
        )
        .description("+48 −12")
        .into(),
        line("Ran cargo test", icon::terminal)
            .description("14s")
            .into(),
        line("Searched for ChatState", icon::search)
            .description("8 results")
            .into(),
    ])
    .into()
}
fn collapsible() -> Slot<'static, Message> {
    result(
        "Ran rustc --version",
        icon::terminal,
        "$ rustc --version\nrustc 1.89.0",
        Status::Complete,
        false,
    )
}
struct OpenState;
impl<'a> Component<'a, Message, Theme> for OpenState {
    type State = bool;
    type Event = bool;
    fn update(&self, state: &mut bool, next: bool, _: &iced::Renderer) -> Option<Message> {
        *state = next;
        None
    }
    fn view(&self, state: &bool) -> impl Widget<bool, Theme> + 'a {
        let open = *state;
        Slot::from(
            column![
                result(
                    "Ran rustc --version",
                    icon::terminal,
                    "$ rustc --version\nrustc 1.89.0",
                    Status::Complete,
                    true
                ),
                trace::disclosure(
                    || line("Ran cargo --version", icon::terminal),
                    || output("$ cargo --version\ncargo 1.89.0")
                )
                .open(open)
                .on_open_change(|next| next),
                button(if open {
                    "Hide the Cargo output"
                } else {
                    "Show the Cargo output"
                })
                .size(icedxdy::Size::Sm)
                .style(button::Outline)
                .on_press(!open)
            ]
            .spacing(8)
            .width(Fill),
        )
        .compose()
    }
}
fn links(index: usize) -> Slot<'static, Message> {
    trace([
        local("Read examples/browser/src/page.rs", "link", index),
        source(
            "Getting Started | rust-lang.org",
            "https://www.rust-lang.org/learn/get-started",
        ),
    ])
    .into()
}
fn search_dialog() -> Slot<'static, Message> {
    dialog(
        |d| {
            d.trigger(trace::action(
                line("Looked up Cargo build caching", icon::globe)
                    .trailing(icon::chevron_right().size(16)),
                None,
            ))
        },
        |_| {
            column![
                dialog::header(
                    "Looked up Cargo build caching",
                    "Three results were used in the response."
                ),
                trace([
                    source(
                        "Caching build outputs",
                        "https://doc.rust-lang.org/cargo/reference/build-cache.html"
                    ),
                    source(
                        "Build profiles",
                        "https://doc.rust-lang.org/cargo/reference/profiles.html"
                    ),
                    source(
                        "Build scripts",
                        "https://doc.rust-lang.org/cargo/reference/build-scripts.html"
                    )
                ])
            ]
            .spacing(16)
            .into()
        },
    )
    .into()
}
fn files_sheet() -> Slot<'static, Message> {
    sheet(
        |s| {
            s.trigger(trace::action(
                line("Explored 4 files", icon::folder_search)
                    .trailing(icon::chevron_right().size(16)),
                None,
            ))
        },
        |_| {
            column![
                sheet::header(
                    "Explored 4 files",
                    "Files read while preparing the response."
                ),
                column![
                    text("examples/browser/src/page.rs"),
                    text("examples/browser/src/component.rs"),
                    text("examples/browser/src/main.rs"),
                    text("Cargo.toml")
                ]
                .spacing(8)
                .padding([0, 16])
            ]
            .into()
        },
    )
    .into()
}
fn group() -> Slot<'static, Message> {
    trace([
        line("Searched for rust-version", icon::search).into(),
        line("Read Cargo.toml", icon::file_text).into(),
        line("Ran cargo fetch", icon::terminal).into(),
    ])
    .into()
}
fn sources() -> Slot<'static, Message> {
    trace::disclosure(
        || line("Used 3 sources", icon::globe),
        || {
            trace([
                source(
                    "Getting Started | rust-lang.org",
                    "https://www.rust-lang.org/learn/get-started",
                ),
                source(
                    "Creating a package | The Cargo Book",
                    "https://doc.rust-lang.org/cargo/guide/creating-a-new-project.html",
                ),
                source(
                    "Configuration | The Cargo Book",
                    "https://doc.rust-lang.org/cargo/reference/config.html",
                ),
            ])
            .into()
        },
    )
    .default_open(true)
    .into()
}
fn steps(index: usize) -> Slot<'static, Message> {
    trace::disclosure(
        || trace::line("Looking into the slow Cargo build").status(Status::Running),
        move || {
            trace([
                trace::disclosure(
                    || line("Searched for Cargo build timings", icon::search),
                    || {
                        row![
                            badge("rust-lang.org").style(badge::Secondary),
                            badge("github.com").style(badge::Secondary)
                        ]
                        .spacing(6)
                        .into()
                    },
                )
                .into(),
                local("Read the last twelve build reports", "steps", index),
                result(
                    "Checking the release profile",
                    icon::search,
                    "cargo build --release --timings",
                    Status::Running,
                    false,
                ),
            ])
            .into()
        },
    )
    .default_open(true)
    .into()
}
fn file_diff(
    path: &'static str,
    title: &'static str,
    stat: &'static str,
    desc: &'static str,
    src: &'static str,
    glyph: Glyph,
) -> Slot<'static, Message> {
    use code_block::{CodeBlock, LineStyle};
    sheet(
        move |s| {
            s.trigger(trace::action(
                line(title, glyph)
                    .description(stat)
                    .trailing(icon::chevron_right().size(16)),
                None,
            ))
        },
        move |_| {
            let diff = CodeBlock::lines(src.lines().map(|line| {
                let (kind, value) = if let Some(add) = line.strip_prefix('+') {
                    (LineStyle::Added, add)
                } else if let Some(del) = line.strip_prefix('-') {
                    (LineStyle::Removed, del)
                } else {
                    (LineStyle::Default, line)
                };
                (kind, text(value).size(12).into())
            }));
            column![sheet::header(path, desc), container(diff).padding([0, 24])].into()
        },
    )
    .into()
}
fn commit() -> Slot<'static, Message> {
    trace::disclosure(
        || {
            line(
                "Committed a1b2c3: Fix the login redirect",
                icon::git_commit_horizontal,
            )
        },
        || {
            trace([
                file_diff(
                    "src/login.rs",
                    "Edited src/login.rs",
                    "+2 −2",
                    "2 additions, 2 deletions",
                    LOGIN_DIFF,
                    icon::file_pen,
                ),
                file_diff(
                    "src/auth/redirect.rs",
                    "Added src/auth/redirect.rs",
                    "+7 −0",
                    "7 additions, 0 deletions",
                    AUTH_DIFF,
                    icon::file_plus,
                ),
            ])
            .into()
        },
    )
    .default_open(true)
    .into()
}
fn turn(index: usize) -> Slot<'static, Message> {
    trace::disclosure(||trace::line("Worked for 2m 4s, 1 command failed").status(Status::Error),move||column![
        trace([
            trace::disclosure(||line("Thought",icon::brain).description("12s"),||text("I should check which Rust tools are available before creating a project.").into()).into(),
            trace::disclosure(||line("Ran 2 commands, 1 failed",icon::terminal).status(Status::Error),||trace([
                line("Ran rustc --version",icon::terminal).into(),result("Failed to run cargo --version",icon::terminal,"command not found: cargo",Status::Error,false)
            ]).into()).default_open(true).into()
        ]),
        text("Found rustc. Cargo is missing from PATH. Reading the project documentation."),
        local("Read examples/browser/src/page.rs","turn",index)
    ].spacing(8).into()).default_open(true).into()
}

const LOGIN_DIFF: &str = r#"@@ -14,2 +14,2 @@ fn finish_login(requested: Option<&str>) -> Route {
-    let destination = requested.unwrap_or("/home");
-    Route::from_path(destination)
+    let destination = redirect::after_login(requested);
+    Route::from_path(&destination)
"#;
const AUTH_DIFF: &str = r#"@@ -0,0 +1,7 @@
+pub fn after_login(requested: Option<&str>) -> String {
+    match requested {
+        Some("/settings") => String::from("/settings"),
+        Some("/inbox") => String::from("/inbox"),
+        _ => String::from("/home"),
+    }
+}"#;

pub static META: page::Meta = page::Meta {
    title: "Trace",
    description: "The building block for reasoning, tool calls, sources and more.",
};
pub static BLOCKS: &[page::Block] = &[
    page::Block::Example {
        id: "demo",
        title: "",
        description: None,
        snippet: "examples/browser/snippets/trace-demo.md",
    },
    page::Block::Md("examples/browser/snippets/trace-built-with.md"),
    page::Block::Md("examples/browser/snippets/trace-composition.md"),
    page::Block::Md("examples/browser/snippets/trace-features.md"),
    page::Block::Group {
        title: "Examples",
        children: &[
            page::Block::Example {
                id: "static",
                title: "Trace Line",
                description: Some(
                    "Use `trace::line(title)` on its own for a step with no detail to show.",
                ),
                snippet: "examples/browser/snippets/trace-static.md",
            },
            page::Block::Example {
                id: "status",
                title: "Status",
                description: Some(
                    "Use `.status(...)` on a trace line when a step is running, has failed or has yet to start.",
                ),
                snippet: "examples/browser/snippets/trace-status.md",
            },
            page::Block::Example {
                id: "description",
                title: "Description",
                description: Some(
                    "Use `.description(...)` to show a diff stat, duration or count after the line title.",
                ),
                snippet: "examples/browser/snippets/trace-description.md",
            },
            page::Block::Example {
                id: "collapsible",
                title: "Collapsible",
                description: Some(
                    "Pass the summary line and detail content to `trace::disclosure` as two closures. Pressing the line expands or collapses the detail.",
                ),
                snippet: "examples/browser/snippets/trace-collapsible.md",
            },
            page::Block::Example {
                id: "open",
                title: "Open State",
                description: Some(
                    "Rows start closed. Use `.default_open(true)` to start open, or `.open(open).on_open_change(Message::Toggle)` to control the row from `update`.",
                ),
                snippet: "examples/browser/snippets/trace-open.md",
            },
            page::Block::Example {
                id: "link",
                title: "Link",
                description: Some(
                    "Wrap the line in `trace::action` and pass a navigation message. The external link uses an arrow-up-right icon.",
                ),
                snippet: "examples/browser/snippets/trace-link.md",
            },
            page::Block::Example {
                id: "dialog",
                title: "Dialog",
                description: Some(
                    "Wrap the trace action in the dialog’s `trigger`. The dialog handles its open state, Escape and focus return.",
                ),
                snippet: "examples/browser/snippets/trace-dialog.md",
            },
            page::Block::Example {
                id: "sheet",
                title: "Sheet",
                description: Some(
                    "The same shape as the dialog. Drawer and Popover work the same way.",
                ),
                snippet: "examples/browser/snippets/trace-sheet.md",
            },
            page::Block::Example {
                id: "group",
                title: "Group",
                description: Some(
                    "Pass adjacent rows to `trace(...)`. Each row can contain a plain line, a link or a disclosure.",
                ),
                snippet: "examples/browser/snippets/trace-group.md",
            },
            page::Block::Example {
                id: "sources",
                title: "Sources",
                description: Some("A disclosure whose detail contains a list of source links."),
                snippet: "examples/browser/snippets/trace-sources.md",
            },
            page::Block::Example {
                id: "steps",
                title: "Chain of Thought",
                description: Some(
                    "A chain of thought is a collapsible row with step rows inside. Set the parent's status and title from its children.",
                ),
                snippet: "examples/browser/snippets/trace-steps.md",
            },
            page::Block::Example {
                id: "commit",
                title: "Commit",
                description: Some(
                    "A commit is a collapsible row with a line for each changed file. Say what changed in the title, Added, Edited or Deleted, and put the line counts in the description. Each file opens its diff in a Sheet.",
                ),
                snippet: "examples/browser/snippets/trace-commit.md",
            },
            page::Block::Example {
                id: "turn",
                title: "Whole Turn",
                description: Some(
                    "The turn contains a summary and its rows. Place text between two `trace(...)` lists to separate steps.",
                ),
                snippet: "examples/browser/snippets/trace-turn.md",
            },
            page::Block::Example {
                id: "thinking",
                title: "Reasoning",
                description: Some(
                    "Use a disclosure for the reasoning text. Its summary shows a spinner while the step is running. See Open State.",
                ),
                snippet: "examples/browser/snippets/trace-thinking.md",
            },
            page::Block::Example {
                id: "tool-calls",
                title: "Tool",
                description: Some(
                    "Show each tool call as a disclosure with its status in the summary and its output in the detail.",
                ),
                snippet: "examples/browser/snippets/trace-tool-calls.md",
            },
        ],
    },
    page::Block::Md("examples/browser/snippets/trace-accessibility.md"),
    page::Block::Md("examples/browser/snippets/trace-api-reference.md"),
];
pub struct TracePage {
    state: page::State,
}

impl TracePage {
    pub fn new(page_index: usize) -> Box<dyn page::Page> {
        Box::new(Self {
            state: page::State::new(page_index, BLOCKS),
        })
    }
}

impl page::Page for TracePage {
    fn page_index(&self) -> usize {
        self.state.page_index
    }
    fn meta(&self) -> &'static page::Meta {
        &META
    }
    fn blocks(&self) -> &'static [page::Block] {
        BLOCKS
    }
    fn state(&self) -> &page::State {
        &self.state
    }
    fn scrollable_id(&self) -> iced::widget::Id {
        self.state.scrollable_id.clone()
    }

    fn init(&self) -> Task<Message> {
        self.state.init()
    }

    fn update(&mut self, event: page::Message) -> Task<Message> {
        self.state.update(&event)
    }

    fn reset_volatile(&mut self) {
        self.state.reset_volatile();
    }

    fn preview(&self, id: &str) -> iced::Element<'_, Message, Theme> {
        let index = self.state.page_index;
        let content = match id {
            "demo" => component(replay::Replay::Demo).boxed(),
            "open" => component(OpenState).boxed(),
            "thinking" => component(replay::Replay::Thinking).boxed(),
            "tool-calls" => component(replay::Replay::Tool).boxed(),
            "static" => static_line().compose(),
            "status" => status().compose(),
            "description" => description().compose(),
            "collapsible" => collapsible().compose(),
            "link" => links(index).compose(),
            "dialog" => search_dialog().compose(),
            "sheet" => files_sheet().compose(),
            "group" => group().compose(),
            "sources" => sources().compose(),
            "steps" => steps(index).compose(),
            "commit" => commit().compose(),
            "turn" => turn(index).compose(),
            _ => unreachable!("unknown trace preview: {id}"),
        };
        Slot::from(container(content).width(Fill.max(384.))).compose()
    }
}
