```rust
use icedcn::widget::scroller::{Item, Position, Scroller, scroller};

pub fn saved_thread<'a, M>(rows: Vec<Item<'a, M>>, opening: Position) -> Scroller<'a, M> {
    scroller(rows).default_scroll_position(opening)
}
```

Choose `Position::Start`, `Position::End` or `Position::LastAnchor` when the thread opens. Use `scroll_to` to move an already mounted viewport.
