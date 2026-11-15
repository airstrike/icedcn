```rust
use icedcn::widget::{bubble, message, scroller};
use scroller::{Reader, Scroller};

pub fn conversation<'a>(turns: &'a [(u64, &'a str, bool)]) -> Scroller<'a, Reader> {
    let rows = turns.iter().map(|&(key, body, starts_turn)| {
        scroller::item(key, message(bubble(body))).anchor(starts_turn)
    });
    scroller(rows).height(480).on_reader_change(|reader| reader)
}
```

Each tuple carries a persistent message key, the current text and whether it starts a turn. Updating the text under the same key preserves the row while a reply streams.
