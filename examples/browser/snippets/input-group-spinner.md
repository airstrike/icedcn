```rust
use iced::Fill;
use icedcn::Slot;
use icedcn::widget::input_group::{Align, addon};
use icedcn::widget::{column, input, input_group, row, spinner};

fn spinners<M: Clone + 'static>(size: f32) -> Slot<'static, M> {
    column![
        input_group(input("Searching...").text_size(size))
            .addon(addon(spinner().circle()).align(Align::InlineEnd)),
        input_group(input("Processing...").text_size(size)).addon(addon(spinner().circle())),
        input_group(input("Saving changes...").text_size(size)).addon(
            addon(row![input_group::text("Saving..."), spinner().circle()].spacing(8))
                .align(Align::InlineEnd),
        ),
        input_group(input("Refreshing data...").text_size(size))
            .addon(addon(spinner().spokes()))
            .addon(addon(input_group::text("Please wait...")).align(Align::InlineEnd)),
    ]
    .spacing(16)
    .width(Fill)
    .max_width(384.)
    .into()
}
```
