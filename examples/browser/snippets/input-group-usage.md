## Usage

Place the input or textarea inside the group. A passive addon focuses the control when clicked. Buttons inside an addon handle their own presses. These examples use a control text size of 16px below a 768px window width and 14px at larger widths.

```rust
use icedcn::Slot;
use icedcn::widget::{input, input_group};

fn website<M: Clone + 'static>() -> Slot<'static, M> {
    input_group(input("example.com"))
        .addon(input_group::addon(input_group::text("https://")))
        .into()
}
```
