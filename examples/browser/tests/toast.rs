use browser::component::toast::Page;
use browser::page;
use iced::{Element, Event, Fill, Point, Rectangle, Size, mouse, touch};
use iced_test::Simulator;
use iced_test::core::widget::{Id, Operation};
use icedcn::Theme;
use icedcn::widget::{button, container, row, toast, toaster};

const TITLE: &str = "Event has been created";

fn preview<'a>(page: &'a dyn page::Page, id: &str) -> Element<'a, browser::Message, Theme> {
    toaster(container(page.preview(id)).center_x(Fill).center_y(Fill))
        .position(toast::Position::TopCenter)
        .into()
}

#[derive(Default)]
struct Containers(Vec<Rectangle>);
impl Operation for Containers {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }
    fn container(&mut self, _: Option<&Id>, bounds: Rectangle, _: &Rectangle) {
        self.0.push(bounds);
    }
}

fn card<M: 'static>(ui: &mut Simulator<'_, M, Theme>, title: &str) -> Rectangle {
    let point = ui.find(title).unwrap().bounds().center();
    let mut boxes = Containers::default();
    ui.operate(&mut boxes);
    boxes
        .0
        .into_iter()
        .filter(|r| r.contains(point) && r.width > 300. && r.height < 160.)
        .max_by(|a, b| a.width.total_cmp(&b.width))
        .unwrap()
}

#[test]
fn all_six_anchors_use_their_window_insets_and_the_full_mobile_card() {
    let page = Page::new(0);
    for (w, h) in [(1440., 900.), (390., 844.)] {
        let width = if w < 600. { w - 32. } else { 356. };
        let edge = if w < 600. { 16. } else { 24. };
        for (label, x, top) in [
            ("Top Left", edge, true),
            ("Top Center", (w - width) / 2., true),
            ("Top Right", w - edge - width, true),
            ("Bottom Left", edge, false),
            ("Bottom Center", (w - width) / 2., false),
            ("Bottom Right", w - edge - width, false),
        ] {
            let mut ui = Simulator::with_size(
                Default::default(),
                Size::new(w, h),
                preview(&*page, "position"),
            );
            ui.click(label).unwrap();
            let mut ui = ui.rebuild(preview(&*page, "position"));
            ui.draw(&Theme::light());
            let r = card(&mut ui, TITLE);
            assert_eq!(
                (r.x, r.y, r.width),
                (x, if top { edge } else { h - edge - r.height }, width),
                "{label} at {w}"
            );
            assert!((r.height - 53.5).abs() < 1.0, "{r:?}");
        }
    }
}

fn notes() -> Element<'static, (), Theme> {
    toaster(
        container(row![
            toast::trigger(button("First"), toast::toast("Older")),
            toast::trigger(button("Second"), toast::toast("Newer")),
        ])
        .center_x(Fill)
        .center_y(Fill),
    )
    .position(toast::Position::TopCenter)
    .into()
}

fn point<M: 'static>(ui: &mut Simulator<'_, M, Theme>, p: Point) {
    ui.point_at(p);
    ui.simulate([Event::Mouse(mouse::Event::CursorMoved { position: p })]);
    ui.draw(&Theme::light());
}

#[test]
fn hovering_the_compact_stack_reveals_older_cards_and_keeps_the_gap_active() {
    let mut ui = Simulator::new(notes());
    for label in ["First", "Second"] {
        ui.click(label).unwrap();
        ui = ui.rebuild(notes());
    }
    ui.draw(&Theme::light());
    assert!(ui.find("Older").is_err());
    let top = card(&mut ui, "Newer");
    point(&mut ui, top.center());
    let older = card(&mut ui, "Older");
    assert_eq!(older.y, top.y + top.height + 14.);
    point(&mut ui, Point::new(top.center_x(), older.y - 5.));
    ui.find("Older").unwrap();
    point(&mut ui, Point::ORIGIN);
    assert!(ui.find("Older").is_err());
}

