## API Reference

### Construction

Use `accordion![(trigger, body), ...]` for fixed items and `accordion(items)` for an iterator. Both assign zero-based positions. A tuple accepts any trigger and body that convert to `View`. Use `accordion::item(trigger, body)` to add item-specific properties.

`.push(item)` appends one positional item. `.extend(items)` appends an iterator. `accordion![]` creates an empty accordion.

Use `accordion::keyed([(key, item), ...])` for records that can move, be inserted, or be removed. Keys retain their concrete Rust type and must be unique. The open selection, input contents, focus and animation follow each key. Keyed builders accept `.push(key, item)` and `.extend(keyed_items)`.

### Selection

An accordion starts closed and owns its selection by default. Single selection allows at most one item open. Use `.multiple()` before selection builders to allow several open items.

| Builder | Single selection | Multiple selection |
| --- | --- | --- |
| `.default_value(...)` | One `Key` | `IntoIterator<Item = Key>` |
| `.value(...)` | `Option<Key>` | `IntoIterator<Item = Key>` |
| `.on_value_change(...)` | `Fn(Option<Key>) -> Message` | `Fn(Vec<Key>) -> Message` |
| `.collapsible(false)` | Prevent clicking the open trigger closed | Available only on single selection |

`Key` is `usize` for positional items and the caller's key type for keyed items. A controlled accordion displays the supplied value and publishes the proposed change. An uncontrolled accordion seeds its default only on mount and retains subsequent user interaction across rebuilds. Missing keys and repeated multiple-selection values are ignored. Removed items lose their child state. Reintroduced keys start with fresh controls.

Call `.multiple()` before setting its change handler. It keeps the current value and defaults, and clears a previously configured single-selection handler.

### Appearance

| Builder | Description |
| --- | --- |
| `.style(style)` | Accepts a `theme::accordion` variant, `Spec` or style function. |
| `.padding(padding)` | Adds space around the accordion's items. |

### Item properties

| Builder | Description |
| --- | --- |
| `.disabled(bool)` | Disables the trigger. Its already-open body remains usable. |
| `.inset(pixels)` | Insets trigger and body horizontally while the divider keeps its full width. |
| `.chevron_closed(icon)` | Replaces the closed-state indicator. |
| `.chevron_open(icon)` | Replaces the open-state indicator. |
| `.content(body)` | Replaces the body supplied to the item constructor. |

The trigger and body participate in the parent’s `Compose` inheritance. Explicit child properties take precedence.
