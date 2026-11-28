```rust
use iced::widget::image;
use iced::{ContentFit, Widget};
use icedcn::theme;
use icedcn::widget::scrollable::Direction;
use icedcn::widget::{Row, column, container, row, scroll_area, text};
use std::sync::OnceLock;

let works = [
    (
        "Ornella Binni",
        include_bytes!("../../assets/artwork/ornella-binni.jpg").as_slice(),
    ),
    (
        "Tom Byrom",
        include_bytes!("../../assets/artwork/tom-byrom.jpg").as_slice(),
    ),
    (
        "Vladimir Malyavko",
        include_bytes!("../../assets/artwork/vladimir-malyavko.jpg").as_slice(),
    ),
];
static IMAGES: [OnceLock<image::Handle>; 3] = [const { OnceLock::new() }; 3];
let figures = Row::with_children(works.into_iter().zip(&IMAGES).map(
    |((artist, bytes), cache)| {
        let handle = cache.get_or_init(|| image::Handle::from_bytes(bytes));
        column![
            image(handle.clone())
                .width(150)
                .height(200)
                .content_fit(ContentFit::Cover)
                .border_radius(6)
                .boxed(),
            row![
                text::xs("Photo by").style(theme::text::Muted),
                text::xs(artist).semibold()
            ]
            .spacing(3)
        ]
        .spacing(8)
        .into()
    },
))
.spacing(16);
frame(
    scroll_area(container(figures).padding(16))
        .direction(Direction::Horizontal(scroll_area::bar()))
        .width(382),
)
```
