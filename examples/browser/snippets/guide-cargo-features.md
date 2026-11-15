## Feature flags

The root crate has no default features. Enable the capabilities needed by your application:

| Feature | Capability |
| --- | --- |
| `fount` | Search and load Google Fonts. |
| `svg` | SVG widget and theme support. |
| `chart` | Native Hyozu charts with theme and composition support. |
| `highlighter` | Syntax highlighting support. |
| `markdown` | Markdown rendering and highlighting. |
| `spinner` | Canvas-based spinner rendering. |
| `sweeten` | Additional table integration. |

For example, an application that renders Markdown and SVG content can use:

```toml
[dependencies.icedcn]
git = "https://github.com/airstrike/icedcn"
branch = "master"
features = ["markdown", "svg"]
```

Keep feature selection in the application's manifest. A widget's documentation may require an iced feature as well as the corresponding icedcn feature.

For `markdown` or `highlighter`, add the matching highlighter fork to the existing `[patch.crates-io]` table in your workspace root:

```toml
iced_highlighter = { git = "https://github.com/airstrike/iced.git", branch = "dev" }
```
