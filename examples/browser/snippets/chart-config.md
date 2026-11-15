## Chart Config

`bar(values)` creates a native Hyozu series. `.with_name("Desktop")` supplies its legend and tooltip name; `.with_color(color!(0x2563eb))` supplies its fill. `bars([desktop, mobile])` groups the series by category. `.data_labels(None)` hides labels on individual bars.

`data(marks)` prepares the chart. Set categories with `.x_axis_labels(...)`, configure each axis with `.x_axis(...)` or `.y_axis(...)`, and supply `.legend(...)` or `.tooltip(...)` as needed. Keep the resulting `Data` in your model; replace or update it when the underlying values change.
