## API Reference

Build the sidebar from a container, a provider and the sections below.

### Sidebar root constructors

| Constructor | Returns | Description |
| --- | --- | --- |
| `sidebar::fixed(content)` | `Container<'a>` | Sidebar flush against the window edge — solid background, no margin. |
| `sidebar::floating(content)` | `Container<'a>` | Sidebar floating with margin, rounded corners, border, and shadow. |
| `sidebar::inset(content)` | `Container<'a>` | Inset sidebar — no border on the sidebar itself; the content area (paired via `sidebar::main_styled`) carries the rounded/border/shadow chrome. |

The three constructors return an icedxdy container with width, padding and style builders.

### Section helpers

| Constructor | Returns | Description |
| --- | --- | --- |
| `sidebar::header(content)` | `Column` | Top section for the brand badge + a `sidebar::trigger` for collapse. |
| `sidebar::content(content)` | `Column` | Fill-height column for groups and menus. Wrap long navigation in a scrollable. |
| `sidebar::footer(content)` | `Column` | Bottom section — typically a `sidebar::menu(...)` with an account / settings button. |
| `sidebar::group(content)` | `Column` | A logical group inside `content`. |
| `sidebar::group_label(label)` | `Text<'a>` | Muted 12-point medium-weight group heading. |
| `sidebar::group_content(content)` | `Column` | Inner container for a group's items. |
| `sidebar::separator()` | `Container` | 1px horizontal divider scoped to the sidebar palette. |
| `sidebar::menu(items)` | `Column` | Vertical list of menu items at consistent spacing. |
| `sidebar::menu_item(content)` | `Row` | Wraps a single menu row (typically a `menu_button`). |
| `sidebar::input(...)` | `Container` | Inline search box scoped to the sidebar palette. |

### `sidebar::menu_button()`

The sidebar menu button uses its sidebar accent for hover and the active theme accent for a selected destination.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.size(size)` | `MenuButtonSize` | `Default` (32px) | Size variant: `Sm` (28px), `Default` (32px), `Lg` (48px). Drives both height and text size. |
| `.active(bool)` | `bool` | `false` | Selected-state — switches palette to main `palette.accent` and bumps weight to medium. |
| `.collapsed(bool)` | `bool` | `false` | Icon-only mode for `Collapsible::Icon` sidebars. Pairs with `.tooltip(...)` for the right-side tooltip. |
| `.tooltip(text)` | `impl ToString` | `None` | Show the tooltip on hover while `.collapsed(true)`. |
| `.on_press(message)` | `Message` | `None` | Publish `message` when the menu button is activated. |

### Sub-menu helpers

| Constructor | Returns | Description |
| --- | --- | --- |
| `sidebar::submenu(items)` | `Column` | Indented sub-list under a `menu_item`. |
| `sidebar::submenu_item(content)` | `Slot` | One row inside a submenu. |
| `sidebar::submenu_button(content)` | `SidebarMenuButton` | Smaller variant of `menu_button` for submenu rows. |
| `sidebar::menu_action(content)` | `SidebarMenuAction` | Trailing action chip (typically a small icon button) on a menu row. |
| `sidebar::menu_badge(content)` | `Slot` | Trailing badge (count, status indicator) on a menu row. |
| `sidebar::menu_skeleton()` | `Container` | Loading placeholder for a menu row. |

### Layout helpers

| Constructor | Returns | Description |
| --- | --- | --- |
| `sidebar::trigger()` | `Slot` | Collapse / expand toggle button (icon-only). Toggles the enclosing `sidebar::provider` from anywhere inside it. |
| `sidebar::rail(on_press)` | `SidebarRail` | Edge-grab handle for resize / toggle. |
| `sidebar::main(content)` | `Container` | The main content area paired with a fixed/floating sidebar. |
| `sidebar::main_styled(content)` | `Container` | The main content area paired with `sidebar::inset` — carries the rounded/border/shadow chrome that the inset sidebar drops. |

### `sidebar::provider(body)`

An iced `Component` around the whole layout that owns whether the sidebar is open, like `SidebarProvider`. `body` receives a `sidebar::Handle`; `s.sidebar(content)` sizes the sidebar for the current state. Any `sidebar::trigger()` inside, and `Cmd+B` (`Ctrl+B`), toggle it.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.default_open(bool)` | `bool` | `true` | Whether an uncontrolled sidebar starts open. |
| `.open(bool)` | `bool` | uncontrolled | Control the sidebar. |
| `.on_open_change(f)` | `Fn(bool) -> Message` | `None` | Publish a message with the requested open state. |
| `.collapsible(mode)` | `sidebar::Collapsible` | `Offcanvas` | `Offcanvas` hides it, `Icon` shrinks it to `WIDTH_ICON`, `None` never collapses. |

`sidebar::Side` (`Left` / `Right`) and the `WIDTH`, `WIDTH_ICON`, `WIDTH_MOBILE` constants describe the geometry. Restyle via `core::theme::sidebar::*` style functions or compose your own surface helper.
