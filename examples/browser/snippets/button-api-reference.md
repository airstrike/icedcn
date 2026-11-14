### Button

`button(content)` creates a Button from any `impl Into<Slot>`. `button![icon, "Label"]` composes inline children with the size variant's gap. Application messages belong in `on_press`; without a handler, the button stays enabled and can serve as an owner's trigger.

| Builder | Type | Default | Behavior |
| --- | --- | --- | --- |
| `.style(style)` | `button::IntoStyle` | `button::Default` | `Default`, `Outline`, `Secondary`, `Ghost`, `Destructive`, `Link`, or a tweaked `Spec`. Link adds its underline on hover. |
| `.size(size)` | `impl Into<Size>` | `Size::Default` | `Xs` 24px, `Sm` 28px, default 32px, `Lg` 36px. Text and gaps inherit the chosen size. |
| `.square()` | — | `false` | Combine with `Xs`, `Sm`, default or `Lg` for 24/28/32/36px square controls. Set an embedded SVG to 12px for icon-xs and 16px for the others. |
| `.anchor()` | — | `false` | Navigation appearance and pointer cursor; pair with `.on_press(Message::OpenUrl(...))`. The application opens the destination in `update`. |
| `.on_press(message)` | `Message` | `None` | Publish on activation. Handler presence does not determine enabled state. |
| `.disabled(disabled)` | `bool` | `false` | Half-opacity paint, no input and no focus traversal. Disabled ancestors also disable the control. |
| `.rounded()` | — | `false` | Pill or circular outline. |
| `.icon_start()` / `.icon_end()` | — | `false` | Reduce the corresponding padding for an inline icon or spinner. |
| `.invalid(invalid)` | `bool` | `false` | Destructive border and ring, including dark-theme treatment. |
| `.expanded(expanded)` | `bool` | `false` | Expanded appearance for Outline, Ghost and Secondary. |
| `.has_popup(has_popup)` | `bool` | `false` | Suppress the 1px active translation for a popup trigger. |
| `.translate(translate)` | `bool` | `true` | Enable or suppress the 1px press translation. |
| `.weight(weight)` | `impl Into<Weight>` | `Weight::Medium` | Cascades to composed text. |
| `.interaction(cursor)` | `iced::mouse::Interaction` | Default arrow | Override the pointer affordance. |
| `.height(height)` | `impl Into<Pixels>` | Size height | Override height while retaining typography. |
| `.width(width)` | `impl Into<Length>` | Intrinsic | Fill or constrain the control; content is centered for `Fill`. Squares keep equal width and height. |
| `.padding(padding)` | `impl Into<Padding>` | Size padding | Override content padding. Button adds the 1px border outside horizontal padding. |
| `.inset(padding)` | `impl Into<Padding>` | Zero | Separate the painted surface from the hitbox. |
| `.focus_ring(width, color)` | Pixels, theme function | Variant ring | Override the keyboard-focus ring. |
| `.map_style(transform)` | Stock Style → Stock Style | Identity | Transform stock paint fields while retaining rich underline/ring information. Button Group uses this to join corners. |

Stock `iced::widget::button::StyleFn` callbacks and typed closures remain supported. Their paint is retained without guessing a variant from its colors; they do not acquire an underline or focus ring automatically. Variant tags and `Spec` preserve that information before conversion. The legacy stock callback geometry remains unchanged.
