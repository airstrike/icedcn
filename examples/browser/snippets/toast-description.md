```rust
use icedcn::{
    View,
    widget::{button, toast},
};

fn description<'a>() -> View<'a> {
    toast::trigger(
        button("Show Toast").style(button::Outline),
        toast::toast("Event has been created").description("Monday, January 3rd at 6:00pm"),
    )
}
```
