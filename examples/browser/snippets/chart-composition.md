## Component

Enable the `chart` feature on your icedcn dependency to include Hyozu and its canvas renderer.

Build charts with Hyozu's native marks and data. `icedcn::widget::chart` re-exports `bar`, `bars`, `data` and `chart`, and exposes the complete library as `chart::hyozu`. Charts compose directly inside icedcn rows, columns and cards.

Construct `Data` when your model initializes or updates. The view borrows it with `chart(&data)`, so chart preparation stays outside the view. Give each chart a height; its width fills the available space.
