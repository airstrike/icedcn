### BreadcrumbLink

`breadcrumb::link(content, message)` publishes the supplied message on pointer release inside the pressed link, or on Enter/Space while focused. Moving out before release cancels pointer activation. Links enter keyboard focus order, display a focus ring and animate from muted to foreground on hover. A disabled ancestor suppresses input. Handle navigation in the application update function.
