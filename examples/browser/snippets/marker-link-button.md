```rust
column![
    button(status_marker("View the pull request").icon(icon::git_branch())).style(button::Link).inset(0).on_press(Message::ShowPullRequest),
    toast::trigger(button(status_marker("Revert this change").icon(icon::refresh_ccw())).style(button::Ghost).inset(0),toast::toast("You clicked the revert button")),
]
.spacing(32)
.max_width(384.0)
.padding([48, 0])
```
