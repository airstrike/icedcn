## Usage

Use the size, weight, line height and wrapping builders to set the article’s type scale. Each listing is an expression you can place in your view.

```rust
use iced::Pixels;
use icedcn::widget::{column, text};

column![
    text::xl2("The Joke Tax")
        .semibold()
        .line_height(Pixels(32.)),
    text::md("The king’s subjects were not amused.").line_height(Pixels(28.)),
]
.spacing(24)
```

The compact size helpers are `xs`, `sm`, `md`, `lg`, `xl`, `xl2` and `xl3`. The `lg` helper preserves `large`'s semibold weight; the others keep the default weight. Use `Pixels(...)` for absolute line heights and a number such as `1.5` for a relative multiplier.
