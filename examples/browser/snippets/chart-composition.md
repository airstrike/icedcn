## Component

Enable the `chart` feature on your icedcn dependency to include Hyozu and its canvas renderer.

Build charts with Hyozu's native marks and data. `icedcn::widget::chart` re-exports `bar`, `bars`, `data` and `chart`, and exposes the complete library as `chart::hyozu`. Charts compose directly inside icedcn rows, columns and cards.

Construct `Data` when your model initializes or updates. The view borrows it with `chart(&data)`, so chart preparation stays outside the view. Give each chart a height; its width fills the available space.

Chart paint follows the active `Theme`: axis labels, grid, background and series defaults use its design tokens. Customize `design.chart` to set chart-1 through chart-5 independently of button colors. Leave marks uncolored, or use semantic colors such as `hyozu::Color::Secondary`, to keep cached data responsive to theme changes. Explicit mark colors remain overrides.

For custom chart fonts, pass `.design(&theme)` or `.design(Cow::<Theme>::Owned(theme.clone()))` so Hyozu can resolve typography during layout. Rebuild the chart view when the design changes; keep the prepared `Data`.

The content builder `chart::tooltip(...)` supplies the compact label, series rows and indicator styles. Hyozu still owns inspection, grouping, hidden-series state and overlay placement. Use `.hover_group(...)` to connect the two.
