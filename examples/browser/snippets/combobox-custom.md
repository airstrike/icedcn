```rust
use icedcn::widget::combobox;

fn countries<Message: 'static>(
    entries: &[(&str, &str, &str)],
) -> combobox::Combobox<'static, Message> {
    let items = entries.iter().map(|&(code, name, continent)| {
        combobox::item(code, name).description(format!("{continent} ({code})"))
    });
    combobox::options(items)
        .placeholder("Search countries...")
        .empty("No countries found.")
}
```
