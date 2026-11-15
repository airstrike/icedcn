```rust
use iced::{Fill, Pixels};
use icedcn::Weight;
use icedcn::widget::{column, text};

let title = text::h1("Taxing Laughter: The Joke Tax Chronicles")
    .size(36)
    .weight(Weight::ExtraBold)
    .line_height(Pixels(40.))
    .wrapping(text::Wrapping::Balanced);
let lead = text::lead("Once upon a time, in a far-off land, there was a very lazy king.")
    .line_height(Pixels(28.));
column![title, lead].width(Fill).spacing(24)
```

This opening excerpt pairs the headline and lead. The full preview uses the heading, paragraph, quote, list and table recipes shown on this page.
