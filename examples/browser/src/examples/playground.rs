//! A Rust scratchpad: the file is a whole program, and its `main` runs in an
//! embedded rawr engine on every edit, natively and on the web.

use std::sync::{Arc, Mutex};

use iced::{Element, Fill, Font, Task};
use icedcn::widget::scrollable::scrollable;
use icedcn::widget::{alert, button, code_block, column, row, text};
use icedcn::{Destructive, Slot, Theme, card};

use crate::icon;

/// The program a new user starts from.
const STARTER: &str = "\
// Each bill gets a tip by the rule in `tip`. Change the rule, the bills
// or what `main` prints: it runs again as you type.

const BILLS: [f64; 4] = [12.50, 48.00, 75.25, 210.00];

/// The tip on a bill, in dollars.
fn tip(bill: f64) -> f64 {
    let rate = if bill < 50.0 { 0.18 } else { 0.20 };
    (bill * rate * 100.0).round() / 100.0
}

fn main() {
    for bill in BILLS {
        let tip = tip(bill);
        println!(\"${bill:>7.2}  tip ${tip:>6.2}  ({:.1}%)\", tip / bill * 100.0);
    }
}
";

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
        let editor = code_block![
            code_block::header![
                code_block::title![file_code, "main.rs"],
                code_block::actions![
                    button(icon::rotate_ccw())
                        .style(button::Ghost)
                        .square()
                        .on_press(Message::Reset),
                    code_block::copy_button(self.source.as_str()),
                ],
            ],
            if wide { content.height(Fill) } else { content },
        ];

        let output: Slot<'_, Message> = match &self.outcome {
            Ok(printed) if printed.is_empty() => text::muted("main printed nothing.").into(),
            Ok(printed) => text(printed.clone()).font(Font::MONOSPACE).into(),
            Err(error) => alert()
                .icon(icon::circle_alert())
                .title("The program doesn't run")
                .description(error.clone())
                .description_font(Font::MONOSPACE)
                .style(Destructive)
                .into(),
        };

        if wide {
            let output = card(
                column![
                    card::header("Output"),
                    card::content(scrollable(output).width(Fill).height(Fill))
                        .width(Fill)
                        .height(Fill),
                ]
                .width(Fill)
                .height(Fill),
            )
            .width(Fill)
            .height(Fill);
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
        let output = card(column![card::header("Output"), card::content(output)]).width(Fill);
        scrollable(column![editor, output].spacing(16).padding(16).width(Fill))
            .width(Fill)
            .height(Fill)
            .into()
    }
}

/// Compile `source` and run its `main`: what it printed, or the error rustc
/// would give, as rawr prints it.
fn run(source: &str) -> Result<String, String> {
    let printed = Arc::new(Mutex::new(String::new()));
    let mut engine = rawr_engine::Engine::new();
    engine.set_max_operations(MAX_OPERATIONS);
    let sink = Arc::clone(&printed);
    engine.on_output(move |_, text| {
        if let Ok(mut printed) = sink.lock() {
            printed.push_str(text);
        }
    });
    let script = engine.script(source).map_err(|error| error.to_string())?;
    let main = script
        .func::<(), ()>("main")
        .map_err(|error| error.to_string())?;
    main.call().map_err(|error| error.to_string())?;
    let printed = printed
        .lock()
        .map(|printed| printed.clone())
        .unwrap_or_default();
    Ok(printed)
}
