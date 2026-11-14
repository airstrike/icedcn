### Copy button

Use `.copyable(true)` to copy the full source, including collapsed lines. Diff markers, line numbers and ANSI escapes are omitted. For custom line slots or a different payload, pass `code_block::copy_button(text)` to `.actions(...)`. It copies the complete text and shows a check and “Copied” tooltip for two seconds after a successful write. Each subsequent successful copy restarts that interval.

Use `.disabled(true)` to disable the button while a transcript is incomplete.
