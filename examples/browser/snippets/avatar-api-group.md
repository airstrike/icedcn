### AvatarGroup

`avatar::group(avatars)` accepts an iterator whose items convert into Avatar builders, including the result of `avatar::badge(...)`. Each portrait keeps its original diameter and gets a 2px outer background ring. Adjacent avatars overlap by 8px; later members draw above earlier ones. Use `avatar::group![...]` for fixed children; it accepts image handles and Avatar builders together. The group forwards inherited properties into its members and count.
