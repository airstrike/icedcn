```rust
use iced::Task;
use icedcn::widget::{scroller, text};
use scroller::{Animation, Item, Position, Reader, Scroller, Target};

#[derive(Clone, Debug)]
pub enum Event { Reading(Reader), Latest }

pub fn view<'a>(rows: Vec<Item<'a, Event>>, reader: &Reader) -> Scroller<'a, Event> {
    scroller(rows).id("conversation").default_scroll_position(Position::Start)
        .on_reader_change(Event::Reading)
        .end_button(reader.can_scroll_down, |size| text("↓").size(size), Event::Latest)
}

pub fn update(reader: &mut Reader, event: Event) -> Task<Event> {
    match event {
        Event::Reading(next) => { *reader = next; Task::none() }
        Event::Latest => scroller::scroll_to("conversation", Target::End, Animation::Smooth),
    }
}
```
