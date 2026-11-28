//! Rust and Cargo listings for the native Code Block examples.

pub const ACTIONS: &str = r##"$ cargo clippy -- -D warnings
    Checking notebook v0.1.0
error: writing `&String` instead of `&str` involves a new object where a slice will do
 --> src/search.rs:4:18
  |
4 | fn matches(query: &String, title: &str) -> bool {
  |                   ^^^^^^^ help: change this to: `&str`"##;

pub const ANSI: &str = concat!(
    "$ cargo test --color always\n",
    "    \x1b[2mRunning tests/search.rs\x1b[22m\n",
    "running 3 tests\n",
    "test blank_query ... \x1b[32mok\x1b[39m\n",
    "test exact_match ... \x1b[32mok\x1b[39m\n",
    "test mixed_case ... \x1b[31mFAILED\x1b[39m\n",
    "\n",
    "test result: \x1b[31mFAILED\x1b[39m. 2 passed; 1 failed; 0 ignored",
);

pub const BASIC: &str = r##"$ cargo build --release
   Compiling icedcn_core v0.1.0
   Compiling icedcn_widget v0.1.0
   Compiling icedcn v0.1.0
   Compiling notebook v0.1.0
    Finished `release` profile [optimized] target(s) in 4.20s"##;

pub const CHAT: &str = r##"fn matches(query: &str, title: &str) -> bool {
    title.to_lowercase().contains(&query.to_lowercase())
}

fn search<'a>(query: &str, titles: &[&'a str]) -> Vec<&'a str> {
    titles
        .iter()
        .copied()
        .filter(|title| matches(query, title))
        .collect()
}"##;

pub const DEMO: &str = r##"use icedcn::{View, widget::{column, input, text}};

#[derive(Clone, Debug)]
pub enum Message { SearchChanged(String) }

pub fn view<'a>(query: &'a str, titles: &[&'a str]) -> View<'a, Message> {
    let search = input("Find a notebook").value(query.to_owned()).on_input(Message::SearchChanged);
    let mut results = column![search].spacing(8);
    for title in titles.iter().copied().filter(|title| title.contains(query)) {
        results = results.push(text::p(title).into());
    }
    results.into()
}"##;

pub const DESTRUCTIVE: &str = r##"thread 'main' panicked at src/notebook.rs:18:22:
index out of bounds: the len is 0 but the index is 0
note: run with `RUST_BACKTRACE=1` to display a backtrace"##;

pub const DIFF: &str = r##" fn matches(query: &str, title: &str) -> bool {
-    title.contains(query)
+    let query = query.to_lowercase();
+    title.to_lowercase().contains(&query)
 }"##;

pub const FLOATING_ACTIONS: &str = r##"$ cargo add serde --features derive
    Updating crates.io index
      Adding serde v1.0 to dependencies
             Features:
             + derive
             + std"##;

pub const COMMAND: &str = "cargo check -p icedcn_widget";

pub const HEADER: &str = r##"$ cargo check -p icedcn_widget
    Checking icedcn_core v0.1.0
    Checking icedcn_widget v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.80s"##;

pub const HIGHLIGHTED_LINES: &str = r##"[package]
name = "notebook"
version = "0.1.0"
edition = "2024"

[dependencies]
icedcn = { git = "https://github.com/airstrike/icedcn", branch = "master" }"##;

pub const HIGHLIGHTING: &str = r##"#[derive(Debug, Default)]
struct Notebook {
    title: String,
    pages: Vec<String>,
    archived: bool,
}"##;

pub const LINE_NUMBERS: &str = r##"[package]
name = "notebook"
version = "0.1.0"
edition = "2024"

[dependencies]
icedcn = { git = "https://github.com/airstrike/icedcn", branch = "master" }"##;

pub const MARKDOWN: &str = r##"Normalize the query and title with `to_lowercase` before comparing them.

```rust
fn matches(query: &str, title: &str) -> bool {
    let query = query.trim().to_lowercase();
    title.to_lowercase().contains(&query)
}
```

Trimming the query also ignores extra spaces at either end."##;

pub const SHOW_MORE: &str = r##"use icedcn::widget::{button, column, input, text};
use icedcn::View;

#[derive(Default)]
pub struct Notebook {
    draft: String,
    notes: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Edited(String),
    Add,
}

impl Notebook {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Edited(draft) => self.draft = draft,
            Message::Add if !self.draft.trim().is_empty() => {
                self.notes.push(std::mem::take(&mut self.draft));
            }
            Message::Add => {}
        }
    }

    pub fn view(&self) -> View<'_, Message> {
        let draft = input("Write a note")
            .value(self.draft.clone())
            .on_input(Message::Edited);
        let add = button("Add").on_press(Message::Add);
        let mut notes = column![draft, add].spacing(8);
        for note in &self.notes {
            notes = notes.push(text::p(note.as_str()).into());
        }
        notes.into()
    }
}"##;

