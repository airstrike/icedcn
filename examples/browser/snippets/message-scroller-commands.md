```rust
use iced::Task;
use icedcn::widget::scroller;
use scroller::{Animation, Item, Scroller, Target};

#[derive(Clone, Debug)]
pub enum Event {
    Jump(u64),
}

pub fn view(rows: Vec<Item<'_, Event>>) -> Scroller<'_, Event> {
    scroller(rows).id("conversation")
}

pub fn update(Event::Jump(key): Event) -> Task<Event> {
    scroller::scroll_to("conversation", Target::Item(key), Animation::Smooth)
}
```

Have the jump menu send `Event::Jump(key)` for the chosen message.
