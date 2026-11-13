```rust
frame(column![
message(bubble("The build failed during dependency installation.").style(Muted)).avatar(portrait(1, "R")),
message(bubble("Can you share the exact error?")).align(Right).avatar(portrait(2, "R")),
message(bubble::group([bubble("Here's the error from the logs").style(Muted).into(),
bubble("Something went wrong with the build. The libraries are not installed correctly. Try running the build again.").style(Muted).into()])).avatar(portrait(1, "R"))
].spacing(24))
```
