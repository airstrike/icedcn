```rust
use icedcn::widget::select;

select([select::group([
    select::label("Fruits"),
    select::item("apple", "Apple"),
    select::item("banana", "Banana"),
    select::item("blueberry", "Blueberry"),
    select::item("grapes", "Grapes"),
    select::item("pineapple", "Pineapple"),
])])
.placeholder("Select a fruit")
.width(192)
```
