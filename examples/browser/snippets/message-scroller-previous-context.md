```rust
use icedcn::widget::scroller::{Item, Scroller, scroller};

pub fn transcript<'a, M>(rows: Vec<Item<'a, M>>, peek: f32) -> Scroller<'a, M> {
    scroller(rows).scroll_margin(24.).previous_item_peek(peek)
}
```

Pass the slider value in logical pixels. The margin adds space above the anchor and the peek keeps the preceding reply in view.
