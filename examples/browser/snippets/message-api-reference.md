## API Reference

| Builder | Behavior |
|---|---|
| `message(content)` | Creates a conversation turn. |
| `.align(horizontal)` | Places the avatar and content on the sender’s side and passes the alignment to child bubbles. |
| `.avatar(slot)` | Reserves a 32-point avatar lane. |
| `.header(slot)`, `.footer(slot)` | Adds identity, time, delivery status or actions. |
| `message::group(items)` | Groups turns with eight points of spacing. |
