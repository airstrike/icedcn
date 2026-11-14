## API Reference

| Builder | Purpose |
| --- | --- |
| `date_picker()` | Open a Calendar inside an anchored Popover. |
| `.value(selection)` / `.default_value(selection)` | Control or seed the selected dates. |
| `.on_value_change(f)` | Publish a message with `Some(selection)` when dates are selected or `None` when the selection is cleared. |
| `.mode(Mode::Range)` | Edit the endpoints across two months. The popover stays open until dismissed. |
| `.caption_dropdown(true)` | Jump to another year or month. |
| `.placeholder(text)` / `.label(formatter)` | Change trigger content and date formatting. |
| `.disabled(true)` | Disable the picker. |

Use `.close_on_select(false)` to keep a single-date calendar open after choosing a day. Escape or an outside press dismisses it. `.default_month(day)` sets the initial month when there is no selection. `.format(pattern)` formats a single date in the trigger, and `.chevron(false)` hides its trailing icon.

The editable `date_picker::input(placeholder)` accepts typed dates or a calendar selection. Down Arrow opens its calendar while editing. `.parse_with(date_picker::relative)` adds relative phrases including tomorrow, next Friday, and in 2 weeks. A partial edit stays visible while the calendar keeps the most recent valid date.
