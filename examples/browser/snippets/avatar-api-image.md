### AvatarImage

Pass an `image::Handle` to `avatar(...)`. Use `image::Handle::from_path`, `from_bytes`, or `from_rgba` to supply pixels through iced. The image covers its circular bounds without stretching. Pending or failed decoding leaves the fallback visible.

An image handle carries pixels. Display the user’s name next to the avatar when their identity needs to be explicit.
