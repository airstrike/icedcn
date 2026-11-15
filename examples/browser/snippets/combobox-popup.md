```rust
use icedcn::widget::combobox;

let countries = [
    ("", "Select country"),
    ("ar", "Argentina"),
    ("au", "Australia"),
];
let items = countries.map(|(code, name)| combobox::item(code, name));
combobox::options(items)
    .popup(true)
    .placeholder("Search")
    .default_value("")
```
