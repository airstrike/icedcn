### Root

Use `collapsible(body)` to create an iced `Component` that owns its open state. The closure receives a `collapsible::Handle` and returns a `View` in your application's message type.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.default_open(bool)` | `bool` | `false` | Seed the uncontrolled state once. |
| `.open(bool)` | `bool` | uncontrolled | Show the supplied state; interaction requests a change. |
| `.on_open_change(f)` | `Fn(bool) -> Message` | `None` | Publish the complete requested open state. A callback does not make the widget controlled. |
| `.disabled(bool)` | `bool` | `false` | Disable every trigger. Controls in already-open content remain usable. |
| `.width(width)` | `impl Into<Length>` | `Shrink` | Width of the root. |

`c.is_open()` exposes the current state and `c.is_disabled()` exposes the root/ancestor trigger-disabled state. An inherited disabled ancestor disables all descendants. Inherited typography reaches the content closure.

[Read the collapsible implementation](https://github.com/airstrike/icedcn/blob/HEAD/widget/src/collapsible.rs).
