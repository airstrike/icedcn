## Features

- **It streams cheaply.** A plain string renders as one text node, and a highlighted string is parsed again only when it changes.
- **It copies cleanly.** Line numbers and diff markers are left out of a selection, and blank lines survive the copy.
- **It reads left to right.** Code stays left to right inside surrounding prose.
- **It highlights natively.** `.language(..)` colors the code in the widget, with no async step, and `"ansi"` renders the colors in terminal output.
- **It fits where code shows up.** Use it in a message, a Markdown fence, a trace, tabs, a dialog or between paragraphs.
- **It scrolls by keyboard.** The content is focusable, and floating actions show on focus as well as on hover.
