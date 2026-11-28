## Custom designs

To change a color everywhere, change the design instead of the call sites. A `Theme` wraps a `Design`, which holds every palette, the chart colors, and the fonts. Start from `Design::light()` or `Design::dark()`:

```rust
use icedcn::core::design::{Accent, Base};
use icedcn::{Design, Theme};

pub fn theme(dark: bool) -> Theme {
    let design = if dark {
        Design::dark()
    } else {
        Design::light()
    };

    design
        .with_name("Acme")
        .with_base(Base::Stone)
        .with_accent(Accent::Violet)
        .with_chart(Accent::Violet)
        .with_font_sans("Inter")
        .into()
}
```

- `with_base` sets the surfaces, the muted and accent palettes, the input border, and the sidebar surfaces: `Neutral`, `Stone`, `Zinc`, `Mauve`, `Olive`, `Mist`, or `Taupe`. `Slate` instead gives the existing neutral colors, code included, a slight blue tint.
- `with_accent` sets the primary and secondary palettes and the sidebar's primary color, from neutrals such as `Zinc` or hues such as `Blue`, `Emerald`, `Rose`, and `Violet`.
- `with_chart` sets the five chart colors, from an accent's chart family or an explicit array.
- `with_menu_accent` highlights menu items with the primary colors, copying the primary palette as it is when called.
- `with_font_sans` and `with_font_mono` choose the fonts. A family name derives its medium, semibold, and bold weights.

The builders apply in order, so call `with_base` before `with_accent`, and `with_menu_accent` after both.

Every field is public, so a palette can also be replaced directly. Set its related colors together:

```rust
use icedcn::core::oklch;
use icedcn::{Design, Palette, Theme};

pub fn violet_destructive() -> Theme {
    let mut design = Design::dark();
    let base = oklch(0.6, 0.2, 290.0);
    design.destructive = Palette {
        base,
        border: base,
        ring: base.alpha(0.5),
        tint_text: base,
        ..design.destructive
    };
    design.into()
}
```

Variants read roles, so the new `destructive` palette restyles `Destructive` on every component. Convert with `.into()` rather than `Theme::new`: the conversion also updates the seed that stock iced widgets read, so their background, text, primary, success, warning, and danger colors match the palettes.

Choosing a font does not load it. Register the font bytes with iced first, or enable the `fount` feature to search and load Google Fonts. The repository's palette example is a working application for inspecting every role:

```sh
cargo run --locked -p palette
```
