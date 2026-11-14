## Usage

```rust
use iced::Widget;
use iced::widget::svg;
use icedxdy::Theme;
use icedxdy::widget::{alert, button};

alert()
    .icon(svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 16v-4M12 8h.01"/></svg>"#.as_bytes()))
        .width(16)
        .height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.card.foreground.into()) })
        .boxed())
    .title("Heads up!")
    .description("You can add components and dependencies to your app using the cli.")
    .action(button("Enable").style(button::Outline))
    .max_width(448)
```

Drop the `.icon(...)` call to render a title-and-description-only alert. Drop `.action(...)` for the simple callout. Use `.style(alert::Destructive)` for the destructive variant.

The action stays enabled without a handler, as in the source example. Add `.on_press(Message::Enable)` to the button to publish your application’s message; handle it in `update`. Alert itself has no open or checked state.
