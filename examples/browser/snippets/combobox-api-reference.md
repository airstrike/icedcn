## API Reference

`combobox(items)` creates a searchable input from string labels. `combobox::options(items)` accepts `combobox::item(value, label)` entries; `.description(text)` adds secondary text and `.group(label)` assigns a heading. Custom rows use `.format_item(|item| ...)` and return any content accepted by `View`. These children receive inherited properties before composition, and their application messages still fire.

The widget owns query, open state and uncontrolled selection. Defaults are seeded once. Controlled widgets always display the supplied value; interactions publish the proposed new value. Disabled ancestors disable inputs, triggers, chips and results.

| Builder | Type | Default | Behavior |
| --- | --- | --- | --- |
| `.placeholder(text)` | `Into<String>` | `""` | Input placeholder; popup search placeholder. |
| `.empty(text)` | `Into<String>` | `"No items found."` | Empty result message. |
| `.default_value(value)` | `Into<String>` | none | Initial single selection. |
| `.value(value)` | `Into<String>` | uncontrolled | Controlled single selection. |
| `.on_value_change(f)` | `Fn(String) -> Message` | none | Complete selected value; clearing sends an empty string. |
| `.multiple(enabled)` | `bool` | `false` | Render removable chips and keep the list open after selection. |
| `.default_values(values)` | iterable of strings | empty | Initial multiple selection; enables multiple mode. |
| `.values(values)` | iterable of strings | uncontrolled | Controlled multiple selection; enables multiple mode. |
| `.on_values_change(f)` | `Fn(Vec<String>) -> Message` | none | Complete updated selection after add/remove. |
| `.auto_highlight(enabled)` | `bool` | `false` | Highlight the first matching result on open/filter. |
| `.clearable(enabled)` | `bool` | `false` | Show an X while a single value is selected. |
| `.invalid(enabled)` | `bool` | `false` | Invalid border and ring. |
| `.disabled(disabled)` | `bool` | `false` | Disable all interaction. |
| `.text_size(px)` | `f32` | 14 | Inline and popup input text size. Chips use 14px. |
| `.width(length)` | `Into<Length>` | 20ch inline; 256px popup | Field width, constrained by available space. |
| `.icon(f)` | `Fn() -> Into<View>` | built-in Lucide chevron | Trigger icon. |
| `.leading_icon(f)` | `Fn() -> Into<View>` | none | Leading addon; `combobox::globe` is provided. |
| `.format_item(f)` | `Fn(&Item) -> Into<View>` | label/description | Custom option content. |
| `.popup(enabled)` | `bool` | `false` | Put search inside the popup. |
| `.trigger(f)` | `Fn(&str) -> Into<View>` | outline button | Custom popup trigger, receiving the selected label. |
| `.content_width(px)` | `Into<Pixels>` | anchor width | Independent popup width. Fixed-width popup buttons allow 28px extra for the inset search. |
| `.align_offset(px)` | `Into<Pixels>` | `0` | Horizontal popup shift. |
| `.separators(enabled)` | `bool` | `true` | Separate consecutive groups. |

Filtering ignores letter case and accents in the option label. Up/Down move through matching options and Enter selects. Home/End reach the first/last result while open. Escape dismisses and restores focus; outside click and Tab dismiss without stealing focus. With an empty multiple input, Backspace removes the last chip. Left and Right move between chips, and Delete removes the focused chip. The focused result scrolls into view.

[Read the combobox implementation](https://github.com/airstrike/icedcn/blob/HEAD/widget/src/combobox.rs).
