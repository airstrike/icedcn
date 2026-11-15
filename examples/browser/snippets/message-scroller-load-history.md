```rust
use icedcn::widget::{bubble, message, scroller};
use scroller::Scroller;

pub fn archive<'a, M: Clone + 'a>(history: &'a [(u64, &'a str)], first: usize) -> Scroller<'a, M> {
    let rows = history
        .iter()
        .skip(first)
        .map(|&(key, body)| scroller::item(key, message(bubble(body))));
    scroller(rows).preserve_scroll_on_prepend(true)
}
```

Lower `first` to mount earlier messages. Keep the stored message keys when a new page arrives.
