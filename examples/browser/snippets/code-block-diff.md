```rust
use icedcn::widget::code_block;
use icedcn::widget::code_block::LineStyle;

fn style(line: &str) -> LineStyle {
    match line.as_bytes().first() {
        Some(b'+') => code_block::Added,
        Some(b'-') => code_block::Removed,
        _ => LineStyle::Default,
    }
}

code_block(
    PATCH
        .lines()
        .map(|line| code_block::line(&line[1..]).style(style(line)))
        .collect::<Vec<_>>(),
)
.header(code_block::header("src/search.rs").actions(code_block::copy_button(PATCH)))

const PATCH: &str = r##" fn matches(query: &str, title: &str) -> bool {
-    title.contains(query)
+    let query = query.to_lowercase();
+    title.to_lowercase().contains(&query)
 }"##;
```
