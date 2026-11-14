//! Regression coverage for the installed Badge recipes after state painting changes.
use browser::component::badge::BadgePage;
use browser::{Message, page};
use iced::keyboard::key::Named;
use iced::{Point, Size};
use iced_test::Simulator;
use iced_test::core::Settings;
use iced_test::core::widget::operation::{Operation, Outcome, focusable};
use icedxdy::Theme;

#[test]
fn all_badge_labels_fit_both_documentation_frame_widths() {
    let examples: &[(&str, &[&str])] = &[
        ("demo", &["Badge", "Secondary", "Destructive", "Outline"]),
        (
            "variants",
            &["Default", "Secondary", "Destructive", "Outline", "Ghost"],
        ),
        ("icon", &["Verified", "Bookmark"]),
        ("spinner", &["Deleting", "Generating"]),
        ("link", &["Open Link"]),
    ];
    let mut page = BadgePage::new(2);
    // These are the current native panel widths at 1440px and 390px windows.
    for (window, width) in [(1440., 634.), (390., 324.)] {
        page.window_width(window);
        for &(id, labels) in examples {
            let theme = Theme::light();
            let mut ui = Simulator::with_size(
                Settings {
                    text_size: 14.into(),
                    ..Settings::default()
                },
                Size::new(width, 300.),
                page.docs_preview(id, &theme),
            );
            ui.draw(&theme);
            for &label in labels {
                let bounds = ui.find(label).expect("complete recipe label").bounds();
                assert!(
                    bounds.x >= 0. && bounds.x + bounds.width <= width,
                    "{window}: {id}/{label}"
                );
                assert!(bounds.y >= 0. && bounds.y + bounds.height <= 300.);
                assert!(bounds.height <= 16.01, "one line inside the 20px badge");
            }
        }
    }
}

#[test]
fn installed_link_repeats_pointer_and_keyboard_fragment_messages_after_rebuild() {
    let page = BadgePage::new(2);
    let mut ui = Simulator::with_size(
        Settings::default(),
        Size::new(324., 100.),
        page.preview("link"),
    );
    let expect_link = |message: Message| {
        assert!(
            matches!(message, Message::Page(2, page::Message::LinkClicked(ref uri)) if uri.as_str() == "#link")
        );
    };
    for _ in 0..2 {
        ui.click("Open Link")
            .expect("click the installed link label");
        let messages = ui.drain().collect::<Vec<_>>();
        assert_eq!(messages.len(), 1);
        messages.into_iter().for_each(expect_link);
        ui = ui.rebuild(page.preview("link"));
    }
    ui.point_at(Point::new(300., 90.));
    for _ in 0..2 {
        ui.tap_key(Named::Enter);
        let messages = ui.drain().collect::<Vec<_>>();
        assert_eq!(messages.len(), 1);
        messages.into_iter().for_each(expect_link);
        ui = ui.rebuild(page.preview("link"));
    }
    // A freshly mounted Link is also reached by the native Tab operation.
    let mut ui = Simulator::with_size(
        Settings::default(),
        Size::new(324., 100.),
        page.preview("link"),
    );
    let mut next = focusable::focus_next::<()>();
    ui.operate(&mut next);
    if let Outcome::Chain(mut op) = next.finish() {
        ui.operate(op.as_mut());
    }
    ui.tap_key(Named::Enter);
    let messages = ui.drain().collect::<Vec<_>>();
    assert_eq!(messages.len(), 1);
    messages.into_iter().for_each(expect_link);
}

#[test]
fn existing_badge_consumers_keep_their_labels_and_child_interactions() {
    use browser::component::{
        card::CardPage, input::InputPage, spinner::SpinnerPage, trace::TracePage,
    };
    let consumers: Vec<(Box<dyn page::Page>, &str, &[&str])> = vec![
        (CardPage::new(0), "image", &["Featured"]),
        (InputPage::new(1), "badge", &["Beta"]),
        (
            SpinnerPage::new(2),
            "badge",
            &["Syncing", "Updating", "Processing"],
        ),
    ];
    for (page, id, labels) in consumers {
        let mut ui =
            Simulator::with_size(Settings::default(), Size::new(600., 500.), page.preview(id));
        ui.draw(&Theme::light());
        for &label in labels {
            let bounds = ui.find(label).expect("consumer badge label").bounds();
            assert!(bounds.width > 0. && bounds.height <= 16.01);
        }
    }
    let page = TracePage::new(3);
    let mut ui = Simulator::with_size(
        Settings::default(),
        Size::new(600., 500.),
        page.preview("steps"),
    );
    ui.click("Searched for the checkout build log")
        .expect("open Badge consumer disclosure");
    ui = ui.rebuild(page.preview("steps"));
    ui.draw(&Theme::light());
    ui.find("vercel.com")
        .expect("first Badge mounts inside disclosure");
    ui.find("github.com")
        .expect("second Badge mounts inside disclosure");
}
