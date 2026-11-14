### Copy button

Pass `code_block::copy_button(text)` to `.actions(...)` or put it in a row with other controls. It copies the complete text and shows a check and “Copied” tooltip for two seconds after a successful write. Each subsequent successful copy restarts that interval.

Use `.disabled(true)` to disable the button while a transcript is incomplete. For ANSI output, copy `output.as_str()` to omit color escapes.
