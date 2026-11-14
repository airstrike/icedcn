## Installation

From an application in this workspace, add the local crate:

```sh
cargo add icedxdy --path ../.. --features svg
```

For a separate application, pass the path to your icedxdy checkout and use its pinned iced dependencies. Enable iced’s `image` and `svg` features. The examples embed their plus icons as SVG; no generated icon module or icon font is required. Supply an `image::Handle` for each portrait.
