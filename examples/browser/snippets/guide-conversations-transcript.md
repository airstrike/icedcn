## A keyed transcript

Compose a conversation from [Message](icedcn://component/message) rows and [Bubble](icedcn://component/bubble) content inside [Message Scroller](icedcn://component/message-scroller). Give each row a stable key and give the viewport an explicit height or a bounded parent.

```rust
use icedcn::Slot;
use icedcn::widget::{bubble, message, scroller};

pub fn transcript<'a, Message: Clone + 'a>(
    entries: &'a [(u64, String, bool)],
) -> Slot<'a, Message> {
    let rows = entries.iter().map(|(id, body, starts_turn)| {
        scroller::item(*id, message(bubble(body.as_str()))).anchor(*starts_turn)
    });
    scroller(rows).height(360).into()
}
```

The boolean marks a turn boundary. The default opening position is the last anchor. Keep each key attached to the same message across edits and prepends; a row's array position is not a stable identity when earlier history is inserted.
