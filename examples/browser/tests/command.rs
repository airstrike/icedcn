//! Interaction, layout and focus rendering in the installed documentation search.
use browser::app::{App, Message};
use iced::time::Instant;
use iced::{Color, Event, Point, Rectangle, Renderer, Size, Vector, mouse, window};
use iced_test::core::Settings;
use iced_test::core::renderer::{Headless, Style};
use iced_test::core::shell::{Bus, Waker};
use iced_test::futures::futures::executor::block_on;
use iced_test::runtime::UserInterface;
use iced_test::simulator::{self, Simulator};
use iced_widget::core::widget::{Id, Operation, operation};
use icedxdy::Theme;
use icedxdy::widget::markdown::highlighter::Map;

#[derive(Default)]
struct Bounds {
    // The open modal is visited after the browser's main view.
    viewports: Vec<Rectangle>,
    buttons: Vec<Rectangle>,
    focused: Option<Rectangle>,
    inputs: Vec<Rectangle>,
    labels: Vec<(String, Rectangle)>,
}

impl Operation for Bounds {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }

    fn scrollable(
        &mut self,
        _: Option<&Id>,
        bounds: Rectangle,
        _: Size,
        _: Vector,
        _: &mut dyn operation::Scrollable,
    ) {
        self.viewports.push(bounds);
    }

    fn focusable(
        &mut self,
        _: Option<&Id>,
        bounds: Rectangle,
        widget: &mut dyn operation::Focusable,
    ) {
        self.buttons.push(bounds);
        if widget.is_focused() {
            self.focused = Some(bounds);
        }
    }

    fn text_input(&mut self, _: Option<&Id>, bounds: Rectangle, _: &mut dyn operation::TextInput) {
        self.inputs.push(bounds);
    }

    fn text(&mut self, _: Option<&Id>, bounds: Rectangle, text: &mut dyn operation::Text) {
        self.labels.push((text.text().to_string(), bounds));
    }
}

fn renderer() -> Renderer {
    let backend = std::env::var("ICED_TEST_BACKEND").ok();
    block_on(Renderer::new(Default::default(), backend.as_deref())).expect("headless renderer")
}

fn redraw() -> Event {
    Event::Window(window::Event::RedrawRequested(Instant::now()))
}

fn dispatch(
    ui: &mut UserInterface<'_, Message, Theme, Renderer>,
    renderer: &mut Renderer,
    cursor: mouse::Cursor,
    events: &[Event],
) -> Vec<Message> {
    let mut bus = Bus::new();
    ui.update(
        &window::Headless,
        &Waker::noop(),
        events,
        cursor,
        renderer,
        &mut bus,
    );
    bus.drain().map(|(message, _)| message).collect()
}

impl Bounds {
    fn read(ui: &mut UserInterface<'_, Message, Theme, Renderer>, renderer: &Renderer) -> Self {
        let mut bounds = Self::default();
        ui.operate(renderer, &mut bounds);
        bounds
    }
}

#[test]
fn focused_search_has_one_field_border_and_one_outer_dialog_ring() {
    let mut renderer = renderer();
    let mut app = App::new(Map::from(&Theme::light()), Map::from(&Theme::dark())).0;
    let size = Size::new(1440., 900.);
    let _ = app.update(Message::Width(size.width));
    let _ = app.update(Message::Search(true));
    for theme in [Theme::light(), Theme::dark()] {
        // At a flat edge, Chromium gives the number of physical pixels and
        // the final sRGB for each stripe. Leave corner antialiasing free.
        let [ring, panel, border, fill] = if theme.neutral.is_dark {
            [38u8, 23, 72, 40]
        } else {
            [234, 255, 229, 242]
        };
        let mut ui = UserInterface::build(app.view(), size, Default::default(), &mut renderer);
        // An update mounts the overlay for drawing; operate alone can visit
        // its controls before the renderer has seen the first modal frame.
        dispatch(
            &mut ui,
            &mut renderer,
            mouse::Cursor::Unavailable,
            &[redraw()],
        );
        let bounds = Bounds::read(&mut ui, &renderer);
        let input = bounds.inputs.last().expect("open search input");
        assert_eq!(
            bounds.focused,
            Some(*input),
            "the modal mounts with keyboard focus"
        );
        let list = bounds.viewports.last().expect("command results");
        for scale in [1, 2] {
            ui.draw(
                &mut renderer,
                &theme,
                &Style::default(),
                mouse::Cursor::Unavailable,
            );
            let width = size.width as u32 * scale;
            let rgba = renderer.screenshot(
                Size::new(width, size.height as u32 * scale),
                scale as f32,
                Color::from(theme.neutral.base),
            );
            let x = (list.x as u32 - 12) * scale;
            let y = (input.center_y() as u32) * scale;
            let mut offset = 0;
            for (points, gray) in [(4, ring), (8, panel), (1, border), (3, fill)] {
                for dx in offset..offset + points * scale {
                    let start = ((y * width + x + dx) * 4) as usize;
                    let pixel = &rgba[start..start + 3];
                    assert!(
                        pixel.iter().all(|channel| channel.abs_diff(gray) <= 3),
                        "{scale}× at ({}, {y}): expected {gray}, found {pixel:?}",
                        x + dx
                    );
                }
                offset += points * scale;
            }
        }
    }
}

