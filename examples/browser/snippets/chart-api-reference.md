## Chart Config

Use `Series::new(label, color, values)` for each plotted series. The same configuration supplies the bar fill, legend swatch and tooltip entries. Values are indexed by category.

## Theming

Grid lines, tick labels, selection and tooltip surfaces follow the active theme. Series accept an explicit color so the same data remains recognizable across light and dark mode.

## Legend

`.legend(true)` displays the color and name of each visible series beneath the plot.

## Accessibility

Focus an interactive chart with Tab. Move between categories with Left and Right, jump to the first or last category with Home and End, and dismiss the tooltip with Escape. Touch a category to inspect its values.
