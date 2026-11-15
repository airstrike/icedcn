```rust
use icedcn::widget::{bubble, message, scroller, status_marker};
use scroller::{Item, Scroller};

pub fn add_rocky<'a, M: Clone + 'a>(
    history: impl IntoIterator<Item = Item<'a, M>>,
    joined_key: u64, reply_key: u64, body: &'a str,
) -> Scroller<'a, M> {
    let joined = status_marker("Rocky has joined the chat").style(status_marker::Separator);
    let reply = message(bubble(body).style(bubble::Tinted)).header("Rocky");
    scroller(history.into_iter().chain([
        scroller::item(joined_key, joined).anchor(true),
        scroller::item(reply_key, reply),
    ]))
}
```
