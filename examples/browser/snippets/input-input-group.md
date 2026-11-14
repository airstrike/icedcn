```rust
use iced::Widget;
use iced::widget::svg;
use icedcn::{Slot, Theme};
use icedcn::widget::{field, input, input_group};

fn website<Message: Clone + 'static>() -> Slot<'static, Message> {
    let info = svg(svg::Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
        fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
        <circle cx="12" cy="12" r="10"/><path d="M12 16v-4m0-4h.01"/></svg>"#))
        .width(16)
        .height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.muted.foreground.into()) });

    field(input_group(input("example.com"))
        .addon(input_group::addon(input_group::text("https://")))
        .addon(input_group::addon(info.boxed()).align(input_group::Align::InlineEnd)))
        .label("Website URL")
        .width(320)
        .into()
}
```
