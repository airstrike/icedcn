```rust
use icedcn::core::Border;
use icedcn::widget::{CodeBlock, code_block};
use icedcn::{Theme, radius, theme, widget::container};
pub fn view<'a, Message>() -> CodeBlock<'a, Message> {
    code_block(FLOATING_ACTIONS).font("Geist Mono").actions(
        container(code_block::copy_button(FLOATING_ACTIONS)).style(|theme: &Theme| {
            theme::container::Style {
                background: Some(theme.neutral.base.into()),
                border: Border {
                    width: 1.,
                    color: theme.neutral.foreground.scale_alpha(0.1),
                    radius: radius::SM.into(),
                },
                ..Default::default()
            }
        }),
    )
}

const FLOATING_ACTIONS: &str = r##"$ cargo add serde --features derive
    Updating crates.io index
      Adding serde v1.0 to dependencies
             Features:
             + derive
             + std"##;
```
