## API Reference

Typography is a collection of helper constructors and two composed
widgets, not a single component. Each helper returns a `Text<'a>` (or
`Blockquote` / `List`) builder you can chain `.size(...)`, `.bold()`,
etc. on.

### Constructors

| Constructor | Returns | Description |
| --- | --- | --- |
| `text::h1(content)` | `Text<'a>` | 30px, semibold, `-0.025em` letter-spacing, foreground. |
| `text::h2(content)` | `Text<'a>` | 20px, medium, `-0.025em` letter-spacing, foreground. |
| `text::h3(content)` | `Text<'a>` | 18px, medium, `-0.025em` letter-spacing, foreground. |
| `text::h4(content)` | `Text<'a>` | 16px, medium, `-0.025em` letter-spacing, foreground. |
| `text::p(content)` | `Text<'a>` | Body paragraph: 16px, foreground. |
| `text::lead(content)` | `Text<'a>` | Lead paragraph: 20px, muted. |
| `text::large(content)` | `Text<'a>` | 18px, semibold, foreground. |
| `text::small(content)` | `Text<'a>` | 14px, foreground. |
| `text::muted(content)` | `Text<'a>` | 14px, muted. |
| `text::xs(content)` | `Text<'a>` | 12px, foreground. |
| `text::inline_code(content)` | `Text<'a>` | 14px, semibold, monospace. Long names wrap within the available width. Wrap in a container with a muted background at the call site for the full inline-code chrome. |
| `text::blockquote(content)` | `Blockquote<'a>` | 2px left rule + 24px gap + italic body. |
| `text::list(items)` | `List<'a, Message>` | Bulleted list with a 24px marker column, hanging indent and 8px between rows. |
| `text(content)` | `Text<'a>` | Crate-root shorthand for `text::small`. |

`content` is `impl IntoFragment<'a>` (`&str`, `String`, `Cow<'a, str>`),
so all constructors accept owned or borrowed strings.

The Typography examples use a larger article scale than the control-oriented heading defaults. Their h1 is 36/40px and extra-bold. The next levels are semibold at 30/36, 24/32 and 20/28px. The isolated h1 centers its balanced lines. The article title is left aligned.

### `Text<'a>` builder methods

| Builder | Type | Description |
| --- | --- | --- |
| `.size(size)` | `impl Into<Size>` | Override the font size. Accepts `Size::Xs`/`Sm`/`Default`/`Lg`, an `f32` (px), or `Size::Custom(v)`. |
| `.weight(weight)` | `impl Into<Weight>` | Override the font weight. Accepts `Weight::Medium`/`Semibold`/`Bold`/etc. or a numeric `u16`. |
| `.font(font)` | `iced::Font` | Select a font family. |
| `.line_height(height)` | `iced::Pixels` or relative multiplier | Set the line box independently of font size. |
| `.wrapping(strategy)` | `text::Wrapping` | Choose a line-breaking strategy, including balanced headlines. |
| `.medium()` | — | Shorthand for `.weight(Weight::Medium)`. |
| `.semibold()` | — | Shorthand for `.weight(Weight::Semibold)`. |
| `.bold()` | — | Shorthand for `.weight(Weight::Bold)`. |
| `.italic()` | — | Render the text in italic (folded into the active font). |
| `.letter_spacing(spacing)` | `impl Into<iced_core::Em>` | Adjust letter spacing in `em` units. |
| `.width(width)` | `impl Into<Length>` | Override the layout width. Defaults to `Shrink`, matching `Text::new()`; add `.width(Fill)` for paragraph copy that should occupy the parent's full width. |
| `.style(style)` | `impl Into<text::StyleFn>` | Override the text style — variant unit structs (`theme::text::Foreground`, `Muted`, `Link`, `Destructive`), a `Spec` (`Foreground + Alpha(0.5)`), or a hand-rolled closure. |

### `Blockquote<'a>`

Composed widget: 2px left rule + 24px gap + italic body in a single
horizontal row. Set the space above and below it with the enclosing column’s `.spacing(...)`.

| Builder | Type | Description |
| --- | --- | --- |
| `.size(size)` | `impl Into<Size>` | Override the body text size. Defaults to 16px (`size::BASE`). |
| `.style(style)` | `impl Into<text::StyleFn>` | Override the body text style. Defaults to `theme::text::Foreground`. |

### `List<'a, Message>`

Each item has a separate bullet and a 24px marker column, so wrapped lines align with the item text. Rows are spaced 8px apart. Set vertical spacing around the list on its parent.

The `text::list(items)` constructor takes any
`IntoIterator<Item = impl IntoFragment<'a>>`. Empty input produces an
empty list (no panic). Items are rendered at body size + foreground
color and have no further builder methods today.

### `text::size` constants

Raw `f32` constants for the type scale, exposed
for call sites that want a literal pixel size (e.g. `Span::size`).

