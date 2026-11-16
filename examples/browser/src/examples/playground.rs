//! A Rust playground inside the app: the user's file is a whole program,
//! and the example runs its `main` as they type and shows what it prints.
//! It runs in an embedded rawr engine, so a mistake shows the error rustc
//! would give, natively and on the web alike.

use std::sync::{Arc, Mutex};

use iced::{Element, Fill, Font, Task};
use icedcn::widget::scrollable::scrollable;
use icedcn::widget::{alert, column, text, textarea};
use icedcn::{Slot, Theme, card};

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
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message, Theme> {
        let mut editor = textarea("fn main() { .. }")
            .value(self.source.clone())
            .on_input(Message::Edited)
            .font(Font::MONOSPACE)
            .min_height(320.);
        if self.outcome.is_err() {
            editor = editor.style(textarea::Invalid);
        }
        let program = card(column![
            card::header("Playground").description("Rust that runs as you type, inside the app."),
            card::content(editor),
        ])
        .width(Fill);

        let output: Slot<'_, Message> = match &self.outcome {
            Ok(printed) if printed.is_empty() => text::muted("main printed nothing.").into(),
            Ok(printed) => text(printed.clone()).font(Font::MONOSPACE).into(),
            Err(error) => alert()
                .icon(icon::circle_alert())
                .title("The program doesn't run")
                .description(error.clone())
                .style(alert::Destructive)
                .into(),
        };
        let printed = card(column![
            card::header("Output").description("What main prints."),
            card::content(scrollable(output).width(Fill).height(Fill)).height(Fill),
        ])
        .width(Fill)
        .height(Fill);

        column![program, printed]
            .spacing(16)
            .padding(16)
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
