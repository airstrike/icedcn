### BreadcrumbEllipsis

`breadcrumb::ellipsis(None)` is a static collapsed-trail indicator. It centers a 16px Lucide ellipsis in a 20px square with the SVG feature; without SVG it uses a text glyph. `breadcrumb::ellipsis(Some(message))` makes the indicator keyboard and pointer actionable. The application decides which crumbs to collapse.

The static part can be a button or trigger child. It inherits the enclosing control’s resolved foreground at draw time, including hover, focus and disabled styling; it does not fix its own color. The trail supplies muted color when the part appears directly in the list.
