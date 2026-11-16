### Actions

Holds buttons at the end of the header. As a direct child of `code_block!`, it floats.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `code_block::actions(button)` | `impl Into<Slot>` | - | The first button. Buttons default to `Size::Xs`. |
| `.push(button)` | `impl Into<Slot>` | - | Another button, 4px apart. |
