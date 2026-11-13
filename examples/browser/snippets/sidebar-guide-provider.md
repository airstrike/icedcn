## SidebarProvider

Wrap the sidebar and its trigger in `sidebar::provider`. An uncontrolled provider keeps its open state across view rebuilds. A controlled provider receives `.open(value)` and sends changes to your application through `.on_open_change(...)`.

### Width

The default panel is 256 points wide. The mobile panel uses 288 points and the collapsed icon rail uses 48 points. The public constants are `WIDTH`, `WIDTH_MOBILE` and `WIDTH_ICON`.

### Keyboard Shortcut

Press Command+B on macOS or Control+B on Windows and Linux to toggle a collapsible sidebar.
