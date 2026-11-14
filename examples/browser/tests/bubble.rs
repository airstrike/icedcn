//! Regressions for Bubble's surface actions and out-of-flow reaction geometry.
use browser::Message;
use browser::component::bubble::BubblePage;
use browser::page::Page;
use iced::alignment::{Horizontal, Vertical};
use iced::keyboard::key::Named;
use iced::widget::{Id, Space};
use iced::{Element, Event, Point, Rectangle, Size, mouse};
use iced_test::core::Settings;
use iced_test::{Selector, Simulator, selector, simulator};
use iced_widget::core::widget::Operation;
use iced_widget::core::widget::operation::{self, Outcome, focusable};
use icedxdy::widget::{bubble, container, toaster};
use icedxdy::{Inherited, Slot, Theme};

type Preview<'a> = Simulator<'a, Message, Theme, iced::Renderer>;

fn preview<'a>(page: &'a dyn Page, id: &str, width: f32) -> Preview<'a> {
    Simulator::with_size(
        Settings::default(),
        Size::new(width, 1400.),
        Element::from(toaster(page.docs_preview(id, &Theme::light()))),
    )
}
fn next(ui: &mut Preview<'_>) -> Rectangle {
    let mut op = focusable::focus_next::<()>();
    ui.operate(&mut op);
    if let Outcome::Chain(mut op) = op.finish() {
        ui.operate(op.as_mut());
    }
    ui.find(selector::is_focused()).unwrap().bounds()
}
fn close(a: f32, b: f32) {
    assert!((a - b).abs() < 0.1, "{a} != {b}");
}
fn click_at(ui: &mut Preview<'_>, at: Point) {
    ui.point_at(at);
    let _ = ui.simulate(simulator::click());
}

#[test]
fn surface_padding_is_clickable_keyboard_reachable_and_inherits_disabled() {
    for disabled in [false, true] {
        let mut content: Slot<'_, Message> = bubble(Space::new().width(20).height(12))
            .on_press(Message::ToggleTheme)
            .into();
        content.inherit(&Inherited::default().with_disabled(disabled));
        let mut ui: Preview<'_> = Simulator::with_size(
            Settings::default(),
            Size::new(300., 100.),
            content.compose(),
        );
        if !disabled {
            let bounds = next(&mut ui);
            assert_eq!(bounds.size(), Size::new(46., 30.));
        }
        click_at(&mut ui, Point::new(2., 2.));
        assert_eq!(ui.drain().count(), usize::from(!disabled));
        for key in [Named::Enter, Named::Space] {
            ui.tap_key(key);
            assert_eq!(ui.drain().count(), usize::from(!disabled));
        }
    }
}

#[test]
fn all_three_suggestions_raise_their_toast_from_the_whole_surface_and_keyboard() {
    let page = BubblePage::new(0);
    for (index, label) in [
        "You clicked forgot password",
        "You clicked help with subscription",
        "You clicked something else. Talk to a human.",
    ]
    .into_iter()
    .enumerate()
    {
        for key in [None, Some(Named::Enter), Some(Named::Space)] {
            let mut ui = preview(&*page, "link-button", 384.);
            let mut bounds = Rectangle::default();
            for _ in 0..=index {
                bounds = next(&mut ui);
            }
            if let Some(key) = key {
                ui.tap_key(key);
            } else {
                click_at(&mut ui, Point::new(bounds.x + 2., bounds.y + 2.));
            }
            let mut ui = ui.rebuild(Element::from(toaster(
                page.docs_preview("link-button", &Theme::light()),
            )));
            ui.find(label).unwrap();
        }
    }
}

#[test]
fn reactions_do_not_change_row_height_and_remain_anchored_at_both_widths() {
    for width in [384., 260.] {
        for side in [Vertical::Top, Vertical::Bottom] {
            for align in [Horizontal::Left, Horizontal::Right] {
                let id = Id::unique();
                let surface = Id::unique();
                let content = bubble(Space::new().width(80).height(40)).reactions(
                    container(Space::new().width(20).height(20)).id(id.clone()),
                    align,
                    side,
                );
                let mut ui: Preview<'_> = Simulator::with_size(
                    Settings::default(),
                    Size::new(width, 200.),
                    Element::from(
                        container(content)
                            .id(surface.clone())
                            .width(iced::Fill)
                            .padding([40, 0]),
                    ),
                );
                let row = ui.find(surface).unwrap().bounds();
                close(row.height, 40. + 18. + 80.);
                let reaction = ui.find(id).unwrap().bounds();
                // Float's operation reports its in-flow bounds. Paint/input is
                // translated by 75% of the 24px chip plus its outer 3px ring.
                let painted_y = reaction.y + if side == Vertical::Top { -21. } else { 21. };
                close(painted_y, if side == Vertical::Top { 24. } else { 94. });
                close(
                    reaction.x,
                    if align == Horizontal::Left { 18. } else { 68. },
                );
            }
        }
    }
}

