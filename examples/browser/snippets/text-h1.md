```rust
use iced::{Center, Fill, Pixels};
use icedcn::Weight;
use icedcn::widget::text;

text::h1("Taxing Laughter: The Joke Tax Chronicles")
    .size(36).weight(Weight::ExtraBold).line_height(Pixels(40.))
    .wrapping(text::Wrapping::Balanced).width(Fill).align_x(Center)
```
