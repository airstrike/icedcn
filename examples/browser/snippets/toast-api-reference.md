## API Reference

### `toaster()`

`toaster(content)` shows the toasts raised inside `content`, usually the whole app, over the window. Each toast leaves by itself after its duration: the toaster asks the runtime for a redraw at the next expiry, so no subscription is needed.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.position(position)` | `toast::Position` | `BottomRight` | Where toasts stack. |
| `.duration(duration)` | `Duration` | 4s | How long each toast shows. |
| `.icons(f)` | `Fn(toast::Kind) -> Option<Slot>` | 16px status icons | Override the status icons. Return `None` for a kind to leave it unadorned. |

### `toast::toast(title)`

| Builder | Type | Description |
| --- | --- | --- |
| `.description(text)` | `impl Into<String>` | Supporting text below the title. |
| `.kind(kind)` | `toast::Kind` | `Default`, `Loading`, `Success`, `Info`, `Warning` or `Error`. Enable the `spinner` feature to animate the loading indicator. |
| `.action(label, message)` | `impl Into<String>`, `Message` | An action button: publishes `message` and dismisses the toast. |
| `.position(position)` | `toast::Position` | Stack this toast elsewhere than the toaster's position. |
| `.duration(duration)` | `Duration` | Show this toast for longer or shorter. |

### Raising a toast

- `toast::trigger(child, toast)`: pressing `child` raises `toast`.
- `toast::show(toast)`: a `Task` raising `toast`, for `update` (after a save completes, say).

- `toast::promise(loading, future, success, error)` shows a spinner while the future is pending. Its result replaces the pending card and starts the normal dismissal timer.
