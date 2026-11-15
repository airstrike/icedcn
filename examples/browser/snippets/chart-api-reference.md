## Legend

`.legend(legend::Config::below().size(12).interactive(true))` displays names and swatches below the plot. Click an entry to hide or show its series. Hidden-series state lives in Hyozu's widget tree; grouped hover includes only the visible series.

## API Reference

| API | Purpose |
| --- | --- |
| `data(marks)` | Prepare native Hyozu data in the model. |
| `bar(values).with_name(name)` | Build a named bar series; `(x, y)` values can use timestamps. |
| `bars(series)` | Group multiple bar series. |
| `chart(&data)` | Borrow data into a native chart; fills its available width. |
| `.height(height)` | Reserve a visible chart height. |
| `.style(chart::transparent)` | Let the enclosing surface own background and border. |
| `.design(theme)` | Use the supplied design for layout, fonts and paint. |
| `.x_axis(...)`, `.y_axis(...)` | Configure axis kind, labels, bounds, lines and grid on the data. |
| `.show_tooltip(bool)` | Enable or disable inspection; enabled by default. |
| `.tooltip(hyozu::Tooltip::default().tracking_line(false).category_band(color))` | Configure native inspection feedback on the data. |
| `.hover(closure)` | Compose a custom annotation for one native mark. |
| `.hover_group(theme.clone(), closure)` | Compose the full visible Cartesian group with resolved colors. |
| `chart::tooltip(entries)` | Compose native hover entries or standalone `(name, value, color)` rows. |
| `.label(label)`, `.hide_label(bool)` | Supply or hide the tooltip heading. |
| `.indicator(tooltip::Line)`, `.hide_indicator(bool)` | Choose Dot, Line or Dashed swatches, or hide them. |
| `.font(font)` | Select the tooltip's numeric font. |
| `.on_action(message)` | Publish native chart actions to your update function. |
| `.legend(config)` | Add a legend to the data. |
| `.corners(radius)` | Override the design's mark rounding. |
