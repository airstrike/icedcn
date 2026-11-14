## Installation

Add the crate from your icedcn checkout:

```sh
cargo add icedcn --path /path/to/icedcn --features svg
```

Use the same pinned iced dependencies as that checkout. The `svg` feature supplies the chevron icons without an icon font. Without it, text chevrons occupy the same control boxes.