| Constant | Value (px) |
| --- | --- |
| `text::size::XS` | 12 |
| `text::size::SM` | 14 |
| `text::size::BASE` | 16 |
| `text::size::LG` | 18 |
| `text::size::XL` | 20 |
| `text::size::XL2` | 24 |
| `text::size::XL3` | 30 |
| `text::size::XL4` | 36 |

### `Size`

| Variant | `text_size()` (px) | Notes |
| --- | --- | --- |
| `Size::Xs` | 12 | Extra-small text. |
| `Size::Sm` | 12.8 | Small text. |
| `Size::Default` | 14 | Default body size. |
| `Size::Lg` | 14 | Same as `Default` — drives container height, not text size. |
| `Size::Custom(v)` | `v` | Arbitrary pixel size. `From<f32>` / `From<u16>` / `From<i32>` blanket impls let you pass numerics directly. |

### `Weight`

`Thin`, `ExtraLight`, `Light`, `Normal`, `Medium`, `Semibold`, `Bold`,
`ExtraBold`, `Black`, `Custom(u16)`. The numeric variant clamps to the
nearest supported font weight (100–900).

### Rich text — `rich_text!` + `Span<'a>`

Build inline runs of differently-styled text without resorting to a
`row![text, text]` hack. Each `Span` carries an optional theme-aware
color `Token` resolved against the active palette at draw time, so the
same widget renders correctly in light and dark modes.

```rust
use icedcn::rich_text;
use icedcn::widget::{rich_text::span, text};
use icedcn::theme::tweak::Token;

rich_text![
    span("Required Field ").size(text::size::SM).medium(),
    span("*").size(text::size::SM).color(Token::Destructive),
]
```

`rich_text![span(...), span(...)]` mirrors `row!` / `column!`. Bare
string literals work too — `From<&str>` and `From<String>` for `Span`
let you write `rich_text!["plain ", span("bold").bold()]`. The
`+ Token` operator overload (`span("hi") + Token::Destructive`) is
shorthand for `.color(...)`.

#### `Span<'a>` builder methods

| Builder | Type | Description |
| --- | --- | --- |
| `.size(size)` | `impl Into<Size>` | Override the span's font size. |
| `.weight(weight)` | `impl Into<Weight>` | Override the span's font weight. |
| `.font(font)` | `iced::Font` | Select a font family. |
| `.line_height(height)` | `iced::Pixels` or relative multiplier | Set the line box independently of font size. |
| `.wrapping(strategy)` | `text::Wrapping` | Choose a line-breaking strategy, including balanced headlines. |
| `.medium()` | — | Shorthand for `.weight(Weight::Medium)`. |
| `.semibold()` | — | Shorthand for `.weight(Weight::Semibold)`. |
| `.bold()` | — | Shorthand for `.weight(Weight::Bold)`. |
| `.italic()` | — | Render the span in italic. |
| `.line_height(line_height)` | `impl Into<text::LineHeight>` | Override the line height. |
| `.letter_spacing(letter_spacing)` | `impl Into<iced_core::Em>` | Adjust letter spacing. |
| `.color(token)` | `impl Into<Token>` | Theme-aware color — resolves against the palette at draw time. |

#### `RichText<'a>` builder methods

| Builder | Type | Description |
| --- | --- | --- |
| `.size(size)` | `impl Into<Size>` | Default font size for spans that don't set their own. |
| `.line_height(line_height)` | `impl Into<text::LineHeight>` | Line height for the whole rich text. |
| `.width(width)` | `impl Into<Length>` | Layout width. Defaults to `Length::Shrink`. |
| `.height(height)` | `impl Into<Length>` | Layout height. Defaults to `Length::Shrink`. |
| `.align_x(alignment)` | `impl Into<text::Alignment>` | Horizontal alignment. |
| `.align_y(alignment)` | `impl Into<alignment::Vertical>` | Vertical alignment. |
| `.wrapping(wrapping)` | `text::Wrapping` | Wrapping strategy. |
| `.letter_spacing(letter_spacing)` | `impl Into<iced_core::Em>` | Letter spacing for the whole rich text. |
| `.style(style)` | `impl Into<text::StyleFn>` | Default catalog class for spans without their own `color`. |

### Style variants

Variant unit structs you pass to `.style(...)` on `Text`, `Blockquote`,
or `RichText`:

| Variant | Description |
| --- | --- |
| `theme::text::Default` | Inherit from parent — no override. |
| `theme::text::Foreground` | Body foreground color. |
| `theme::text::Muted` | Secondary / muted text color. |
| `theme::text::Link` | Primary brand color (link text). |
| `theme::text::Destructive` | Destructive / error color. |

Compose with tweak atoms via `+`: `Foreground + Alpha(0.5)`,
`Default + Fg(Token::Primary)`, etc.
