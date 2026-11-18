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

let rows = [
    (
        "Midnight City Lights",
        "Electric Nights",
        "Neon Dreams",
        "3:45",
        include_bytes!("../../assets/item/midnight-city-lights.jpg").as_slice(),
    ),
    (
        "Coffee Shop Conversations",
        "Urban Stories",
        "The Morning Brew",
        "4:05",
        include_bytes!("../../assets/item/coffee-shop-conversations.jpg").as_slice(),
    ),
    (
        "Digital Rain",
        "Binary Beats",
        "Cyber Symphony",
        "3:30",
        include_bytes!("../../assets/item/digital-rain.jpg").as_slice(),
    ),
];
static COVERS: [OnceLock<image::Handle>; 3] = [const { OnceLock::new() }; 3];
column(
    rows.into_iter()
        .zip(&COVERS)
        .map(|((title, album, artist, duration, cover), cache)| {
            item::item([
                item::media(artwork(cover, cache).boxed())
                    .style(item::Image)
                    .into(),
                item::content([
                    item::title(format!("{title} - {album}")).into(),
                    item::description(artist).into(),
                ])
                .into(),
                item::content([item::description(duration).into()])
                    .width(Shrink)
                    .into(),
            ])
            .style(item::Outline)
            .on_press(top.clone())
        }),
)
.spacing(16)
.width(Fill)
.max_width(448.0)
```
