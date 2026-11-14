```rust
use icedcn::widget::combobox;

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
        .filter(|(code, _, _)| !code.is_empty())
        .map(|&(code, label, continent)| {
            combobox::item(code, label).description(format!("{continent} ({code})"))
        }),
)
.placeholder("Search countries...")
.empty("No countries found.")
.icon(icon::chevron_down)
```
