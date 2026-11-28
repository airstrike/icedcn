//! Timed playback of the same example transcript and tool outputs as the docs.
use super::{command_output, line};
use crate::{Message, icon};
use iced::time::{Duration, Instant};
use iced::widget::Action;
use iced::widget::component::Component;
use iced::{Element, Event, Fill, Rectangle, Widget, mouse, window};
use icedcn::theme::container::Style;
use icedcn::widget::{button, column, container, spinner, text, trace};
use icedcn::{Theme, View};
use trace::Status;

#[derive(Clone, Copy, PartialEq)]
pub(super) enum Replay {
    Demo,
    Thinking,
    Tool,
}
#[derive(Clone, Debug)]
pub(super) enum Input {
    Start,
    Reset,
    Tick,
    Link(&'static str),
}
#[derive(Default)]
pub(super) struct State {
    start: Option<Instant>,
    step: usize,
    done: bool,
}

impl Replay {
    fn count(self) -> usize {
        match self {
            Self::Demo => 6,
            Self::Thinking => 2,
            Self::Tool => 4,
        }
    }
    fn question(self) -> &'static str {
        match self {
            Self::Demo => "Why does cargo build --locked fail in CI?",
            Self::Thinking => "Why is the release build so slow?",
            Self::Tool => "Which Rust tools are installed?",
        }
    }
    fn answer(self) -> &'static str {
        match self {
            Self::Demo => {
                "The last commit changed Cargo.toml without updating Cargo.lock. Run cargo build locally, review the lockfile changes, and commit them."
            }
            Self::Thinking => {
                "The timings report puts most of the build time in the final link. The release profile enables fat LTO. Try thin LTO and compare the next build report."
            }
            Self::Tool => "rustc and rustfmt are installed. Cargo is missing from PATH.",
        }
    }
}
const REASON: &str = "CI uses cargo build --locked. I’ll check whether the last commit updated Cargo.toml and Cargo.lock together, then reproduce the build.";
const SLOW: &str = "Cargo’s timings report shows the final executable takes nearly four minutes after its dependencies finish. Cargo.toml enables fat LTO for the release profile.";
fn thought(kind: Replay, running: bool) -> View<'static, Input> {
    let title = if running {
        "Thinking"
    } else if kind == Replay::Demo {
        "Thought for 5s"
    } else {
        "Thought"
    };
    let reason = if kind == Replay::Thinking {
        SLOW
    } else {
        REASON
    };
    trace::disclosure(
        move || {
            if running {
                trace::line(title)
                    .icon(spinner().size(16.))
                    .status(Status::Running)
            } else {
                line(
                    title,
                    if kind == Replay::Demo {
                        icon::circle_check
                    } else {
                        icon::brain
                    },
                )
            }
        },
        move || text(reason).size(14).into(),
    )
    .default_open(running)
    .into()
}
fn command(
    kind: Replay,
    command: &'static str,
    stdout: &'static str,
    error: bool,
    running: bool,
    open: bool,
) -> View<'static, Input> {
    if running {
        return trace::line(format!("Running {command}"))
            .icon(spinner().size(16.))
            .status(Status::Running)
            .into();
    }
    let summary = if error { "Failed to run" } else { "Ran" };
    let glyph = if kind == Replay::Tool {
        icon::terminal
    } else if error {
        icon::circle_alert
    } else {
        icon::circle_check
    };
    trace::disclosure(
        move || {
            trace::line(format!("{summary} {command}"))
                .icon(glyph().size(16))
                .status(if error {
                    Status::Error
                } else {
                    Status::Complete
                })
        },
        move || command_output(stdout, error),
    )
    .default_open(open)
    .into()
}
fn references() -> View<'static, Input> {
    trace::disclosure(
        || line("Used 2 sources", icon::globe),
        || {
            let sources = [
                (
                    "cargo build | The Cargo Book",
                    "https://doc.rust-lang.org/cargo/commands/cargo-build.html",
                ),
                (
                    "Cargo.toml and Cargo.lock | The Cargo Book",
                    "https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html",
                ),
            ];
            trace(sources.map(|(title, url)| {
                trace::action(
                    line(title, icon::globe).trailing(icon::arrow_up_right().size(16)),
                    Some(Input::Link(url)),
                )
            }))
            .into()
        },
    )
    .into()
}
impl<'a> Component<'a, Message, Theme> for Replay {
    type State = State;
    type Event = Input;
    fn update(&self, state: &mut State, event: Input, _: &iced::Renderer) -> Option<Message> {
        match event {
            Input::Link(url) => return Some(Message::OpenUrl(url)),
            Input::Reset => *state = State::default(),
            Input::Start => {
                *state = State {
                    start: Some(Instant::now()),
                    step: 0,
                    done: false,
                };
            }
            Input::Tick => {
                state.step += 1;
                if state.step >= self.count() {
                    state.start = None;
                    state.done = true;
                }
            }
        }
        None
    }
    fn listen(
        &self,
        state: &State,
        event: &Event,
        _: Rectangle,
        _: mouse::Cursor,
    ) -> Action<Input> {
        if let (Some(start), Event::Window(window::Event::RedrawRequested(now))) =
            (state.start, event)
        {
            let deadline = start + Duration::from_millis((state.step as u64 + 1) * 650);
            if *now >= deadline {
                return Action::publish(Input::Tick);
            }
            return Action::request_redraw_at(deadline);
        }
        Action::none()
    }
    fn view(&self, state: &State) -> impl Widget<Input, Theme> + 'a {
        let kind = *self;
        let done = state.done || (kind == Replay::Demo && state.start.is_none());
        let playing = state.start.is_some();
        let step = if done { self.count() } else { state.step };
        let mut layout = column::Column::new().spacing(12).width(Fill);
        if playing || done {
            let bubble = container(text(self.question()).size(14))
                .padding([8, 12])
                .style(|theme: &Theme| {
                    let mut s = Style::default();
                    s.background = Some(theme.muted.base.into());
                    s.border.radius = 12.into();
                    s
                });
            layout = layout.push(container(bubble).width(Fill).align_x(iced::Right).into());
            let mut items = vec![];
            if kind != Replay::Tool {
                items.push(thought(kind, step == 0));
            }
            let cmds: &[(&str, &str, bool)] = match kind {
                Self::Demo => &[
                    ("cargo --version", "cargo 1.89.0", false),
                    (
                        "git diff --stat HEAD~1 -- Cargo.toml Cargo.lock",
                        " Cargo.toml | 2 +-\n 1 file changed, 1 insertion(+), 1 deletion(-)",
                        false,
                    ),
                    (
                        "cargo build --locked",
                        "error: the lock file Cargo.lock needs to be updated but --locked was passed",
                        true,
                    ),
                ],
                Self::Tool => &[
                    ("rustc --version", "rustc 1.89.0", false),
                    ("rustfmt --version", "rustfmt 1.8.0", false),
                    ("cargo --version", "command not found: cargo", true),
                ],
                Self::Thinking => &[],
            };
            let offset = usize::from(kind == Replay::Demo);
            for (i, &(cmd, stdout, error)) in cmds.iter().enumerate() {
                if step >= offset + i {
                    items.push(command(
                        kind,
                        cmd,
                        stdout,
                        error,
                        !done && step == offset + i,
                        kind == Replay::Demo && i == 1,
                    ));
                }
            }
            if kind == Replay::Demo && step >= 4 {
                items.push(references());
            }
            layout = layout.push(trace(items).into());
            if done {
                layout = layout.push(text(self.answer()).size(14).into());
            }
        }
        if !playing {
            let (label, msg) = if kind == Replay::Demo {
                ("Replay", Input::Start)
            } else if done {
                ("Reset", Input::Reset)
            } else {
                ("Send message", Input::Start)
            };
            layout = layout.push(
                button(label)
                    .size(icedcn::Size::Sm)
                    .style(button::Outline)
                    .on_press(msg)
                    .into(),
            );
        }
        Element::from(layout)
    }
}
