## Composition

```text
bubble(content)
└── .reactions(content, align, side)
```

`bubble(content)` takes the content `View`, and `.reactions(content, align, side)` attaches the reaction `View`.

Group consecutive bubbles from the same sender:

```text
bubble::group(items)
├── bubble(content)
└── bubble(content)
```

`bubble::group([...])` composes the group. Set `.align(...)` on each bubble. The surrounding Message can also supply the default alignment.
