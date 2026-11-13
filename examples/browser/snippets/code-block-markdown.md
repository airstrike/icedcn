```rust
let source = r##"export function useDebounce<T>(value: T, delay = 300) {
  const [debounced, setDebounced] = React.useState(value)

  React.useEffect(() => {
    const timeout = setTimeout(() => setDebounced(value), delay)
    return () => clearTimeout(timeout)
  }, [value, delay])

  return debounced
}"##;

column![
    text::label("How do I debounce a search input in React?"),
    text::p("Wrap the value in a useDebounce hook."),
    code_block(source).title("tsx").wrap(true),
    text::p("Pass the debounced value to your fetch instead of the raw input."),
    button("Replay").on_press(Message::Replay),
].spacing(12)
```
