## API Reference

The sidebar suite is a family of helpers, not a single component. Compose the canonical shadcn shape by stacking constructors at the call site.

### Sidebar root constructors

| Constructor | Returns | Description |
| --- | --- | --- |
| `sidebar::fixed(content)` | `Container<'a>` | Sidebar flush against the window edge — solid background, no margin. Mirrors shadcn's default `sidebar` variant. |
| `sidebar::floating(content)` | `Container<'a>` | Sidebar floating with margin, rounded corners, border, and shadow. Mirrors shadcn's `floating` variant. |
| `sidebar::inset(content)` | `Container<'a>` | Inset sidebar — no border on the sidebar itself; the content area (paired via `sidebar::main_styled`) carries the rounded/border/shadow chrome. Mirrors shadcn's `inset` variant. |

The three constructors return an icedxdy container with width, padding and style builders.

### Section helpers

| Constructor | Returns | Description |
| --- | --- | --- |
| `sidebar::header(content)` | `Column` | Top section for the brand badge + a `sidebar::trigger` for collapse. Mirrors shadcn's `SidebarHeader`. |
| `sidebar::content(content)` | `Column` | Fill-height column for groups and menus. Wrap long navigation in a scrollable. Mirrors shadcn's `SidebarContent`. |
| `sidebar::footer(content)` | `Column` | Bottom section — typically a `sidebar::menu(...)` with an account / settings button. Mirrors shadcn's `SidebarFooter`. |
| `sidebar::group(content)` | `Column` | A logical group inside `content`. Mirrors shadcn's `SidebarGroup`. |
| `sidebar::group_label(label)` | `Text<'a>` | Muted 12-point medium-weight group heading. Mirrors shadcn's `SidebarGroupLabel`. |
| `sidebar::group_content(content)` | `Column` | Inner container for a group's items. Mirrors shadcn's `SidebarGroupContent`. |
| `sidebar::separator()` | `Container` | 1px horizontal divider scoped to the sidebar palette. Mirrors shadcn's `SidebarSeparator`. |
| `sidebar::menu(items)` | `Column` | Vertical list of menu items at consistent spacing. Mirrors shadcn's `SidebarMenu`. |
| `sidebar::menu_item(content)` | `Row` | Wraps a single menu row (typically a `menu_button`). Mirrors shadcn's `SidebarMenuItem`. |
| `sidebar::input(...)` | `Slot` | Inline search box scoped to the sidebar palette. Mirrors shadcn's `SidebarInput`. |

### `sidebar::menu_button()`

The sidebar menu button uses its sidebar accent for hover and the active theme accent for a selected destination.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.size(size)` | `MenuButtonSize` | `Default` (32px) | Size variant: `Sm` (28px), `Default` (32px), `Lg` (48px). Drives both height and text size. |
| `.active(bool)` | `bool` | `false` | Selected-state — switches palette to main `palette.accent` and bumps weight to medium. |
| `.collapsed(bool)` | `bool` | `false` | Icon-only mode for `Collapsible::Icon` sidebars. Pairs with `.tooltip(...)` for the right-side tooltip. |
| `.tooltip(text)` | `impl ToString` | `None` | Tooltip rendered when collapsed (only fires while `.collapsed(true)`). |
| `.on_press(message)` | `Message` | `None` | Click handler. |

### Sub-menu helpers

| Constructor | Returns | Description |
| --- | --- | --- |
| `sidebar::submenu(items)` | `Column` | Indented sub-list under a `menu_item`. Mirrors shadcn's `SidebarMenuSub`. |
| `sidebar::submenu_item(content)` | `Slot` | One row inside a submenu. Mirrors shadcn's `SidebarMenuSubItem`. |
| `sidebar::submenu_button(content)` | `SidebarMenuButton` | Smaller variant of `menu_button` for submenu rows. Mirrors shadcn's `SidebarMenuSubButton`. |
| `sidebar::menu_action(content)` | `SidebarMenuAction` | Trailing action chip (typically a small icon button) on a menu row. Mirrors shadcn's `SidebarMenuAction`. |
| `sidebar::menu_badge(content)` | `Slot` | Trailing badge (count, status indicator) on a menu row. Mirrors shadcn's `SidebarMenuBadge`. |
| `sidebar::menu_skeleton()` | `Container` | Loading placeholder for a menu row. Mirrors shadcn's `SidebarMenuSkeleton`. |

### Layout helpers

| Constructor | Returns | Description |
| --- | --- | --- |
| `sidebar::trigger()` | `Slot` | Collapse / expand toggle button (icon-only). Toggles the enclosing `sidebar::provider` from anywhere inside it. Mirrors shadcn's `SidebarTrigger`. |
| `sidebar::rail(on_press)` | `SidebarRail` | Edge-grab handle for resize / toggle. Mirrors shadcn's `SidebarRail`. |
| `sidebar::main(content)` | `Container` | The main content area paired with a fixed/floating sidebar. Mirrors shadcn's `SidebarInset` for the flush variants. |
| `sidebar::main_styled(content)` | `Container` | The main content area paired with `sidebar::inset` — carries the rounded/border/shadow chrome that the inset sidebar drops. |

### `sidebar::provider(body)`

An iced `Component` around the whole layout that owns whether the sidebar is open, like `SidebarProvider`. `body` receives a `sidebar::Handle`; `s.sidebar(content)` sizes the sidebar for the current state. Any `sidebar::trigger()` inside, and `Cmd+B` (`Ctrl+B`), toggle it.

| Builder | Type | Default | Description |
| --- | --- | --- | --- |
| `.default_open(bool)` | `bool` | `true` | Whether an uncontrolled sidebar starts open. |
| `.open(bool)` | `bool` | uncontrolled | Control the sidebar (shadcn `open`). |
| `.on_open_change(f)` | `Fn(bool) -> Message` | `None` | Receives the new open state (shadcn `onOpenChange`). |
| `.collapsible(mode)` | `sidebar::Collapsible` | `Offcanvas` | `Offcanvas` hides it, `Icon` shrinks it to `WIDTH_ICON`, `None` never collapses. |

`sidebar::Side` (`Left` / `Right`) and the `WIDTH`, `WIDTH_ICON`, `WIDTH_MOBILE` constants describe the geometry. Restyle via `core::theme::sidebar::*` style functions or compose your own surface helper.
