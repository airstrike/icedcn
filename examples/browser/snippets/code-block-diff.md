```rust
use iced::Font;
use icedxdy::widget::{CodeBlock, code_block};

pub fn view<Message: Clone + 'static>() -> CodeBlock<'static, Message> {
    code_block(DIFF)
        .font(Font::new("Geist Mono"))
        .language("rust")
        .diff()
        .title(code_block::title("src/search.rs"))
        .actions(code_block::copy_button(code_block::diff_text(DIFF)))
}

const DIFF: &str = r##" fn matches(query: &str, title: &str) -> bool {
-    title.contains(query)
+    let query = query.to_lowercase();
+    title.to_lowercase().contains(&query)
 }"##;
```
