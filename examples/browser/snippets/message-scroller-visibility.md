```rust
use icedcn::widget::scroller::{Item, Reader, Scroller, scroller};

#[derive(Clone, Debug)]
pub enum Event { Reading(Reader) }

pub fn transcript(rows: Vec<Item<'_, Event>>) -> Scroller<'_, Event> {
    scroller(rows).on_reader_change(Event::Reading)
}

pub fn update(reader: &mut Reader, Event::Reading(next): Event) {
    *reader = next;
}
```

Highlight `reader.current_anchor` in the outline. It stays set after the anchor scrolls above the viewport. The keys in `reader.visible` track rows intersecting the reading area, in transcript order.
