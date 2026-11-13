```rust
let route = r##"import { streamText } from "ai"

export async function POST(request: Request) {
  const { messages } = await request.json()
  const result = streamText({ model: "anthropic/claude-sonnet-4.5", messages })

  return result.toUIMessageStreamResponse()
}"##;
let chat = r##""use client"

import { useChat } from "@ai-sdk/react"

export function Chat() {
  const { messages } = useChat()

  return <MessageList messages={messages} />
}"##;

tabs(|t| column![
    t.list([t.trigger("route", "route.ts"), t.trigger("chat", "chat.tsx")]),
    t.content("route", code_block(route).title("app/api/chat/route.ts").start_line(1)),
    t.content("chat", code_block(chat).title("components/chat.tsx").start_line(1)),
].into()).default_value("route")
```
