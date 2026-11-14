### Labeling Reactions

Reactions render as a row of emoji. A screen reader reads each glyph with no context, and counters like `+8` are announced as "plus eight". The web component groups the row with a descriptive accessible label. In a native application, include an explicit text alternative when the meaning would otherwise be ambiguous:

```rust
use iced::alignment::{Horizontal, Vertical};
use icedxdy::widget::bubble;

bubble("Thanks for checking.")
    .reactions("Reactions: thumbs up, fire, and 8 more", Horizontal::Right, Vertical::Bottom)
```

When reactions are interactive, render buttons with descriptive text. Set reaction padding to zero because the button provides its own padding:

```rust
use iced::alignment::{Horizontal, Vertical};
use icedxdy::widget::{bubble, button};

bubble("Thanks for checking.")
    .reactions(button("Thumbs up").style(button::Secondary), Horizontal::Right, Vertical::Bottom)
    .reaction_padding(0)
```
