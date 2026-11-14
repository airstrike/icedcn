```rust
use iced::Font;
use icedxdy::core::Border;
use icedxdy::widget::{CodeBlock, code_block};
use icedxdy::{Theme, radius, theme, widget::container};
pub fn view<Message: Clone + 'static>() -> CodeBlock<'static, Message> {
    code_block(FLOATING_ACTIONS)
        .font(Font::new("Geist Mono"))
        .actions(
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
