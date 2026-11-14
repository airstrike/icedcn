## Composition

Use the following composition to build a bubble:

```text
Bubble
├── BubbleContent
└── BubbleReactions
```

In Rust, `bubble(content)` takes the content `Slot`, and `.reactions(content, align, side)` attaches the reaction `Slot`.

Use `BubbleGroup` to group consecutive bubbles from the same sender:

```text
BubbleGroup
├── Bubble
│   └── BubbleContent
└── Bubble
    └── BubbleContent
```

`bubble::group([...])` composes the group. Set `.align(...)` on each bubble. The surrounding Message can also supply the default alignment.
