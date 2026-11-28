### Actions

Holds buttons at the end of the header. Passed to the block's `.actions(..)`, it floats.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `code_block::actions(button)` | `impl Into<View>` | - | The first button. Buttons default to `Size::Xs`. |
| `.push(button)` | `impl Into<View>` | - | Another button, 4px apart. |
| `code_block::actions![..]` | - | - | Buttons of different types. |
