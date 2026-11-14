### Label the trigger

Keep the filename and an instruction such as “Open preview dialog” visible in the card content. Use `attachment::trigger(target)` for the keyboard-focusable full-card target. Keep the remove and copy buttons in `.action(...)` so activating either performs only its own action.

For an icon-only action, add a tooltip that names the action.

In the Image example, `Message::Open(url)` carries the exact image URL. Handle it in the application’s `update` function by opening that URL. The installed browser routes it through `Message::OpenUrl`. Copy and remove have separate messages and do not activate the preview.
