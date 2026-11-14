## Composition

```text
collapsible(|c| ...)
├── c.trigger(child)
└── c.content(child)
```

The closure receives the handle: `c.trigger(child)` marks a trigger and `c.content(child)` mounts the expandable content. The child keeps its own handlers.
