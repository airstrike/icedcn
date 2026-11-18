```rust
use icedcn::Slot;
use icedcn::widget::select;

fn zone(label: &str, items: &[(&str, &str)]) -> select::Entry {
    select::group(
        std::iter::once(select::label(label)).chain(
            items
                .iter()
                .map(|(value, label)| select::item(*value, *label)),
        ),
    )
}

fn timezones<'a, Message: 'static>() -> Slot<'a, Message> {
    select([
        zone(
            "North America",
            &[
                ("est", "Eastern Standard Time"),
                ("cst", "Central Standard Time"),
                ("mst", "Mountain Standard Time"),
                ("pst", "Pacific Standard Time"),
                ("akst", "Alaska Standard Time"),
                ("hst", "Hawaii Standard Time"),
            ],
        ),
        zone(
            "Europe & Africa",
            &[
                ("gmt", "Greenwich Mean Time"),
                ("cet", "Central European Time"),
                ("eet", "Eastern European Time"),
                ("west", "Western European Summer Time"),
                ("cat", "Central Africa Time"),
                ("eat", "East Africa Time"),
            ],
        ),
        zone(
            "Asia",
            &[
                ("msk", "Moscow Time"),
                ("ist", "India Standard Time"),
                ("cst_china", "China Standard Time"),
                ("jst", "Japan Standard Time"),
                ("kst", "Korea Standard Time"),
                ("ist_indonesia", "Indonesia Central Standard Time"),
            ],
        ),
        zone(
            "Australia & Pacific",
            &[
                ("awst", "Australian Western Standard Time"),
                ("acst", "Australian Central Standard Time"),
                ("aest", "Australian Eastern Standard Time"),
                ("nzst", "New Zealand Standard Time"),
                ("fjt", "Fiji Time"),
            ],
        ),
        zone(
            "South America",
            &[
                ("art", "Argentina Time"),
                ("bot", "Bolivia Time"),
                ("brt", "Brasilia Time"),
                ("clt", "Chile Standard Time"),
            ],
        ),
    ])
    .placeholder("Select a timezone")
    .width(256)
    .into()
}
```
