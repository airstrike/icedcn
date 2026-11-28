```rust
use icedcn::View;
use icedcn::widget::textarea;

pub fn view<'a>() -> View<'a> {
    textarea("Type your message here.").into()
}
```
