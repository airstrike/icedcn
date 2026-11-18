```rust
fn artwork(bytes: &'static [u8], handle: &OnceLock<image::Handle>) -> image::Image {
    image(
        handle
            .get_or_init(|| image::Handle::from_bytes(bytes))
            .clone(),
    )
    .content_fit(ContentFit::Cover)
    .width(Fill)
    .height(Fill)
    .border_radius(6)
}

let models = [
    (
        "v0-1.5-sm",
        "Everyday tasks and UI generation.",
        include_bytes!("../../assets/item/v0-small.jpg").as_slice(),
    ),
    (
        "v0-1.5-lg",
        "Advanced thinking or reasoning.",
        include_bytes!("../../assets/item/v0-large.jpg").as_slice(),
    ),
    (
        "v0-2.0-mini",
        "Open Source model for everyone.",
        include_bytes!("../../assets/item/v0-mini.jpg").as_slice(),
    ),
];
static IMAGES: [OnceLock<image::Handle>; 3] = [const { OnceLock::new() }; 3];
Row::with_children(models.into_iter().zip(&IMAGES).map(
    |((name, description, asset), cache)| {
        item::item([
            item::header([aspect_ratio(artwork(asset, cache).boxed(), 1.0).into()]),
            item::content([
                item::title(name).into(),
                item::description(description).into(),
            ])
            .into(),
        ])
        .orientation(icedcn::Vertical)
        .style(item::Outline)
        .into()
    },
))
.spacing(16)
.width(Fill)
```
