```rust
let source = r##"{
  "location": "San Francisco",
  "temperature": 18,
  "unit": "celsius",
  "conditions": ["fog", "wind"]
}"##;

let mut parser = Parser::new(&Settings { token: "json".into() });
let lines = source.lines().map(|line| {
    let spans = parser.parse_line(line).map(|(range, class)| {
        let value = span(&line[range]);
        match class {
            Code::String => value.color(Token::Success),
            Code::Constant => value.color(Token::Info),
            _ => value,
        }
    });
    (LineStyle::Default, Slot::from(rich_text(spans).font(Font::MONOSPACE)))
});
CodeBlock::lines(lines).title("getWeather output")
```
