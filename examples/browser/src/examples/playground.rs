//! A Rust scratchpad: the file is a whole program, and its `main` runs in an
//! embedded rawr engine on every edit, natively and on the web.

use std::sync::mpsc;

use iced::{Element, Fill, Task};
use icedcn::widget::scrollable::scrollable;
use icedcn::widget::{button, code_block, column, row};
use icedcn::{Destructive, Theme};

use crate::icon;

/// The program a new user starts from.
const STARTER: &str = r#"use std::fmt;

struct Planet {
    name: &'static str,
    moons: u32,
    day: f64,
}

impl Planet {
    /// How many of its days pass in one Earth week.
    fn days_per_week(&self) -> f64 {
        7.0 * 24.0 / self.day
    }
}

impl fmt::Display for Planet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Planet { name, moons, day } = self;
        let week = self.days_per_week();
        write!(f, "{name:<8} {moons:>5} {day:>8.1} {week:>7.2}")
    }
}

fn main() {
    let planets = [
        Planet { name: "Mercury", moons: 0, day: 4222.6 },
        Planet { name: "Earth", moons: 1, day: 24.0 },
        Planet { name: "Mars", moons: 2, day: 24.7 },
        Planet { name: "Jupiter", moons: 95, day: 9.9 },
    ];
    println!("{:<8} {:>5} {:>8} {:>7}", "planet", "moons", "day (h)", "per wk");
    for planet in &planets {
        println!("{planet}");
    }
}
"#;

/// The window width that fits 120 columns of 14px mono (8.4px each) in
/// each half: 1008 + 60 of line numbers and padding + 2 of border = 1070 per
/// half, then 16 between the halves, 32 around them and 2 of window border.
pub const WIDTH: f32 = 2. * 1070. + 16. + 32. + 2.;

/// How many steps a run may take, so a loop that never ends stops instead
/// of freezing the app.
const MAX_OPERATIONS: u64 = 1_000_000;

pub struct State {
    source: String,
    outcome: Result<String, String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Edited(String),
    Reset,
}

impl Default for State {
    fn default() -> Self {
        Self {
            source: STARTER.to_owned(),
            outcome: run(STARTER),
        }
    }
}

impl State {
    pub fn title(&self) -> String {
        "Playground".to_owned()
    }

    /// What the last run printed, or `None` when it failed.
    #[cfg(test)]
    pub(crate) fn printed(&self) -> Option<&str> {
        self.outcome.as_deref().ok()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Edited(source) => {
                self.outcome = run(&source);
                self.source = source;
            }
            Message::Reset => {
                self.outcome = run(STARTER);
                STARTER.clone_into(&mut self.source);
            }
        }
        Task::none()
    }

    /// Side by side from `width` 768 up, filling the window; stacked below
    /// it, at natural heights under a page scroll.
    pub fn view(&self, width: f32) -> Element<'_, Message, Theme> {
        let wide = width >= 768.;
        let content = code_block::content(self.source.as_str())
            .language("rust")
            .line_numbers(true)
            .on_input(Message::Edited);
        let editor = code_block(if wide { content.height(Fill) } else { content }).header(
            code_block::header(code_block::title("main.rs").icon(icon::file_code())).actions(
                code_block::actions![
                    button(icon::rotate_ccw())
                        .style(button::Ghost)
                        .square()
                        .on_press(Message::Reset),
                    code_block::copy_button(self.source.as_str()),
                ],
            ),
        );

        let (printed, failed) = match &self.outcome {
            Ok(printed) if printed.is_empty() => ("main printed nothing.", false),
            Ok(printed) => (printed.as_str(), false),
            Err(error) => (error.as_str(), true),
        };
        let title = code_block::title("Output").icon(if failed {
            icon::circle_alert()
        } else {
            icon::terminal()
        });
        let content = code_block::content(printed).wrap(failed);
        let output = code_block(if wide { content.height(Fill) } else { content })
            .header(code_block::header(title).actions(code_block::copy_button(printed)));
        let output = if failed {
            output.style(Destructive)
        } else {
            output
        };

        if wide {
            return row![
                column![editor].width(Fill).height(Fill),
                column![output].width(Fill).height(Fill),
            ]
            .spacing(16)
            .padding(16)
            .width(Fill)
            .height(Fill)
            .into();
        }
        scrollable(column![editor, output].spacing(16).padding(16).width(Fill))
            .width(Fill)
            .height(Fill)
            .into()
    }
}

/// Compile `source` and run its `main`: what it printed, or the error rustc
/// would give, as rawr prints it.
fn run(source: &str) -> Result<String, String> {
    let (sender, printed) = mpsc::channel();
    let mut engine = rawr_engine::Engine::new();
    engine.set_max_operations(MAX_OPERATIONS);
    engine.on_output(move |_, text| {
        let _ = sender.send(text.to_owned());
    });
    let script = engine.script(source).map_err(|error| error.to_string())?;
    let main = script
        .func::<(), ()>("main")
        .map_err(|error| error.to_string())?;
    main.call().map_err(|error| error.to_string())?;
    Ok(printed.try_iter().collect())
}
