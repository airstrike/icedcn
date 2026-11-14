```rust
use icedcn::widget::select;

select([select::group([
    select::item("apple", "Apple"),
    select::item("banana", "Banana"),
    select::item("blueberry", "Blueberry"),
    select::item("grapes", "Grapes").disabled(true),
    select::item("pineapple", "Pineapple"),
])])
.placeholder("Select a fruit")
.disabled(true)
.width(192)
```
