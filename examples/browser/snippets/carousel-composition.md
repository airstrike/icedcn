## Composition

```text
carousel(count, slide)
├── slide(index) → slide content
└── .controls(previous, next) → navigation button content
```

`carousel(count, slide)` owns the content viewport and controls. The factory returns each item through `Slot`; `.visible(n)` sets its basis, `.spacing(px)` sets the gutter, and `.controls(previous, next)` replaces the buttons' contents. Explicit child settings win over inherited typography; a disabled ancestor disables every descendant.
