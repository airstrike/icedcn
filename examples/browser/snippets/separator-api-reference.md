## API Reference

[Read the separator implementation](https://github.com/airstrike/icedcn/blob/HEAD/widget/src/separator.rs).

`separator()` creates a 1px border-colored line. Horizontal lines span the width available from their parent; vertical lines stretch to the height of their row.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.orientation(orientation)` | `impl Into<Orientation>` | `Horizontal` | Use `Horizontal` or `Vertical`. |

The separator is passive: it does not receive keyboard focus, capture pointer presses, or publish messages. Compose it directly with `column!` or `row!`; it needs no application state.

Set the size and spacing with the surrounding layout.
