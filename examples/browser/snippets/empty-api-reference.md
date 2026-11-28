## API Reference

### `empty::empty()`

A centered empty state with 24px padding and 16px between sections. Set its title, description, media and content with the builder methods below.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.media(media)` | `impl Into<View>` | `None` | The media above the title, usually `empty::media(..)`. |
| `.title(title)` | `&str`, `Text` or `View` | `None` | Strings are styled as `EmptyTitle`. |
| `.description(description)` | `&str`, `Text` or `View` | `None` | Strings are styled as `EmptyDescription`. |
| `.content(content)` | `impl Into<View>` | `None` | Actions below the header, at most 384px wide. |
| `.padding(padding)` | `impl Into<Padding>` | `24` | Override the root padding. |

### `empty::media()`

Unframed by default. `.style(empty::Icon)` puts the content on a 32px muted tile and sizes icons at 16px.
