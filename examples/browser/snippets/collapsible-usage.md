## Usage

```rust
use icedcn::Slot;
use icedcn::widget::{button, collapsible, column, text};

fn view<'a>() -> Slot<'a, ()> {
    collapsible(|c| {
        column![
            c.trigger(button("Can I use this in my project?")),
            c.content(text::small(
                "Yes. Free to use for personal and commercial projects. No attribution required."
            )),
        ]
        .into()
    })
    .into()
}
```
