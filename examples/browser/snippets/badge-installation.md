## Installation

From an application in this workspace, add the local crate:

```sh
cargo add icedcn --path ../.. --features spinner,svg
```

For a separate application, pass the path to your icedcn checkout. The icon examples also use iced's `svg` feature. They embed their SVG paths, so no generated icon module or font is needed.
