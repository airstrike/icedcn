## API Reference

| Builder | Behavior |
|---|---|
| `attachment(title)` | Creates a file card. |
| `.description(text)`, `.media(slot)` | Adds metadata and an image or icon. |
| `.state(state)` | Idle, Uploading, Processing, Error or Done. |
| `.vertical(true)`, `.size(size)`, `.width(length)` | Sets the layout and density. |
| `.trigger(f)` | Wraps the main target independently of file actions. |
| `.action(slot)` | Appends a trailing or overlaid action. |
| `attachment::group(cards)` | Provides a native horizontal viewport. |
