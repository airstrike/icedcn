```rust
use icedcn::Slot;
use icedcn::widget::textarea;

pub fn view() -> Slot<'static, ()> {
    textarea("Type your message here.").into()
}
```
