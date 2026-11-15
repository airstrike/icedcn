## Light and dark mode

Return an `icedcn::Theme` from your application's theme function and attach it with `.theme(State::theme)` on the iced application builder. Keep the chosen mode in your application state.

```rust
use icedcn::Theme;

#[derive(Default)]
pub struct Appearance {
    pub dark: bool,
}

impl Appearance {
    pub fn theme(&self) -> Theme {
        if self.dark {
            Theme::dark()
        } else {
            Theme::light()
        }
    }
}
```

Both themes provide independent surface, text, border, and focus colors. Test a component's enabled, disabled, selected, and focused states in both modes, especially when applying a custom style.
