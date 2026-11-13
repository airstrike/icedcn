```rust
let source = r##"import { streamText } from "ai"

export async function POST(request: Request) {
  const { messages } = await request.json()

  const result = streamText({
    model: "anthropic/claude-sonnet-4.5",
    messages,
  })

  return result.toUIMessageStreamResponse()
}"##;

column![
    text::label("How do I stream a response?"),
    text::p("Create a route handler that returns the stream:"),
    code_block(source).title("app/api/chat/route.ts"),
    text::p("The client reads it with useChat."),
].spacing(12)
```
