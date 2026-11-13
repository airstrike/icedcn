```rust
column![
    status_marker("Switched to release-candidate")
        .icon(icon::git_branch())
        .style(Border),
    status_marker("Reviewed 8 related files")
        .icon(icon::search())
        .style(Border),
    status_marker("Opened implementation notes")
        .icon(icon::file_text())
        .style(Border),
]
.spacing(12)
.max_width(384.0)
.padding([48, 0])
```
