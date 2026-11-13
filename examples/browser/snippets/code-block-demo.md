```rust
let source = r##"import { streamText } from "ai"

export async function POST(request: Request) {
  const { messages } = await request.json()

  const result = streamText({
    model: "anthropic/claude-sonnet-4.5",
    system: "You are a helpful assistant. Answer in one short paragraph.",
    messages,
  })

  return result.toUIMessageStreamResponse()
}"##;

let wrap = self.wrap;
let actions = row![
    button(icon::wrap_text()).on_press(Message::ToggleWrap),
    button(icon::download()).on_press(Message::SaveFile),
    button(icon::copy()).on_press(Message::CopySource),
];
code_block(source)
    .title("app/api/chat/route.ts")
    .actions(actions)
    .start_line(1)
    .wrap(wrap)
```
