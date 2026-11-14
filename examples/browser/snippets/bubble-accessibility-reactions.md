### Labeling Reactions

When a reaction row contains only emoji or a count such as `+8`, add text that explains which reactions the count represents.

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
