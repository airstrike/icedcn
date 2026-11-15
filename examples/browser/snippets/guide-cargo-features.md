## Feature flags

The root crate has no default features. Enable the capabilities needed by your application:

| Feature | Capability |
| --- | --- |
| `svg` | SVG widget and theme support. |
| `chart` | Native Hyozu charts with theme and composition support. |
| `highlighter` | Syntax highlighting support. |
| `markdown` | Markdown rendering and highlighting. |
| `spinner` | Canvas-based spinner rendering. |
| `sweeten` | Additional table integration. |

For example, a local application that renders Markdown and SVG content can use:

```toml
icedcn = { path = "../icedcn", features = ["markdown", "svg"] }
```

Keep feature selection in the application's manifest. A widget's documentation may require an iced feature as well as the corresponding icedcn feature.
