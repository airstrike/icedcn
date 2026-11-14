## Installation

From an application in this workspace, add the local crate:

```sh
cargo add icedxdy --path ../.. --features svg
```

For a separate application, pass the path to your icedxdy checkout and use the workspace's pinned iced dependencies. Calendar composes the included Button. The time example embeds its clock SVG and uses iced's `svg` feature.