pub const SIZE: &str = r##"$ cargo check -p notebook
    Checking notebook v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.32s"##;

pub const START_LINE: &str = r##"    let query = self.query.trim().to_lowercase();

    self.notebooks
        .iter()
        .filter(|book| book.title.to_lowercase().contains(&query))
        .map(|book| book.title.as_str())
        .collect()
}"##;

pub const TRACE: &str = r##"$ cargo --version
cargo 1.85.0 (d73d2caf9 2024-12-31)"##;

pub const TRACE_ERROR: &str = r##"$ cargo run --bin notes
error: no bin target named `notes` in default-run packages
help: available bin targets:
    notebook"##;

pub const TYPESET: &str = r##"use std::path::Path;

fn load_titles(path: &Path) -> std::io::Result<Vec<String>> {
    let source = std::fs::read_to_string(path)?;
    Ok(source.lines().map(str::to_owned).collect())
}"##;

pub const EDITABLE: &str = r##"fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}"##;

pub const WRAP: &str = r##"error: failed to read notebook index from /home/reader/.local/share/notebook/collections/research/field-notes/archive/index.toml: No such file or directory (os error 2)"##;

pub struct File {
    pub id: &'static str,
    pub title: &'static str,
    pub language: &'static str,
    pub code: &'static str,
}

pub const FILES: &[File] = &[
    File {
        id: "view",
        title: "src/view.rs",
        language: "rust",
        code: r##"use icedcn::widget::{column, text};
use icedcn::View;

pub fn view(titles: &[String]) -> View<'_> {
    let mut items = column![].spacing(8);
    for title in titles {
        items = items.push(text::p(title.as_str()).into());
    }
    items.into()
}"##,
    },
    File {
        id: "search",
        title: "src/search.rs",
        language: "rust",
        code: r##"fn matches(query: &str, title: &str) -> bool {
    title.to_lowercase().contains(&query.to_lowercase())
}

fn search<'a>(query: &str, titles: &[&'a str]) -> Vec<&'a str> {
    titles
        .iter()
        .copied()
        .filter(|title| matches(query, title))
        .collect()
}"##,
    },
    File {
        id: "config",
        title: ".cargo/config.toml",
        language: "toml",
        code: r##"[env]
RUST_LOG = "notebook=debug""##,
    },
];

pub struct Sample {
    pub language: &'static str,
    pub title: &'static str,
    pub code: &'static str,
}

pub const SAMPLES: &[Sample] = &[
    Sample {
        language: "toml",
        title: "Cargo.toml",
        code: r##"[package]
name = "notebook"
version = "0.1.0"
edition = "2024""##,
    },
    Sample {
        language: "rust",
        title: "src/main.rs",
        code: r##"fn main() {
    println!("Notebook is ready.");
}"##,
    },
    Sample {
        language: "text",
        title: "notes.txt",
        code: "A language that is not in the list renders as plain text.",
    },
];

pub struct Stream {
    pub title: Option<&'static str>,
    pub language: &'static str,
    pub code: &'static str,
    pub line_numbers: bool,
    pub highlight: &'static [usize],
    pub diff: bool,
}

pub const STREAMS: &[Stream] = &[
    Stream {
        title: Some("src/matcher.rs"),
        language: "rust",
        line_numbers: true,
        highlight: &[],
        diff: false,
        code: r##"fn matches(query: &str, title: &str) -> bool {
    let query = query.trim().to_lowercase();
    title.to_lowercase().contains(&query)
}"##,
    },
    Stream {
        title: None,
        language: "sh",
        line_numbers: false,
        highlight: &[],
        diff: false,
        code: "cargo test --lib search",
    },
    Stream {
        title: Some("src/search.rs"),
        language: "rust",
        line_numbers: true,
        highlight: &[],
        diff: true,
        code: r##" fn search<'a>(query: &str, titles: &[&'a str]) -> Vec<&'a str> {
     titles.iter().copied()
-        .filter(|title| title.contains(query))
+        .filter(|title| matches(query, title))
         .collect()
 }"##,
    },
    Stream {
        title: Some("src/search.rs"),
        language: "rust",
        line_numbers: true,
        highlight: &[5, 6, 7, 8],
        diff: false,
        code: r##"fn matches(query: &str, title: &str) -> bool {
    title.to_lowercase().contains(&query.to_lowercase())
}

fn search<'a>(query: &str, titles: &[&'a str]) -> Vec<&'a str> {
    titles
        .iter()
        .copied()
        .filter(|title| matches(query, title))
        .collect()
}"##,
    },
    Stream {
        title: Some(".cargo/config.toml"),
        language: "toml",
        line_numbers: false,
        highlight: &[],
        diff: false,
        code: r##"[env]
RUST_LOG = "notebook=debug""##,
    },
];
