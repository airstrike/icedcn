| align | Rust builder | Description |
|---|---|---|
| `start` | `.align(Horizontal::Left)` | Align the bubble to the start of the conversation. |
| `end` | `.align(Horizontal::Right)` | Align the bubble to the end of the conversation. |

Use `message(content).align(...)`; a nested bubble inherits this alignment unless it has an explicit `.align(...)`.
