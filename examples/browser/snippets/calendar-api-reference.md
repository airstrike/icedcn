## API Reference

[Read the calendar implementation](https://github.com/airstrike/icedcn/blob/HEAD/widget/src/calendar.rs).

| Rust builder or type | Default | Behavior |
| --- | --- | --- |
| `calendar()` | Single, local current month | Compose a native calendar. |
| `.mode(Mode::Single / Mode::Range)` | `Single` | Select a civil date or inclusive range. |
| `.default_selected(Selection)` | Empty | Seed local selection once. |
| `.selected(Option<Selection>)` | Uncontrolled | Display the application's value, including `None`. |
| `.on_select(handler)` | No callback | Publish the complete new optional selection; a callback is not required for interaction. |
| `.default_month(Date)` | Selected date, then `today()` | Seed the visible month once. |
| `.month(Date)` / `.on_month_change(handler)` | Uncontrolled | Control or observe month navigation. |
| `.number_of_months(usize)` | `1` | Show 1–12 consecutive months. |
| `.stacked(bool)` | Wrap to available width | Use the application's window breakpoint to choose month direction. |
| `.caption_dropdown(bool)` | `false` | Open month and year menus while keeping the date grid visible. |
| `.long_months(bool)` | `false` | Use full month names in both dropdown triggers and menu choices. |
| `.week_numbers(bool)` | `false` | Show Sunday-first locale week numbers; the week containing January 1 is week 01. |
| `.fixed_weeks(bool)` | `false` | Keep six week rows visible. |
| `.cell_size(f32)` | `28` | Set each date cell's width and height in logical pixels. |
| `.padding(f32)` | `8` | Set space around the months. |
| `.weekday_line_height(f32)` | `22.4` | Set the weekday line box; small-text cards use `18.285715`. |
| `.disabled(bool)` | `false` | Disable all interaction; disabled ancestors also apply. |
| `.unavailable(predicate)` | All dates available | Disable matching dates for both pointer and keyboard selection. |
| `.strike_unavailable(bool)` | `false` | Cross out unavailable dates. |
| `.annotate(formatter)` | None | Add a secondary line to in-month cells. |
| `.system(System::Persian)` | Gregorian | Use Solar Hijri month geometry, numerals and right-to-left weeks. |
| `Selection::single(date)` | — | A date, or an externally supplied open range endpoint. |
| `Selection::range(start, end)` | — | An inclusive range with normalized endpoints. |

Tab reaches the navigation controls and one date in the grid. Arrow keys move by day or week; Home and End move to the week boundaries. Page Up and Page Down move by month, or by year with Shift. Shift with Left/Right moves by month; Shift with Up/Down moves by year. Enter and Space select the focused date. Disabled dates are skipped. Escape dismisses caption menus and returns focus to their trigger.
