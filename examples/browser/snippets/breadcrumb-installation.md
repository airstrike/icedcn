## Installation

Enable SVG for the bundled Lucide chevron and ellipsis and the custom separator examples.

```toml
[dependencies]
iced = { git = "https://github.com/airstrike/iced", branch = "dev", features = ["svg"] }
icedxdy = { git = "https://github.com/airstrike/icedxdy", features = ["svg"] }
```

Use the iced and cosmic-text patches from the [icedxdy workspace manifest](https://github.com/airstrike/icedxdy/blob/HEAD/Cargo.toml) so both dependencies resolve to the same forks and revisions.
