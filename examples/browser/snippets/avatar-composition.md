## Composition

Give `avatar` an image handle and a fallback. Use `avatar::badge` to place a status mark on the portrait and `avatar::group` to arrange portraits together.

```text
Avatar
├── AvatarImage
├── AvatarFallback
└── AvatarBadge
```

Use the following composition to build an `AvatarGroup`:

```text
AvatarGroup
├── Avatar
│   ├── AvatarImage
│   ├── AvatarFallback
│   └── AvatarBadge
├── Avatar
│   ├── AvatarImage
│   ├── AvatarFallback
│   └── AvatarBadge
└── AvatarGroupCount
```
