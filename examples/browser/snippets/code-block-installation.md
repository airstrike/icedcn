## Installation

Add `icedcn` to the same Cargo workspace as your application. This repository uses the patched iced development revision recorded in its lockfile; use the same iced dependency and patch configuration when embedding these examples.

```toml
[dependencies]
icedcn = { path = "../icedcn", features = ["highlighter"] }
```

Enable `highlighter` for `.language(...)`. The `markdown` feature includes it. Enable `iced/markdown` for the Markdown examples, `iced/svg` for SVG icons, and `iced/tokio` for timed replay. The download actions use `rfd` to choose and write a file.

The browser loads Geist Mono. Set `.font(Font::MONOSPACE)` to use the platform monospace font, or load Geist Mono with your application's font configuration and use `Font::new("Geist Mono")`.
