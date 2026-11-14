//! Measure the result viewport and hit target in the installed documentation search.
use browser::app::{App, Message};
use iced::time::Instant;
use iced::{Event, Point, Rectangle, Size, Vector, window};
use iced_test::core::Settings;
use iced_test::simulator::{self, Simulator};
use iced_widget::core::widget::{Id, Operation, operation};
use icedxdy::Theme;
use icedxdy::widget::markdown::highlighter::Map;

#[derive(Default)]
struct Bounds {
    // The open modal is visited after the browser's main view.
    viewports: Vec<Rectangle>,
    buttons: Vec<Rectangle>,
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

    fn focusable(&mut self, _: Option<&Id>, bounds: Rectangle, _: &mut dyn operation::Focusable) {
        self.buttons.push(bounds);
    }

    fn text(&mut self, _: Option<&Id>, bounds: Rectangle, text: &mut dyn operation::Text) {
        self.labels.push((text.text().to_string(), bounds));
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
            let expected = (width - 32.).min(512.) - 40.;
            assert!(
                (viewport.width - expected).abs() < 1.,
                "{width}: viewport {viewport:?}, expected {expected}"
            );
            assert!(
                (row.width - viewport.width + 8.).abs() < 1.,
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
