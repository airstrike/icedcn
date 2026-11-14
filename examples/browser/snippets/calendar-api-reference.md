## API Reference

| Builder | Purpose |
| --- | --- |
| `.mode(Mode::Range)` | Choose a start and end date over consecutive clicks. The default is single-day selection. |
| `.selected(value)`, `.default_selected(value)`, `.on_select(f)` | Set the controlled or initial selection. On selection changes, publish a message carrying `Option<Selection>`. |
| `.month(date)`, `.default_month(date)`, `.on_month_change(f)` | Set the controlled or initial month. On navigation, publish a message carrying the new month’s `Date`. |
| `.system(System::Persian)` | Display Solar Hijri months and Persian digits. Returned dates use the Gregorian calendar. |
| `.caption_dropdown(true)` | Select a month or year directly. |
| `.number_of_months(n)` | Display adjacent months. |
| `.week_numbers(true)` | Show a week-number column. |
| `.unavailable(predicate)` | Disable specified dates. |
| `.cell_size(pixels)`, `.annotate(formatter)` | Resize cells and add per-day details. |
