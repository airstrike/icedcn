## Installation

An application in this workspace can inherit both dependencies in its `Cargo.toml`. Enable iced’s `image` feature to display the photos.

```toml
[dependencies]
icedxdy = { workspace = true }
iced = { workspace = true, features = ["image"] }
```

For a separate application, point `icedxdy` at your checkout and use the iced revision and patches from its workspace manifest.
