## Installation

From an application in this workspace, add the local crate:

```sh
cargo add icedcn --path ../.. --features svg
```

Use the same `iced` revision as this workspace, with its `svg` feature enabled. For a separate application, point the path at your checkout. The animated disclosure indicator is part of icedcn. Every SVG and helper is included in the snippets. Load Geist for the same typography; otherwise iced uses its font fallback.
