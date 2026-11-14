```rust
use icedcn::widget::combobox;

fn timezones<Message: 'static>(groups: &[(&str, &[&str])]) -> combobox::Combobox<'static, Message> {
    let options = groups.iter().flat_map(|&(group, zones)| {
        zones.iter().map(move |&zone| combobox::item(zone, zone).group(group))
    });
    combobox::options(options)
        .placeholder("Select a timezone")
        .empty("No timezones found.")
        .leading_icon(combobox::globe)
        .separators(false)
        .content_width(240)
        .align_offset(-4.)
}
```
