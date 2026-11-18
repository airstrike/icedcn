```rust
use icedcn::widget::combobox;

fn timezones<'a, Message: 'a>(groups: &[(&str, &[&str])]) -> combobox::Combobox<'a, Message> {
    let options = groups.iter().flat_map(|&(group, zones)| {
        zones
            .iter()
            .map(move |&zone| combobox::item(zone, zone).group(group))
    });
    combobox::options(options)
        .placeholder("Select a timezone")
        .empty("No timezones found.")
}
```
