## Installation

From an application in this workspace, add the local crate:

```sh
cargo add icedcn --path ../.. --features svg
```

For a separate application, pass the path to your icedcn checkout. The complete icon examples use iced's `svg` feature and embed their SVG paths; they do not require the browser's generated icon module. Import the group macro and the parts you use from `icedcn::widget::button::group`.
