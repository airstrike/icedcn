## Installation

From an application in this workspace, add the local crate:

```sh
cargo add icedxdy --path ../.. --features svg
```

For a separate application, use the path to your icedxdy checkout and the same iced revision as the workspace. Enable iced's `svg` feature for the icon examples. The snippets embed their Lucide SVG paths and need no browser-private icon helpers. Use your application's current `Theme` for the Variants example. The browser loads Geist and Geist Mono; load those font families in your application for the same typography, otherwise iced uses its font fallback.
