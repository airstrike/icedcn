## Trace

Put a `CodeBlock` with `size="sm"` in `TraceCollapsibleContent` for tool and terminal output. Add `wrap` and a `max-h-64` class to `CodeBlockContent` so long output wraps and scrolls inside the trace. The block sets its own text color, so `TraceCollapsibleContent` does not mute it.

The Rust recipe uses `trace::disclosure`, `.compact(true)`, `.wrap(true)` and `.max_height(256.)`.
