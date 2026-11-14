## Installation

From an application in this workspace, add the local crate:

```sh
cargo add icedxdy --path ../.. --features svg
```

Use the same `iced` and `iced_widget` revisions as this workspace, with iced's `svg` feature enabled. For a separate application, point the path at your checkout. The File Tree snippet uses `iced_widget` for its small animated icon. Every SVG and helper is included in the snippets. Load Geist for the same typography; otherwise iced uses its font fallback.
