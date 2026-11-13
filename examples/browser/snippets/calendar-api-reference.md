## API Reference

| Builder | Purpose |
| --- | --- |
| `.mode(Mode::Range)` | Choose a start and end date over consecutive clicks. The default is single-day selection. |
| `.selected(value)`, `.default_selected(value)`, `.on_select(handler)` | Control, seed and observe optional selections. |
| `.month(date)`, `.default_month(date)`, `.on_month_change(handler)` | Control, seed and observe the visible month. |
| `.system(System::Persian)` | Display Solar Hijri months and Persian digits. Returned dates use the Gregorian calendar. |
| `.caption_dropdown(true)` | Select a month or year directly. |
| `.number_of_months(n)` | Display adjacent months. |
| `.week_numbers(true)` | Show a week-number column. |
| `.unavailable(predicate)` | Disable specified dates. |
| `.cell_size(pixels)`, `.annotate(formatter)` | Resize cells and add per-day details. |
