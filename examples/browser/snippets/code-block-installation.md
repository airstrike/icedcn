## Installation

Add `icedcn` to the same Cargo workspace as your application. This repository uses the patched iced development revision recorded in its lockfile; use the same iced dependency and patch configuration when embedding these examples.

```toml
[dependencies]
icedcn = { path = "../icedcn", features = ["highlighter"] }
```

Enable `highlighter` for `.language(...)`. The `markdown` feature includes it. Enable `iced/markdown` for the Markdown examples, `iced/svg` for SVG icons, and `iced/tokio` for timed replay. The download actions use `rfd` to choose and write a file.

Code blocks use the platform monospace font by default. The browser loads Geist Mono and selects it with `.font("Geist Mono")`. Load the same font in your application to match the examples.
