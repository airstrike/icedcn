| align | Rust builder | Description |
|---|---|---|
| `start` | `.align(Horizontal::Left)` | Align the bubble to the start of the conversation. |
| `end` | `.align(Horizontal::Right)` | Align the bubble to the end of the conversation. |

**Note:** When building chat interfaces, you probably want to use alignment on the `Message` component itself, not the `Bubble` component. You can use the `role` prop on the `Message` component to automatically align the bubble to the start or end of the conversation.

In Rust, use `message(content).align(...)`; a nested bubble inherits this alignment unless it has an explicit `.align(...)`.
