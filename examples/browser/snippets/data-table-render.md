### Render the Table

Mount the demo in your view. Pass the borrowed records from your application model.

```rust
use icedcn::Slot;
use crate::payments::{Payment, payments};

#[derive(Clone, Debug)]
pub enum Message { CopyId(String) }

pub fn view<'a>(rows: &'a [Payment<'a>]) -> Slot<'a, Message> {
    payments(rows, Message::CopyId)
}
```
