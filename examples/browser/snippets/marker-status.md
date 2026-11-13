```rust
column![
    status_marker("Compacting conversation")
        .icon(spinner().size(16.)),
    status_marker("Running tests")
        .icon(spinner().size(16.))
        .style(Separator),
]
.spacing(32)
.max_width(384.0)
.padding([48, 0])
```
