## Theming

The chart design uses the active icedcn theme for text, fonts and grid lines. Its five blue chart tokens are independent of control colors and remain the same in light and dark mode. Native `hyozu::Color::Primary` selects chart-1 and `Secondary` selects chart-2; the interactive example uses these roles for Mobile and Desktop. The monthly tutorial keeps its explicit `color!` fills.

Use `.style(chart::transparent)` when the enclosing card owns the surface. Pass `.design(theme)` to include the theme's font in native layout. `.hover_group(theme.clone(), ...)` supplies that same design and resolves visible series colors before constructing the annotation. Keep `Data` in the model; changing the design does not rebuild the data.

The default mark radius is 4 pixels; use `.corners(...)` to change it. Native Hyozu palettes, marks and styles remain available through `chart::hyozu`.

## Tooltip

`chart::tooltip(group.entries.iter()).label(&group.label)` composes the resolved native group. Rows include the real visible series names, values and colors, including after legend toggles. No second data lookup is needed. Use `hover::Annotation::new(...)` with `.transparent().padding(0)` so the content owns its compact surface while Hyozu positions it.

The examples below show dot, line and dashed indicators, a hidden label, and the nested label used by a single line-indicator row. `.hide_indicator(true)` removes the swatch. `.font("Geist Mono")` sets the numeric font; load that font in your application. Standalone anatomy accepts `(name, value, color)` tuples through the same content builder.
