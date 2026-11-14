### AvatarGroup

The `AvatarGroup` component displays a group of avatars with overlapping styling.

`avatar::group(avatars)` accepts an iterator of Avatar builders, including the result of `avatar::badge(...)`. Each portrait keeps its original diameter and gets a 2px outer background ring. Adjacent avatars overlap by 8px; later members draw above earlier ones. The group forwards inherited properties into its members and count.
