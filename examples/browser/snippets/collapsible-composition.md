## Composition

Use the following composition to build a `Collapsible`:

```text
Collapsible
├── CollapsibleTrigger
└── CollapsibleContent
```

In Rust, the closure receives the handle: `c.trigger(child)` marks a trigger and `c.content(child)` mounts the expandable content. The child keeps its own handlers.
