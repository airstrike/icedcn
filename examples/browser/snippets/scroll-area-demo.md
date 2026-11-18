```rust
let rows = (1..=50).rev().map(|n| {
    column![text::small(format!("v1.2.0-beta.{n}")), separator()]
        .spacing(8)
        .into()
});
let contents = column![text::label("Tags"), column(rows).spacing(8)]
    .spacing(16)
    .padding(16);
frame(scroll_area(contents).width(190).height(286))
```
