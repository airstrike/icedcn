## Features

- **Plain streaming output stays small.** A string without line numbers uses one selectable paragraph. These highlighted recipes parse their fixed samples once and render a safe prefix as output arrives.
- **Line decorations stay separate.** Line numbers and diff markers are not selectable code. Copy actions choose the exact source string, including hidden lines; the Diff action deliberately copies the original patch.
- **Code stays left to right.** The native examples retain code order independently of surrounding prose. The project's browser does not include the upstream RTL example.
- **Highlighting is optional.** These recipes use iced's existing parser with GitHub Light/Dark colors. Plain output has no highlighting dependency. ANSI colors reset at each line.
- **Code fits other content.** Compose it inside a Message, a Markdown fence, Trace, Tabs, or prose.
- **Content supports keyboard scrolling.** Tab to the content, then use arrows, Page Up/Down, Home/End or Space. Floating actions appear when content or an action has keyboard focus.
