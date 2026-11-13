```rust
column![
    status_marker("Switched to a new branch")
        .icon(icon::git_branch()),
    status_marker("Explored 4 files")
        .icon(icon::search())
        .style(Separator),
    status_marker("Syncing completed")
        .icon(icon::book_open_check())
        .stacked(true),
]
.spacing(48)
.max_width(384.0)
.padding([48, 0])
```
