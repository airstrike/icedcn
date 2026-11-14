## Composition

The title is required. Builders supply the media, description, actions, and optional activation target.

```text
attachment(title)
├── .media(icon or image)
├── content
│   ├── title
│   └── .description(metadata)
├── .action(button)
└── .trigger(build_target)
```

Place several cards inside a scrollable row with `attachment::group`.

```text
attachment::group
├── attachment(title)
└── attachment(title)
```
