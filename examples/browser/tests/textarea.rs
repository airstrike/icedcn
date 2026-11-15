use browser::component::textarea::TextareaPage;
use iced::keyboard::key::Named;
use iced::mouse::{self, Button};
use iced::touch::{self, Finger};
use iced::{Element, Event, Point, Rectangle, Size};
use iced_test::Simulator;
use iced_test::core::widget::operation::{Outcome, TextInput, focusable};
use iced_test::core::widget::{Id, Operation};
use icedcn::Theme;
use icedcn::widget::{column, field, textarea};

#[derive(Default)]
struct Editors(Vec<(Rectangle, String)>);

impl Operation for Editors {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }
    fn text_input(&mut self, _: Option<&Id>, bounds: Rectangle, input: &mut dyn TextInput) {
        self.0.push((bounds, input.text().into_owned()));
    }
}

fn editor<'a, M: 'a>(ui: &mut Simulator<'a, M, Theme>) -> (Rectangle, String) {
    let mut editors = Editors::default();
    ui.operate(&mut editors);
    assert_eq!(editors.0.len(), 1);
    editors.0.pop().unwrap()
}

#[test]
fn docs_constrain_each_recipe_at_both_viewports() {
    let mut page = TextareaPage::new(0);
    for (viewport, frame, expected) in [(390., 340., 260.), (1440., 638., 320.)] {
        page.window_width(viewport);
        for id in ["demo", "field", "invalid", "button"] {
            let mut ui = Simulator::with_size(
                Default::default(),
                Size::new(frame, 288.),
                page.docs_preview(id, &Theme::light()),
            );
            let (bounds, _) = editor(&mut ui);
            assert_eq!(bounds.width, expected, "{id} at {viewport}");
            assert_eq!(bounds.height, 64.);
            if id == "button" {
                ui.find("Send message").unwrap();
            }
        }
    }
}

#[test]
fn a_disabled_field_skips_tab_and_a_label_activates_multiline_editing() {
    let view: Element<'_, String, Theme> = column![
        field(textarea("Unavailable"))
            .disabled(true)
            .label("Disabled"),
        field(textarea("Write").on_input(|s| s)).label("Message"),
    ]
    .into();
    let mut ui = Simulator::new(view);
    ui.click("Disabled").unwrap();
    ui.typewrite("ignored");
    assert!(ui.drain().next().is_none());
    let mut next = focusable::focus_next::<()>();
    ui.operate(&mut next);
    if let Outcome::Chain(mut next) = next.finish() {
        ui.operate(next.as_mut());
    }
    ui.typewrite("A");
    ui.click("Message").unwrap();
    for _ in 0..4 {
        ui.tap_key(Named::Enter);
        ui.typewrite("B");
    }
    assert_eq!(ui.drain().last().as_deref(), Some("A\nB\nB\nB\nB"));
    let (bounds, value) = editor(&mut ui);
    assert_eq!(value, "A\nB\nB\nB\nB");
    assert!(bounds.height >= 100., "five lines occupy {bounds:?}");
}

#[test]
fn controlled_replacements_remain_authoritative_and_keep_a_valid_cursor() {
    let view = |v: &str| -> Element<'static, String, Theme> {
        field(textarea("").value(v).on_input(|s| s))
            .label("Message")
            .into()
    };
    let mut ui = Simulator::new(view("初\nA"));
    ui.click("Message").unwrap();
    ui.tap_key(Named::ArrowDown);
    ui.tap_key(Named::End);
    ui.typewrite("B");
    assert_eq!(ui.drain().last().as_deref(), Some("初\nAB"));
    let mut ui = ui.rebuild(view(""));
    ui.typewrite("X");
    assert_eq!(ui.drain().last().as_deref(), Some("X"));
    let frozen: Element<'_, String, Theme> =
        field(textarea("").value("kept")).label("Message").into();
    let mut ui = ui.rebuild(frozen);
    ui.typewrite("ignored");
    assert_eq!(editor(&mut ui).1, "kept");
}

#[test]
fn grip_resizes_with_mouse_and_touch_and_clamps_at_the_minimum() {
    let view: Element<'_, (), Theme> = textarea("").into();
    let mut ui = Simulator::new(view);
    let (bounds, _) = editor(&mut ui);
    let grip = Point::new(bounds.x + bounds.width - 5., bounds.y + bounds.height - 5.);
    let end = Point::new(grip.x, grip.y + 50.);
    ui.point_at(grip);
    ui.simulate([Event::Mouse(mouse::Event::ButtonPressed(Button::Left))]);
    ui.point_at(end);
    ui.simulate([
        Event::Mouse(mouse::Event::CursorMoved { position: end }),
        Event::Mouse(mouse::Event::ButtonReleased(Button::Left)),
    ]);
    assert_eq!(editor(&mut ui).0.height, 114.);
    let id = Finger(7);
    ui.simulate([
        Event::Touch(touch::Event::FingerPressed { id, position: end }),
        Event::Touch(touch::Event::FingerMoved {
            id: Finger(8),
            position: Point::ORIGIN,
        }),
    ]);
    assert_eq!(editor(&mut ui).0.height, 114.);
    ui.simulate([
        Event::Touch(touch::Event::FingerMoved {
            id,
            position: Point::ORIGIN,
        }),
        Event::Touch(touch::Event::FingerLost {
            id,
            position: Point::ORIGIN,
        }),
        Event::Touch(touch::Event::FingerMoved { id, position: end }),
    ]);
    assert_eq!(editor(&mut ui).0.height, 64.);
}

#[test]
fn resizing_a_scrolled_editor_uses_pointer_deltas_in_one_coordinate_space() {
    use iced_test::core::widget::operation::Animation;
    use iced_test::core::widget::operation::scrollable::{AbsoluteOffset, scroll_to};
    use icedcn::widget::{scrollable, space};

    for touch in [false, true] {
        let id = Id::unique();
        let view: Element<'_, (), Theme> = scrollable::scrollable(column![
            space().height(800),
            textarea("").default_value("one\ntwo\nthree\nfour\nfive"),
            space().height(800),
        ])
        .id(id.clone())
        .height(360)
        .into();
        let mut ui = Simulator::with_size(Default::default(), Size::new(360., 360.), view);
        ui.operate(&mut scroll_to(
            id,
            AbsoluteOffset {
                x: None,
                y: Some(720.),
            },
            Animation::Instant,
        ));
        ui.draw(&Theme::light());
        let (before, _) = editor(&mut ui);
        assert!(before.height > 100.);
        let grip = Point::new(
            before.x + before.width - 6.,
            before.y + before.height - 6. - 720.,
        );
        let end = Point::new(grip.x, grip.y + 50.);
        let id = Finger(11);
        ui.point_at(grip);
        ui.simulate([if touch {
            Event::Touch(touch::Event::FingerPressed { id, position: grip })
        } else {
            Event::Mouse(mouse::Event::ButtonPressed(Button::Left))
        }]);
        ui.point_at(end);
        if touch {
            ui.simulate([
                Event::Touch(touch::Event::FingerMoved { id, position: end }),
                Event::Touch(touch::Event::FingerLifted { id, position: end }),
            ]);
        } else {
            ui.simulate([
                Event::Mouse(mouse::Event::CursorMoved { position: end }),
                Event::Mouse(mouse::Event::ButtonReleased(Button::Left)),
            ]);
        }
        assert_eq!(
            editor(&mut ui).0.height,
            before.height + 50.,
            "touch={touch}"
        );
    }
}
