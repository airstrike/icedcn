```rust
message(bubble(INSTALL).style(Muted)).footer(row![
    button(icon::copy())
        .square()
        .style(button::Ghost)
        .on_press(Action::Copy),
    toggle(icon::thumbs_up())
        .pressed(state.vote == Some(true))
        .on_pressed_change(|_| Action::Vote(true)),
    toggle(icon::thumbs_down())
        .pressed(state.vote == Some(false))
        .on_pressed_change(|_| Action::Vote(false)),
])
```
