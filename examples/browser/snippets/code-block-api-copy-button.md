### CodeBlockCopyButton

The native recipe composes a ghost icon button, a tooltip, and the application clipboard task. The complete Header snippet shows all state and messages.

| API | Type | Default | Behavior |
| --- | --- | --- | --- |
| `Event::Copy(id, value)` | `String` | required | Exact text to copy; independent of visible clipping. |
| `button(...).disabled(bool)` | `bool` | `false` | Use while the source is incomplete. |
| `Event::Copied(id, at)` | `Instant` | on success | Show a check for two seconds; stale resets cannot clear a newer copy. |
