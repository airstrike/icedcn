## Installation

From an application in this workspace, add the local crate:

```sh
cargo add icedcn --path ../.. --features svg
```

For an application outside this workspace, use the path to your checkout and the same iced revisions and patches as the workspace. Enable iced's `svg` feature for the built-in Lucide vectors. Load Geist to match the examples' typography.
