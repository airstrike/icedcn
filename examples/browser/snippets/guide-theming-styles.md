## How styles resolve

A widget's style is a function of the theme and the widget's status. iced calls it each time the widget draws, with the theme your application returns. The widget keeps the function, not the colors it produces.

Constructors take only content. `button("Save")` draws with the button's default variant, and when the application switches themes, it repaints with the new one on the next frame.

`.style(..)` replaces that function. Most components' style modules have a `Spec`: the variant's function, the tweaks to apply after it, and which variant it is. Variants, tweaks, and variants of your own all convert into a `Spec`, and `Spec::resolve` runs the variant and then each tweak against a theme:

```rust
use icedcn::Theme;
use icedcn::theme::{Destructive, button, radius};

pub fn square_danger(theme: &Theme) -> button::Style {
    let spec: button::Spec = (Destructive + radius(0)).into();
    spec.resolve(theme, button::Status::Active)
}
```

`resolve` returns the variant and its tweaks, which is useful in tests. A widget resolves with its full status instead, and knowing the variant lets it add what a color function alone cannot describe. A button uses it for the variant's focus ring, a link's hover underline, and the destructive border of an invalid button.
