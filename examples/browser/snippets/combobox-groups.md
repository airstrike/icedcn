```rust
use icedxdy::widget::combobox;

const ZONES: &[(&str, &[&str])] = &[
    (
        "Americas",
        &[
            "(GMT-5) New York",
            "(GMT-8) Los Angeles",
            "(GMT-6) Chicago",
            "(GMT-5) Toronto",
            "(GMT-8) Vancouver",
            "(GMT-3) São Paulo",
        ],
    ),
    (
        "Europe",
        &[
            "(GMT+0) London",
            "(GMT+1) Paris",
            "(GMT+1) Berlin",
            "(GMT+1) Rome",
            "(GMT+1) Madrid",
            "(GMT+1) Amsterdam",
        ],
    ),
    (
        "Asia/Pacific",
        &[
            "(GMT+9) Tokyo",
            "(GMT+8) Shanghai",
            "(GMT+8) Singapore",
            "(GMT+4) Dubai",
            "(GMT+11) Sydney",
            "(GMT+9) Seoul",
        ],
    ),
];

combobox::options(ZONES.iter().flat_map(|&(group, zones)| {
    zones
        .iter()
        .map(move |&zone| combobox::item(zone, zone).group(group))
}))
.placeholder("Select a timezone")
.empty("No timezones found.")
.icon(icon::chevron_down)
```
