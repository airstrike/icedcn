```rust
use icedcn::widget::{code_block, collapsible::Collapsible};

pub fn view<Message: 'static>(source: &str) -> Collapsible<'_, Message> {
    code_block::show_more(256, move || {
        code_block(source)
            .language("rust")
            .font("Geist Mono")
            .title("src/notebook.rs")
            .start_line(1)
            .actions(code_block::copy_button(source))
    })
}
```
