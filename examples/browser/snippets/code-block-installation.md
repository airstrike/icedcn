## Installation

Add `icedxdy` to the same Cargo workspace as your application. This repository uses the patched iced development revision recorded in its lockfile; use the same iced dependency and patch configuration when embedding these examples.

```toml
[dependencies]
icedxdy = { path = "../icedxdy" }
```

The widget needs no syntax engine for plain output. The highlighted examples include their fixed token tables and require no runtime syntax engine. Markdown examples also enable `icedxdy/markdown` and `iced/markdown`. Interactive examples use iced's `tokio` runtime; the download examples use `rfd` and `tokio`. The self-contained icon helpers use `iced/svg` and `icedxdy/svg`.

The browser loads Geist Mono. Set `.font(Font::MONOSPACE)` to use the platform monospace font, or load Geist Mono with your application's font configuration and use `Font::new("Geist Mono")`.
