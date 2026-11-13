```rust
use icedxdy::widget::combobox;

const COUNTRIES: &[(&str, &str, &str)] = &[
    ("", "Select country", ""),
    ("ar", "Argentina", "South America"),
    ("au", "Australia", "Oceania"),
    ("br", "Brazil", "South America"),
    ("ca", "Canada", "North America"),
    ("cn", "China", "Asia"),
    ("co", "Colombia", "South America"),
    ("eg", "Egypt", "Africa"),
    ("fr", "France", "Europe"),
    ("de", "Germany", "Europe"),
    ("it", "Italy", "Europe"),
    ("jp", "Japan", "Asia"),
    ("ke", "Kenya", "Africa"),
    ("mx", "Mexico", "North America"),
    ("nz", "New Zealand", "Oceania"),
    ("ng", "Nigeria", "Africa"),
    ("za", "South Africa", "Africa"),
    ("kr", "South Korea", "Asia"),
    ("gb", "United Kingdom", "Europe"),
    ("us", "United States", "North America"),
];

combobox::options(
    COUNTRIES
        .iter()
        .map(|&(code, label, _)| combobox::item(code, label)),
)
.popup(true)
.placeholder("Search")
.default_value("")
```
