```rust
let source = r##"$ pnpm lint
apps/web/components/message-list.tsx
  18:22  error  'messages' is possibly 'undefined'. Add a guard before calling map  @typescript-eslint/no-unsafe-call
  42:7   warning  React Hook React.useEffect has a missing dependency: 'scrollToBottom'  react-hooks/exhaustive-deps

✖ 2 problems (1 error, 1 warning)"##;

code_block(source)
    .title("Terminal")
    .wrap(self.wrap)
    .actions(row![
        button(icon::wrap_text()).on_press(Message::ToggleWrap),
        button(icon::download()).on_press(Message::SaveLog),
        button(icon::copy()).on_press(Message::CopyLog),
    ])
```
