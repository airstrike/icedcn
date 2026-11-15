## Theming

Charts use the active icedcn theme for text colors, grid lines, tooltip surfaces and default series colors. Text uses the application renderer's default font. Pass `.design(&theme)` when you want the theme's font and text sizes to participate in chart layout. Explicit series colors remain stable across light and dark mode. The default mark radius is 4 pixels; use `.corners(...)` on a chart to change it.

All native Hyozu customization remains available, including `.palette(...)` on `Data`, `.design(...)` on a chart, and `.hover(...)` for a composed tooltip. The tooltip example below renders a category label and both series values in an icedcn card surface.
