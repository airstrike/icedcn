```rust
let source = r##"export default {
  experimental: {
    reactCompiler: true,
    viewTransition: true,
  },
  images: {
    remotePatterns: [{ hostname: "avatars.githubusercontent.com" }],
  },
}"##;

CodeBlock::lines(source.lines().enumerate().map(|(index, line)| {
    let style = if [2, 3].contains(&index) { LineStyle::Highlighted } else { LineStyle::Default };
    (style, Slot::from(text(line).font(Font::MONOSPACE)))
})).start_line(1)
```
