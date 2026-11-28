```rust
use iced::FillPortion;
use icedcn::widget::{progress, row, space};

row![
    space::horizontal().width(FillPortion(1)),
    progress(self.hero_value()).width(FillPortion(3)),
    space::horizontal().width(FillPortion(1))
]
.width(iced::Fill)
.into()
```
