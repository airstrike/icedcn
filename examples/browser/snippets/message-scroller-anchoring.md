```rust
use icedcn::widget::{bubble, message, scroller};
use scroller::Scroller;

#[derive(Clone, Copy, PartialEq)]
pub enum Speaker {
    User,
    Assistant,
}

pub fn transcript<'a, M: Clone + 'a>(
    turns: &'a [(u64, Speaker, &'a str)],
    anchor: Speaker,
) -> Scroller<'a, M> {
    scroller(turns.iter().map(|&(key, speaker, body)| {
        scroller::item(key, message(bubble(body))).anchor(speaker == anchor)
    }))
}
```
