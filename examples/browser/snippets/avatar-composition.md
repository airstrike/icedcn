## Composition

Use the following composition to build an `Avatar`:

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

In Rust, `avatar(image).fallback(content)` composes the image and fallback. `avatar::badge(avatar, content)` retains the badge on that Avatar, so it can be used inside `avatar::group([...])`. Append the count with `.count(content)`.
