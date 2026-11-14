## API Reference

### `checkbox()`

Constructor for an unchecked, uncontrolled `Checkbox`. The value lives in the widget tree unless the app controls it.

**`.default_checked(state)`** accepts `impl Into<State>`. Default: `Unchecked`.
Sets the initial state of an uncontrolled checkbox.

**`.checked(state)`** accepts `impl Into<State>`. Default: uncontrolled.
Controls the checkbox: it always shows `state`.

**`.on_checked_change(f)`** accepts `Fn(bool) -> Message`. Default: `None`.
Publishes a message with the new checked state on each toggle in either mode.

**`.disabled(bool)`** accepts `bool`. Default: `false`.
Disables the checkbox. A disabled ancestor (a disabled field) disables it too.

**`.label(label)`** accepts `impl IntoFragment`. Default: `None`.
Renders an optional inline label next to the box.

**`.width(width)`** accepts `impl Into<Length>`. Default: `Length::Shrink`.
Sets the outer width.

**`.style(style)`** accepts a value convertible to
`theme::checkbox::StyleFn<'a, Theme>`. Default: `theme::checkbox::Default`.
Use the variant unit structs `checkbox::Default`, `checkbox::Invalid`,
`checkbox::Disabled`, or a `theme::checkbox::Spec`.

### `checkbox::State`

- `State::Checked` renders the check glyph.
- `State::Unchecked` renders an empty box.
- `State::Indeterminate` renders a horizontal stroke. A click commits it to checked.

`From<bool> for State` maps `true` to `Checked` and `false` to `Unchecked`.

### Labels

Inside `field(checkbox()).label("...")`, pressing the label toggles the checkbox through the field’s `activate` operation. With `.card()` the whole card is the label, and its tint follows the checkbox.

### Validation

Apply `theme::checkbox::Invalid` via `.style(...)` to land the destructive border and persistent invalid ring.

### Keyboard interaction

Tab and Shift-Tab move between enabled controls. Space toggles on release; repeated key-down events produce one change. Enter does not toggle. Disabled controls ignore pointer, keyboard, and linked-label activation, including when disabled through an ancestor. Pointer focus does not show the keyboard focus ring.

The 16px control keeps an expanded pointer target without increasing its layout size. Pressing inside and releasing outside cancels the change. `Disabled` is a style variant; use `.disabled(true)` to prevent interaction.
