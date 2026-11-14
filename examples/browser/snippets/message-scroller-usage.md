## Usage

Give the viewport a height and give each row a key that survives edits and prepends. Here the application supplies the messages and identifies which ones start a turn.

```rust
use icedxdy::Slot;
use icedxdy::widget::{bubble, message, scroller};

fn transcript<'a, Message: 'a>(messages: &'a [(u64, &'a str, bool)]) -> Slot<'a, Message> {
    let rows = messages.iter().map(|&(id, body, starts_turn)| {
        scroller::item(id, message(bubble(body))).anchor(starts_turn)
    });
    scroller(rows).height(360).into()
}
```