#[test]
fn finger_taps_and_lost_contacts_keep_the_card_but_an_outward_swipe_dismisses_it() {
    let mut ui = Simulator::new(notes());
    ui.click("First").unwrap();
    let mut ui = ui.rebuild(notes());
    ui.draw(&Theme::light());
    let start = card(&mut ui, "Older").center();
    let end = Point::new(start.x, start.y - 50.);
    let id = touch::Finger(1);
    for release in [
        touch::Event::FingerLifted {
            id,
            position: start,
        },
        touch::Event::FingerLost { id, position: end },
    ] {
        ui.simulate([
            Event::Touch(touch::Event::FingerPressed {
                id,
                position: start,
            }),
            Event::Touch(release),
        ]);
        ui = ui.rebuild(notes());
        ui.find("Older").unwrap();
    }
    ui.simulate([
        Event::Touch(touch::Event::FingerPressed {
            id,
            position: start,
        }),
        Event::Touch(touch::Event::FingerMoved { id, position: end }),
        Event::Touch(touch::Event::FingerLifted { id, position: end }),
    ]);
    let mut ui = ui.rebuild(notes());
    assert!(ui.find("Older").is_err());
}

#[test]
fn the_full_article_keeps_dom_order_until_the_notification_shortcut_is_pressed() {
    use browser::app::App;
    use iced::event::Status;
    use iced::keyboard::key::Named;
    use iced_test::core::widget::operation::{Focusable, Outcome, focusable};
    use iced_test::simulator;
    use icedcn::widget::markdown::highlighter::Map;

    let theme = Theme::light();
    let mut app = App::new(Map::from(&theme), Map::from(&theme)).0;
    let _ = app.update(browser::Message::OpenComponent("toast".into()));
    let mut ui = Simulator::with_size(Default::default(), Size::new(1440., 900.), app.view());
    ui.draw(&theme);
    ui.click("Show Toast").unwrap();
    let mut ui = ui.rebuild(app.view());
    ui.draw(&theme);
    ui.find("Undo").unwrap();

    // The browser's shortcut subscription runs the global focus operation
    // only when the widget tree leaves Tab unhandled.
    let statuses = ui.simulate([simulator::press_key(Named::Tab, None)]);
    if statuses == [Status::Ignored] {
        let mut focus = focusable::focus_next::<()>();
        ui.operate(&mut focus);
        if let Outcome::Chain(mut select) = focus.finish() {
            ui.operate(select.as_mut());
        }
    }
    ui.tap_key(Named::Enter);
    let mut ui = ui.rebuild(app.view());
    assert!(ui.find("Undo").is_ok(), "ordinary Tab stays in the article");
    assert!(
        !ui.drain()
            .any(|m| matches!(m, browser::Message::Page(_, page::Message::Noop)))
    );

    // The full page has many controls around the trigger. Record which one
    // owns focus immediately before entering the notification list.
    #[derive(Default)]
    struct Focus(Option<Rectangle>);
    impl Operation for Focus {
        fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
            visit(self);
        }
        fn focusable(&mut self, _: Option<&Id>, rect: Rectangle, control: &mut dyn Focusable) {
            if control.is_focused() {
                self.0 = Some(rect);
            }
        }
    }
    let mut before = Focus::default();
    ui.operate(&mut before);
    assert!(before.0.is_some());

    use iced::keyboard::{Event::KeyPressed, Key, Modifiers};
    let mut shortcut = simulator::press_key(Key::Character("t".into()), None);
    if let Event::Keyboard(KeyPressed { modifiers, .. }) = &mut shortcut {
        *modifiers = Modifiers::ALT;
    }
    assert_eq!(ui.simulate([shortcut]), [Status::Captured]);
    for _ in 0..2 {
        assert_eq!(
            ui.simulate([simulator::press_key(Named::Tab, None)]),
            [Status::Captured]
        );
    }
    let action = ui.find("Undo").unwrap().bounds();
    let mut focused = Focus::default();
    ui.operate(&mut focused);
    assert!(
        focused.0.is_some_and(|r| r.contains(action.center())),
        "Undo owns focus"
    );
    ui.tap_key(Named::Escape);
    ui.find("Undo").unwrap();
    ui.tap_key(Named::Enter);
    let mut ui = ui.rebuild(app.view());
    ui.draw(&theme);
    let mut restored = Focus::default();
    ui.operate(&mut restored);
    assert_eq!(
        restored.0, before.0,
        "activation returns focus to the previous article control"
    );
    assert!(
        ui.find("Undo").is_err(),
        "the action must dismiss the notification"
    );
    assert!(
        ui.drain()
            .any(|m| matches!(m, browser::Message::Page(_, page::Message::Noop)))
    );
}
