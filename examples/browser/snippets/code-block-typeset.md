```rust
let source = r##"import { streamText } from "ai"

export async function POST(request: Request) {
  const { messages } = await request.json()
  const result = streamText({ model: "anthropic/claude-sonnet-4.5", messages })

  return result.toUIMessageStreamResponse()
}"##;

column![
    text::h3("Stream a response"),
    text::p("Create a route handler that calls streamText and returns the stream to the client."),
    code_block(source).title("app/api/chat/route.ts").start_line(1),
    text::p("The useChat hook reads the stream on the client and renders the answer while it is written."),
].spacing(12)
```
