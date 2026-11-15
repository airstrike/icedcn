## Installation

From an application in this workspace, add icedcn:

```sh
cargo add icedcn --path ../..
```

For a separate application, point the path at your checkout and use its pinned iced dependencies with the `image` feature. Supply an `image::Handle` for each portrait.

The icon examples use `crate::icon`. Follow [Icons](icedcn://guide/installation#icons) to generate the Lucide module and register `icon::FONT`. Basic, Badge, Groups and Sizes require no icon setup.
