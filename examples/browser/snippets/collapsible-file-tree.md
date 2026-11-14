```rust
use iced::alignment::Vertical;
use iced::widget::{svg, text::LineHeight};
use iced::{Fill, Padding};
use icedxdy::theme::tweak::{Fg, Token};
use icedxdy::widget::{button, collapsible, column, row, tabs, text};
use icedxdy::{Size, Slot, Theme, card};

fn icon(path: &'static str, size: f32) -> svg::Svg<'static, Theme> {
    svg(svg::Handle::from_memory(format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">{path}</svg>"#
    ).into_bytes()))
    .width(size).height(size)
    .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
}


mod chevron {
    use std::f32::consts::FRAC_PI_2;
    use std::sync::LazyLock;
    use std::time::Duration;

    use iced::time::Instant;
    use iced::{Animation, Element, Event, Length, Radians, Rectangle, Renderer, Size};
    use iced::{animation::Easing, mouse, window};
    use iced_widget::Widget as _;
    use iced_widget::core::svg::{Handle, Renderer as _, Svg};
    use iced_widget::core::widget::{Meta, Tree, Widget, tree};
    use iced_widget::core::{Layout, Shell, layout, renderer};
    use icedxdy::widget::animation::cubic_bezier;
    use icedxdy::{Compose, Slot, Theme};

    static PATH: LazyLock<Handle> = LazyLock::new(|| {
        Handle::from_memory(br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m9 18 6-6-6-6"/></svg>"#)
    });

    pub fn chevron<Message: 'static>(open: bool) -> Slot<'static, Message> {
        Slot::new(Chevron { open })
    }

    struct Chevron {
        open: bool,
    }
    struct State {
        turn: Animation<bool>,
        frame: Instant,
    }

    impl Meta for Chevron {}
    impl<Message> Widget<Message, Theme, Renderer> for Chevron {
        fn tag(&self) -> tree::Tag {
            tree::Tag::of::<State>()
        }
        fn state(&self) -> tree::State {
            tree::State::new(State {
                turn: Animation::new(self.open)
                    .duration(Duration::from_millis(150))
                    .easing(Easing::Custom(|t| cubic_bezier(0.4, 0., 0.2, 1., t))),
                frame: Instant::now(),
            })
        }
        fn size(&self) -> Size<Length> {
            Size::new(14.into(), 14.into())
        }
        fn layout(&mut self, tree: &mut Tree, _: &Renderer, limits: &layout::Limits) {
            tree.size = limits.resolve(14, 14, Size::ZERO);
        }
        fn update(
            &mut self,
            tree: &mut Tree,
            event: &Event,
            _: Layout,
            _: mouse::Cursor,
            _: &Renderer,
            shell: &mut Shell<'_, Message>,
            _: &Rectangle,
        ) {
            if let Event::Window(window::Event::RedrawRequested(now)) = event {
                let state = tree.state.downcast_mut::<State>();
                if state.advance(self.open, *now) {
                    shell.request_redraw();
                }
            }
        }
        fn draw(
            &self,
            tree: &Tree,
            renderer: &mut Renderer,
            _: &Theme,
            style: &renderer::Style,
            layout: Layout,
            _: mouse::Cursor,
            viewport: &Rectangle,
        ) {
            let state = tree.state.downcast_ref::<State>();
            let angle = state.turn.interpolate(0., FRAC_PI_2, state.frame);
            renderer.draw_svg(
                Svg::new(PATH.clone())
                    .color(style.text_color)
                    .rotation(Radians(angle)),
                layout.bounds(),
                *viewport,
            );
        }
    }
    impl<Message> Compose<Message> for Chevron {
        fn compose<'a>(self: Box<Self>) -> Element<'a, Message, Theme>
        where
            Self: 'a,
        {
            (*self).boxed()
        }
    }

    impl State {
        fn advance(&mut self, open: bool, now: Instant) -> bool {
            self.frame = now;
            if self.turn.value() != open {
                self.turn.go_mut(open, now);
            }
            self.turn.is_animating(now)
        }
    }
}
use chevron::chevron;

enum Entry {
    File(&'static str),
    Folder(&'static str, &'static [Entry]),
}

const FILE_TREE: &[Entry] = &[
    Entry::Folder(
        "components",
        &[
            Entry::Folder(
                "ui",
                &[
                    Entry::File("button.rs"),
                    Entry::File("card.rs"),
                    Entry::File("dialog.rs"),
                    Entry::File("input.rs"),
                    Entry::File("select.rs"),
                    Entry::File("table.rs"),
                ],
            ),
            Entry::File("login_form.rs"),
            Entry::File("register_form.rs"),
        ],
    ),
    Entry::Folder(
        "lib",
        &[
            Entry::File("util.rs"),
            Entry::File("style.rs"),
            Entry::File("api.rs"),
        ],
    ),
    Entry::Folder(
        "state",
        &[
            Entry::File("window_size.rs"),
            Entry::File("debounce.rs"),
            Entry::File("storage.rs"),
        ],
    ),
    Entry::Folder(
        "types",
        &[Entry::File("types.rs"), Entry::File("request.rs")],
    ),
    Entry::Folder(
        "public",
        &[
            Entry::File("favicon.ico"),
            Entry::File("logo.svg"),
            Entry::File("images"),
        ],
    ),
    Entry::File("main.rs"),
    Entry::File("layout.rs"),
    Entry::File("theme.rs"),
    Entry::File("Cargo.toml"),
    Entry::File("rustfmt.toml"),
    Entry::File("README.md"),
    Entry::File(".gitignore"),
];

fn entry<Message: Clone + 'static>(node: &'static Entry) -> Slot<'static, Message> {
    match node {
        Entry::File(name) => button(row![
            icon(r#"<path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"/><path d="M14 2v4a2 2 0 0 0 2 2h4"/>"#, 14.),
            *name,
        ]
        .spacing(8)
        .align_y(Vertical::Center)
        .width(Fill))
        .size(Size::Sm)
        .style(button::Link + Fg(Token::Foreground))
        .width(Fill)
        .into(),
        Entry::Folder(name, children) => collapsible(move |folder| {
            column![
                folder.trigger(button(row![
                    chevron(folder.is_open()),
                    icon(r#"<path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/>"#, 14.),
                    *name,
                ]
                .spacing(4)
                .align_y(Vertical::Center)
                .width(Fill))
                .size(Size::Sm)
                .style(button::Ghost)
                .width(Fill)),
                folder.content(
                    column(children.iter().map(entry))
                        .width(Fill)
                        .spacing(4)
                        .padding(Padding::ZERO.top(4).left(20))
                ),
            ]
            .width(Fill)
            .into()
        })
        .width(Fill)
        .into(),
    }
}

fn file_tree<Message: Clone + 'static>() -> Slot<'static, Message> {
    let header = tabs(|tabs| {
        tabs.list([
            tabs.trigger(
                "explorer",
                text::small("Explorer")
                    .medium()
                    .line_height(LineHeight::Absolute(20.into())),
            ),
            tabs.trigger(
                "settings",
                text::small("Outline")
                    .medium()
                    .line_height(LineHeight::Absolute(20.into())),
            ),
        ])
    })
    .default_value("explorer")
    .width(Fill)
    .list_width(Fill);
    card(
        column![
            card::header(Slot::from(header)).inset(12.),
            card::content(column(FILE_TREE.iter().map(entry)).width(Fill).spacing(4))
                .padding([0, 12]),
        ]
        .width(Fill)
        .padding([12, 0])
        .spacing(8),
    )
    .gap(0)
    .padding(0)
    .width(Fill.max(256))
    .into()
}


fn view() -> Slot<'static, ()> {
    file_tree()
}
```
