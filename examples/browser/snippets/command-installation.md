## Installation

Add the library and the icon generator to your application:

```toml
[dependencies]
iced = "0.15.0-dev"
icedcn = { git = "https://github.com/airstrike/icedcn" }

[build-dependencies]
iced_lucide = { git = "https://github.com/airstrike/iced_lucide", branch = "codex/configurable-source" }
```

Copy the iced and cosmic-text patches from the [workspace manifest](https://github.com/airstrike/icedcn/blob/HEAD/Cargo.toml) so both dependencies resolve to the same forks and revisions.

For the icon examples, create `fonts/icons.toml`:

```toml
module = "icon"

[icons]
bell = "bell"
calculator = "calculator"
calendar = "calendar"
circle_question_mark = "circle-question-mark"
clipboard_paste = "clipboard-paste"
code = "code"
copy = "copy"
credit_card = "credit-card"
file_text = "file-text"
folder = "folder"
folder_plus = "folder-plus"
house = "house"
image = "image"
inbox = "inbox"
layout_grid = "layout-grid"
list = "list"
plus = "plus"
scissors = "scissors"
settings = "settings"
smile = "smile"
trash = "trash"
user = "user"
zoom_in = "zoom-in"
zoom_out = "zoom-out"
```

Generate `src/icon.rs` and the font subset with this `build.rs`:

```rust
fn main() {
    iced_lucide::build("fonts/icons.toml").expect("build command icons");
}
```

Declare `mod icon;` in your application root, then register `.fonts([icon::FONT])` on your iced application builder. The examples import this generated module with `use crate::icon;`. Load Geist for the same typography; otherwise iced uses its font fallback. Basic and Usage need no icon setup.
