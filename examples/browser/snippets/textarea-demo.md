```rust
use icedcn::Slot;
use icedcn::widget::textarea;

pub fn view<'a>() -> Slot<'a, ()> {
    textarea("Type your message here.").into()
}
```
