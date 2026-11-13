```rust
let source = r##" export function Greeting({ name }: { name: string }) {
-  return <p>Hello, {name}</p>
+  return <p className="text-sm text-muted-foreground">
+    Hello, {name}!
+  </p>
 }"##;

CodeBlock::lines(source.lines().map(|line| {
    let style = match line.chars().next() {
        Some('+') => LineStyle::Added,
        Some('-') => LineStyle::Removed,
        _ => LineStyle::Default,
    };
    (style, Slot::from(text(line).font(Font::MONOSPACE)))
}))
.title("components/greeting.tsx")
.actions(button(icon::copy()).on_press(Message::CopyPatch))
```
