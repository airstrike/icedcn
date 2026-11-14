```rust
use icedcn::widget::{CodeBlock, code_block};

pub fn view<Message: 'static>() -> CodeBlock<'static, Message> {
    code_block(DIFF)
        .font("Geist Mono")
        .language("rust")
        .diff()
        .title("src/search.rs")
        .actions(code_block::copy_button(code_block::diff_text(DIFF)))
}

const DIFF: &str = r##" fn matches(query: &str, title: &str) -> bool {
-    title.contains(query)
+    let query = query.to_lowercase();
+    title.to_lowercase().contains(&query)
 }"##;
```
