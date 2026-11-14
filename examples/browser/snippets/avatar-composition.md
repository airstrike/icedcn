## Composition

```text
avatar(Some(image))
└── .fallback(initials)
```

```text
avatar::group(avatars)
├── avatar(Some(image)).fallback(initials)
├── avatar::badge(avatar, badge)
└── .count(content)
```

`avatar(image).fallback(content)` composes the image and fallback. `avatar::badge(avatar, content)` retains the badge on that Avatar, so it can be used inside `avatar::group([...])`. Append the count with `.count(content)`.
