```rust
column![
    status_marker("Switched to a new branch").icon(icon::git_branch()),
    status_marker(shimmer("Thinking...")).icon(spinner().size(16.)),
    status_marker("Conversation compacted").style(Separator),
    status_marker("Explored 4 files").icon(icon::search()),
]
.spacing(32)
.max_width(384.0)
.padding([48, 0])
```
