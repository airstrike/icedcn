## Usage

Pass a control to `field` and give it a label. A description stays beside the control when the field changes orientation.

```rust
use iced::Fill;
use icedcn::View;
use icedcn::widget::{field, input};

fn full_name<'a>() -> View<'a> {
    field(input("Evil Rabbit").width(Fill))
        .label("Full name")
        .description("This appears on invoices and emails.")
        .into()
}
```
