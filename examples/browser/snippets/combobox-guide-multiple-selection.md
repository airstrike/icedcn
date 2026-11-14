## Multiple Selection

Use `.multiple(true)` for a set of choices. For a controlled combobox, pass the selected values to `.values(...)` and store the new selection in `update` when `.on_values_change(...)` publishes a message. Clearable choices can be removed directly from the input.
