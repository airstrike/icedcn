## Usage

Use the size, weight, line height and wrapping builders to set the article’s type scale. Each listing is an expression you can place in your view.

```rust
use iced::Pixels;
use icedcn::widget::{column, text};

column![
    text::h3("The Joke Tax").size(24).semibold().line_height(Pixels(32.)),
    text::p("The king’s subjects were not amused.").line_height(Pixels(28.)),
].spacing(24)
```