#[test]
fn reaction_action_works_at_its_translated_position_and_by_keyboard() {
    let page = BubblePage::new(0);
    for width in [384., 260.] {
        for key in [None, Some(Named::Enter), Some(Named::Space)] {
            let mut ui = preview(&*page, "reactions", width);
            let button = next(&mut ui);
            if let Some(key) = key {
                ui.tap_key(key);
            } else {
                click_at(
                    &mut ui,
                    Point::new(
                        button.center_x(),
                        button.center_y() + button.height * 0.75 + 3.,
                    ),
                );
            }
            ui.draw(&Theme::light());
            let mut ui = ui.rebuild(Element::from(toaster(
                page.docs_preview("reactions", &Theme::light()),
            )));
            assert!(
                ui.find("You clicked yes, running command...").is_ok(),
                "reaction failed at {width}, {key:?}, {button:?}"
            );
        }
    }
}

#[test]
fn review_repeats_with_pointer_keyboard_and_keeps_state_on_resize_and_retheme() {
    let page = BubblePage::new(0);
    let mut ui = preview(&*page, "collapsible", 384.);
    for _ in 0..2 {
        ui.click("Show more").unwrap();
        ui.find("Show less").unwrap();
        ui.find(|candidate: selector::Candidate<'_>| match candidate {
            selector::Candidate::Text {
                content, bounds, ..
            } if content
                .ends_with("so the other themes can choose their own focus treatment later.") =>
            {
                Some(bounds)
            }
            _ => None,
        })
        .unwrap();
        ui.click("Show less").unwrap();
    }
    ui.find(selector::is_focused()).unwrap();
    ui.tap_key(Named::Enter);
    ui.find("Show less").unwrap();
    let mut ui = ui.resize(Size::new(260., 1400.));
    ui.find("Show less").unwrap();
    let mut ui = ui.rebuild(Element::from(toaster(
        page.docs_preview("collapsible", &Theme::dark()),
    )));
    ui.find("Show less").unwrap();
    ui.tap_key(Named::Space);
    ui.find("Show more").unwrap();
}

#[test]
fn translated_tooltip_and_popover_triggers_remain_reachable() {
    let page = BubblePage::new(0);
    let mut ui = preview(&*page, "tooltip", 260.);
    let button = next(&mut ui);
    ui.draw(&Theme::light());
    ui.draw(&Theme::light());
    ui.find("Read on Jan 5, 2026 at 4:32 PM").unwrap();
    ui.tap_key(Named::Escape);
    ui.draw(&Theme::light());
    assert!(ui.find("Read on Jan 5, 2026 at 4:32 PM").is_err());
    let at = Point::new(
        button.center_x(),
        button.center_y() + button.height * 0.75 + 3.,
    );
    ui.point_at(at);
    let _ = ui.simulate([Event::Mouse(mouse::Event::CursorMoved { position: at })]);
    ui.draw(&Theme::light());
    ui.draw(&Theme::light());
    ui.find("Read on Jan 5, 2026 at 4:32 PM").unwrap();

    let mut ui = preview(&*page, "popover", 260.);
    let button = next(&mut ui);
    for key in [None, Some(Named::Enter), Some(Named::Space)] {
        if let Some(key) = key {
            ui.tap_key(key);
        } else {
            click_at(
                &mut ui,
                Point::new(
                    button.center_x(),
                    button.center_y() + button.height * 0.75 + 3.,
                ),
            );
        }
        ui.find("Command failed with exit code 1").unwrap();
        ui.find("ENOENT: no such file or directory, open pnpm-lock.yaml")
            .unwrap();
        ui.tap_key(Named::Escape);
        ui = ui.rebuild(Element::from(toaster(
            page.docs_preview("popover", &Theme::light()),
        )));
        assert!(ui.find("Command failed with exit code 1").is_err());
        ui.find(selector::is_focused()).unwrap();
    }
    ui.tap_key(Named::Enter);
    ui.find("Command failed with exit code 1").unwrap();
    click_at(&mut ui, Point::new(5., 5.));
    ui = ui.rebuild(Element::from(toaster(
        page.docs_preview("popover", &Theme::light()),
    )));
    assert!(ui.find("Command failed with exit code 1").is_err());
}

#[test]
fn every_recipe_fits_its_frame_at_full_and_narrow_widths() {
    let page = BubblePage::new(0);
    for width in [384., 260.] {
        for id in [
            "demo",
            "variants",
            "alignment",
            "group-demo",
            "link-button",
            "reactions",
            "collapsible",
            "tooltip",
            "popover",
        ] {
            let mut ui = preview(&*page, id, width);
            let select = |candidate: selector::Candidate<'_>| match candidate {
                selector::Candidate::Text { bounds, .. } => Some(bounds),
                _ => None,
            };
            let mut op = select.find_all();
            ui.operate(&mut operation::black_box(&mut op));
            let Outcome::Some(text) = op.finish() else {
                panic!("{id}: missing content")
            };
            assert!(!text.is_empty(), "{id}: empty recipe");
            for bounds in text {
                assert!(
                    bounds.x >= -0.1 && bounds.x + bounds.width <= width + 0.1,
                    "{id} at {width}: text escapes frame {bounds:?}"
                );
                assert!(
                    bounds.y >= 0. && bounds.y + bounds.height <= 1400.,
                    "{id} at {width}: vertical clipping {bounds:?}"
                );
            }
            assert!(ui.find("Show less").is_err());
            assert!(ui.find("Command failed with exit code 1").is_err());
            assert!(ui.find("Read on Jan 5, 2026 at 4:32 PM").is_err());
        }
    }
}
