## API Reference

| Builder | Purpose |
| --- | --- |
| `input_otp(length)` | Set the maximum code length. |
| `.group_size(count)` | Separate equal runs of slots with dashes. |
| `.pattern(Pattern::Digits)` | Accept digits only. Use `Alphanumeric` for letters and digits. |
| `.default_value(value)` | Seed an uncontrolled code. |
| `.value(value).on_value_change(handler)` | Read and write the code from application state. |
| `.disabled(true)` | Disable input, paste and keyboard focus. |
| `.invalid(true)` | Display an invalid border and focus ring. |
| `.cell_size(width,height)` | Set individual slot dimensions. |
