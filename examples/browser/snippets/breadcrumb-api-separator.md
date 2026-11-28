### BreadcrumbSeparator

`.separator(factory)` replaces the automatically inserted separators. The factory is `Fn() -> View` and is called once per gap. The SVG-enabled default is a 14px Lucide chevron; without SVG it is a text chevron in the same box. Custom separators inherit the list styling.
