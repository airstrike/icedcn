## Installation

Enable SVG for the bundled Lucide chevron and ellipsis and the custom separator examples.

```toml
[dependencies]
iced = { git = "https://github.com/airstrike/iced", branch = "dev", features = ["svg"] }
icedcn = { git = "https://github.com/airstrike/icedcn", features = ["svg"] }
```

Use the iced and cosmic-text patches from the [icedcn workspace manifest](https://github.com/airstrike/icedcn/blob/HEAD/Cargo.toml) so both dependencies resolve to the same forks and revisions.
