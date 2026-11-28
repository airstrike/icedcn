## Components of your own

A component outside icedcn accepts the shared variants by converting its own style type from them. `Tweaked::build` starts from the variant and adds each tweak in order:

```rust
use std::ops::Add;

use icedcn::theme::{Destructive, Token, Tweak, Tweaked};
use icedcn::{Color, Theme};

#[derive(Debug, Clone, Copy)]
pub struct Meter {
    fill: Token,
}

impl Meter {
    pub fn fill(&self, theme: &Theme) -> Color {
        self.fill.resolve(theme)
    }
}

impl From<Destructive> for Meter {
    fn from(_: Destructive) -> Meter {
        Meter {
            fill: Token::Destructive,
        }
    }
}

impl<T: Into<Tweak>> Add<T> for Meter {
    type Output = Meter;

    fn add(self, tweak: T) -> Meter {
        match tweak.into() {
            Tweak::Bg(fill) => Meter { fill },
            _ => self,
        }
    }
}

impl<V> From<Tweaked<V>> for Meter
where
    Meter: From<V>,
{
    fn from(tweaked: Tweaked<V>) -> Meter {
        tweaked.build()
    }
}
```

A meter's `.style(..)` can then take `impl Into<Meter>`, and `Destructive`, `Destructive + Bg::Warning`, or a shared `danger()` all work as they do on icedcn's own components. A variant declared with `variant!` joins the same way.
