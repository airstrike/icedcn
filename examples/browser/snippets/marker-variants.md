```rust
column![
    status_marker("A default marker for inline notes."),
    status_marker("A separator marker").style(Separator),
    status_marker("A border marker for row boundaries.").style(Border),
]
.spacing(32)
.max_width(384.0)
.padding([48, 0])
```
