## Legend

`.legend(legend::Config::below())` displays series names and swatches below the plot. Add `.interactive(true)` to let users click a legend entry to hide or show that series. Hidden-series state lives in the chart's widget tree.

## API Reference

| API | Purpose |
| --- | --- |
| `data(marks)` | Prepare native Hyozu data in the model. |
| `bar(values).with_name(name)` | Build a named bar series. |
| `bars(series)` | Group multiple bar series. |
| `chart(&data)` | Borrow data into a native chart; fills its available size. |
| `.height(height)` | Reserve a visible chart height. |
| `.x_axis(...)`, `.y_axis(...)` | Configure labels, bounds, lines and grid on the data. |
| `.show_tooltip(bool)` | Enable or disable data inspection; enabled by default. |
| `.tooltip(formatter)` | Format the native tooltip. |
| `.hover(closure)` | Compose a custom tooltip from iced widgets. |
| `.on_action(message)` | Publish native chart actions to your update function. |
| `.legend(config)` | Add a legend to the data. |
| `.corners(radius)` | Override the theme's mark rounding. |
