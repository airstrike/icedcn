## API Reference

Set the active question with `item` and `on_item_change`, or use `default_item` for local navigation. `answers` and `on_answers_change` control responses. Use `default_answers` and `reset_action` to restore a draft.

`enabled_when` removes an inapplicable item from progress, validation and submission. `validate` can return an `Invalid` with an item name and message. On submission, `.on_submit(...)` publishes a message with the choices, freeform text and explicit skip status.
