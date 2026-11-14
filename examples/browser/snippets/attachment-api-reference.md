### Attachment

`attachment(title)` creates the file card. The filename accepts `impl Into<String>`.

| Builder | Type | Default | Behavior |
| --- | --- | --- | --- |
| `.state(...)` | `attachment::State` | `Done` | Selects the upload lifecycle styling and title shimmer. |
| `.size(...)` | `impl Into<Size>` | `Default` | Selects default, small, or extra-small density. |
| `.vertical(...)` | `bool` | `false` | Places the media above the content when true. |
| `.width(...)` | `impl Into<Length>` | `Fill` horizontally, 120 px vertically | Sets the width of the card. |
| `.style(...)` | `Fn(&Theme) -> container::Style` | Card surface | Customizes the background, border, radius and text color. |