#[test]
fn search_result_viewport_and_hitbox_fill_the_dialog() {
    let mut app = App::new(Map::from(&Theme::light()), Map::from(&Theme::dark())).0;
    for width in [375., 1440.] {
        let _ = app.update(Message::Width(width));
        let _ = app.update(Message::Search(true));
        let mut ui: Simulator<'_, Message, Theme, iced::Renderer> = Simulator::with_size(
            Settings {
                text_size: 14.into(),
                ..Settings::default()
            },
            Size::new(width, 720.),
            app.view(),
        );
        ui.simulate([Event::Window(
            window::Event::RedrawRequested(Instant::now()),
        )]);
        for (query, title) in [
            ("", "Aspect Ratio"),
            (
                "message opening saved",
                "Message Scroller › Opening Saved Threads",
            ),
        ] {
            if !query.is_empty() {
                ui.typewrite(query);
            }
            let mut layout = Bounds::default();
            ui.operate(&mut layout);
            let viewport = layout
                .viewports
                .last()
                .expect("results scroll within the dialog");
            let (_, label) = layout
                .labels
                .iter()
                .rev()
                .find(|(text, bounds)| title == text && viewport.contains(bounds.center()))
                .expect("result text visible in this viewport");
            let row = layout
                .buttons
                .iter()
                .rev()
                .find(|r| r.contains(label.center()))
                .expect("result has a clickable row");
            let expected = (width - 32.).min(512.) - 16.;
            assert!(
                (viewport.width - expected).abs() < 1.,
                "{width}: viewport {viewport:?}, expected {expected}"
            );
            assert!(
                (row.width - viewport.width).abs() < 1.,
                "{width}: row {row:?} inside {viewport:?}"
            );
            assert!(
                label.height < 24.,
                "{width}: one-line result wrapped: {label:?}"
            );
            if !query.is_empty() {
                ui.point_at(Point::new(row.x + row.width - 2., label.center_y()));
                ui.simulate(simulator::click());
                assert!(
                    ui.drain()
                        .any(|m| matches!(m, Message::Navigate(_, Some(_))))
                );
            }
        }
    }
}

#[test]
fn dismissing_search_returns_focus_with_the_opening_and_dismissal_modality() {
    use iced::keyboard::key::Named;
    use iced_widget::core::widget::operation::{Outcome, focusable};

    let mut renderer = renderer();
    let size = Size::new(1440., 900.);
    let theme = Theme::dark();
    for (pointer_open, pointer_close) in
        [(true, true), (true, false), (false, true), (false, false)]
    {
        let mut app = App::new(Map::from(&Theme::light()), Map::from(&theme)).0;
        let _ = app.update(Message::Width(size.width));
        let mut ui = UserInterface::build(app.view(), size, Default::default(), &mut renderer);
        dispatch(
            &mut ui,
            &mut renderer,
            mouse::Cursor::Unavailable,
            &[redraw()],
        );
        let bounds = Bounds::read(&mut ui, &renderer);
        let target = bounds
            .labels
            .iter()
            .find(|(label, _)| label == "Search documentation...")
            .expect("header search label")
            .1
            .center();
        let trigger = *bounds
            .buttons
            .iter()
            .find(|b| b.contains(target))
            .expect("focusable search button");
        let (cursor, events): (_, Vec<_>) = if pointer_open {
            (
                mouse::Cursor::Available(target),
                simulator::click().collect(),
            )
        } else {
            // Walk the host's real keyboard focus order up to the search button.
            for _ in 0..bounds.buttons.len() {
                let mut step = focusable::focus_next::<()>();
                ui.operate(&renderer, &mut step);
                if let Outcome::Chain(mut select) = step.finish() {
                    ui.operate(&renderer, select.as_mut());
                }
                if Bounds::read(&mut ui, &renderer).focused == Some(trigger) {
                    break;
                }
            }
            (
                mouse::Cursor::Unavailable,
                simulator::tap_key(Named::Enter, None).collect(),
            )
        };
        let open = dispatch(&mut ui, &mut renderer, cursor, &events)
            .into_iter()
            .find(|event| matches!(event, Message::Search(true)))
            .expect("trigger opens search");
        let cache = ui.into_cache();
        let _ = app.update(open);
        let mut ui = UserInterface::build(app.view(), size, cache, &mut renderer);
        dispatch(&mut ui, &mut renderer, cursor, &[redraw()]);
        let cursor = mouse::Cursor::Available(Point::new(50., 150.));
        let dismiss: Vec<_> = if pointer_close {
            simulator::click().collect()
        } else {
            simulator::tap_key(Named::Escape, None).collect()
        };
        let close = dispatch(&mut ui, &mut renderer, cursor, &dismiss)
            .into_iter()
            .find(|event| matches!(event, Message::Search(false)))
            .expect("action dismisses search");
        let cache = ui.into_cache();
        let _ = app.update(close);
        let mut ui = UserInterface::build(app.view(), size, cache, &mut renderer);
        dispatch(&mut ui, &mut renderer, cursor, &[redraw(), redraw()]);
        assert_eq!(Bounds::read(&mut ui, &renderer).focused, Some(trigger));
        ui.draw(&mut renderer, &theme, &Style::default(), cursor);
        let pixels = renderer.screenshot(Size::new(1440, 900), 1., Color::from(theme.neutral.base));
        let offset = ((trigger.y as usize - 2) * 1440 + trigger.center_x() as usize) * 4;
        let expected = if pointer_open && pointer_close {
            10
        } else {
            63
        };
        assert!(
            pixels[offset].abs_diff(expected) <= 1,
            "pointer open={pointer_open}, pointer close={pointer_close}: expected {expected}, got {}",
            pixels[offset]
        );
    }
}
