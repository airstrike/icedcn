## API Reference

See the [Radix UI Separator](https://www.radix-ui.com/docs/primitives/components/separator#api-reference) documentation.

`separator()` creates a 1px border-colored line. Horizontal lines span the width available from their parent; vertical lines stretch to the height of their row.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.orientation(orientation)` | `impl Into<Orientation>` | `Horizontal` | Use `Horizontal` or `Vertical`. |

The separator is passive: it does not receive keyboard focus, capture pointer presses, or publish messages. Compose it directly with `column!` or `row!`; it needs no application state.

Radix's `decorative` and `asChild` props have no corresponding Rust builders. This native separator renders a visual rule; it does not expose Radix's DOM separator role or ARIA orientation. Size and spacing come from the surrounding layout.
