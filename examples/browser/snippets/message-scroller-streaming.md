```rust
use icedcn::widget::{bubble, message, scroller};
use scroller::Scroller;

pub fn live_reply<'a, M: Clone + 'a>(question: &'a str, reply: &'a str) -> Scroller<'a, M> {
    scroller([
        scroller::item(1, message(bubble(question))).anchor(true),
        scroller::item(2, message(bubble(reply).style(bubble::Ghost))),
    ])
    .auto_scroll(true)
}
```

Append arriving text to the existing reply in your update handler. Returning to the bottom resumes following.
