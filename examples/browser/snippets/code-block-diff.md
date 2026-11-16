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

code_block![
    code_block::header![
        "src/search.rs",
        code_block::actions![code_block::copy_button(PATCH)],
    ],
    PATCH
        .lines()
        .map(|line| code_block::line(&line[1..]).style(style(line)))
        .collect::<Vec<_>>(),
]

const PATCH: &str = r##" fn matches(query: &str, title: &str) -> bool {
-    title.contains(query)
+    let query = query.to_lowercase();
+    title.to_lowercase().contains(&query)
 }"##;
```
