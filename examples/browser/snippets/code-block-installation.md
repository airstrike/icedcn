## Installation

Add `icedcn` to the same Cargo workspace as your application. This repository uses the patched iced development revision recorded in its lockfile; use the same iced dependency and patch configuration when embedding these examples.

```toml
[dependencies]
icedcn = { path = "../icedcn", features = ["highlighter"] }
```

Enable `highlighter` for `.language(...)`. The `markdown` feature includes it. Enable `iced/markdown` for the Markdown examples, `iced/svg` for SVG icons, and `iced/tokio` for timed replay. The download actions use `rfd` to choose and write a file.

Code blocks use the platform monospace font by default. The browser loads Geist Mono and selects it with `.font("Geist Mono")`. Load the same font in your application to match the examples.

Demo and Actions use the Lucide font. Add the pinned generator with `cargo add --build --git https://github.com/airstrike/iced_lucide --rev 85fd2f6c533d0a88512a6a74d7db99b5de00b985 iced_lucide`, call `iced_lucide::build_all("icon")?` in your build script, declare `mod icon;` at the crate root, and register `icon::FONT` with the application’s `.font(...)` builder.
