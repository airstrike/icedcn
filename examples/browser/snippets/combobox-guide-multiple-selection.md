## Multiple Selection

Supply the selected values with `.values(...)`. Each selection or removal publishes the complete list. Store that list in your application’s update handler.

```rust
use icedcn::widget::combobox;

#[derive(Clone, Debug)]
enum Message {
    Selection(Vec<String>),
}

fn frameworks(selected: &[String]) -> combobox::Combobox<'static, Message> {
    combobox(["iced", "egui", "Slint", "GTK", "Druid"])
        .values(selected.iter().cloned())
        .on_values_change(Message::Selection)
        .placeholder("Add framework")
        .width(320)
}
```
