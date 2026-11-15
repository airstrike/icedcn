```rust
use icedcn::{
    Slot,
    widget::{button, toast},
};

fn description() -> Slot<'static, ()> {
    toast::trigger(
        button("Show Toast").style(button::Outline),
        toast::toast("Event has been created").description("Monday, January 3rd at 6:00pm"),
    )
}
```
