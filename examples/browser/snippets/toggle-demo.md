```rust
struct Bookmark;
impl<'a> Component<'a, Message, Theme> for Bookmark {
    type State = bool;
    type Event = bool;
    fn update(&self, state: &mut bool, next: bool, _: &iced::Renderer) -> Option<Message> {
        *state = next;
        None
    }
    fn view(&self, pressed: &bool) -> impl Widget<bool, Theme> + 'a {
        static FILLED: OnceLock<svg::Handle> = OnceLock::new();
        let glyph: icedcn::View<'a, bool> = if *pressed {
            let handle =
                FILLED.get_or_init(|| svg::Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M19 21 12 17 5 21V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2z" fill="black" stroke="black" stroke-linejoin="round" stroke-width="2"/></svg>"#));
            svg(handle.clone())
                .width(14)
                .height(14)
                .style(|t: &Theme, _| svg::Style {
                    color: Some(t.neutral.foreground.into()),
                })
                .boxed()
                .into()
        } else {
            icon::bookmark().size(14).into()
        };
        Element::from(
            toggle(row![glyph, "Bookmark"].spacing(6))
                .size(Size::Sm)
                .style(toggle::Outline)
                .pressed(*pressed)
                .on_pressed_change(|v| v),
        )
    }
}

component(Bookmark).boxed()
```
