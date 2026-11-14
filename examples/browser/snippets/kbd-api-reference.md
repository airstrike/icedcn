## API Reference

### `kbd()`

A keyboard key: 12px medium-weight text on a 20px muted keycap.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.style(style)` | `impl Into<container::Spec>` | `kbd::Default` | `kbd::Tooltip` for keys inside tooltip content, or a tweaked `Spec`. |

### `kbd_group!` / `kbd::group()`

Lays keys (and any other children) out in a row with a 4px gap.

Use `.style(kbd::Tooltip)` when a key appears inside a tooltip.
