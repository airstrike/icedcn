## Composition

### Simple

Pass a list of strings to `combobox(items)`. The widget owns the search field and filtered list. Use `.empty(message)` to set the text shown when no options match.

### With chips

Enable `.multiple(true)` to show selected values as removable chips. Seed the selection with `.default_values(items)` or use `.values(items)` with `.on_values_change(handler)` to keep the selection in the application.

### With groups

Pass `combobox::item(value, label).group(name)` entries to `combobox::options(items)` to display related options under the same label. Selection publishes the chosen value.
