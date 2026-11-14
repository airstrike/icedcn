### AvatarImage

The `AvatarImage` component displays the avatar image.

Pass `Some(image::Handle)` to `avatar(...)`. Use `image::Handle::from_path`, `from_bytes`, or `from_rgba` to supply pixels through iced. The image covers its circular bounds without stretching. Pending or failed decoding leaves the fallback visible.

The web `src` maps to the handle. HTML `alt` has no Avatar accessibility equivalent in this pinned iced API; a handle does not carry alternative text. Provide a visible user name alongside the avatar when the identity must be conveyed independently of the image.
