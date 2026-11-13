```rust
let source = r##"export function useDebounce<T>(value: T, delay = 300) {
  const [debounced, setDebounced] = React.useState(value)

  React.useEffect(() => {
    const timeout = setTimeout(() => setDebounced(value), delay)
    return () => clearTimeout(timeout)
  }, [value, delay])

  return debounced
}"##;

let visible = source.char_indices().nth(self.revealed).map_or(source.len(), |(i, _)| i);
column![
    code_block(&source[..visible]).title("hooks/use-debounce.ts").start_line(1),
    button("Replay").on_press(Message::Replay),
].spacing(12)
```
