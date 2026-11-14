//! Card recipe regressions: spacing rebuilds, constrained frames, and lazy headers.
use browser::component::card::CardPage;
use browser::{Message, docs, page};
use iced::{Element, Font, Size};
use iced_test::Simulator;
use iced_test::core::Settings;
use icedxdy::widget::{button, text};
use icedxdy::{Inherited, Slot, Theme, card};

fn settings() -> Settings {
    Settings {
        font: Font::new("Geist"),
        text_size: 14.into(),
        ..Settings::default()
    }
}

fn preview<'a>(page: &'a dyn page::Page, id: &str, width: f32) -> Simulator<'a, Message, Theme> {
    Simulator::with_size(settings(), Size::new(width, 700.), page.preview(id))
}

#[test]
fn spacing_rebuilds_keep_input_values_and_ignore_empty_selection() {
    let page = CardPage::new(0);
    let mut ui = preview(&*page, "spacing", 384.);
    ui.click("Email").expect("email label focuses its input");
    ui.typewrite("qa@example.com");
    ui.click("Password")
        .expect("password label focuses its input");
    ui.typewrite("secret");
    for (choice, inset) in [
        ("20px", 20.),
        ("24px", 24.),
        ("32px", 32.),
        ("32px", 32.),
        ("16px", 16.),
    ] {
        ui.click(choice).expect("spacing control");
        ui.find("qa@example.com")
            .expect("email survives composition");
        ui.find("••••••")
            .expect("password stays masked and retained");
        assert!(ui.find("secret").is_err());
        let header = ui.find("Login to your account").expect("header").bounds();
        let email = ui.find("Email").expect("email label").bounds();
        assert!((header.x - inset).abs() <= 1., "{choice}: {header:?}");
        assert!((email.x - header.x).abs() <= 1., "shared inset");
    }
}

#[test]
fn constrained_cards_keep_content_inside_the_declared_frames() {
    let page = CardPage::new(0);
    for width in [558., 260.] {
        for (id, height, labels) in [
            (
                "demo",
                480.,
                &[
                    "Login to your account",
                    "Sign Up",
                    "Email",
                    "Password",
                    "Forgot your password?",
                    "Login",
                    "Login with Google",
                ][..],
            ),
            ("small", 384., &["Small Card", "Action"][..]),
            (
                "spacing",
                544.,
                &["16px", "20px", "24px", "32px", "Login with Google"][..],
            ),
            (
                "edge-to-edge",
                384.,
                &["Terms of Service", "Decline", "Accept"][..],
            ),
            (
                "image",
                512.,
                &["Design systems meetup", "Featured", "View Event"][..],
            ),
        ] {
            let theme = docs::theme(false);
            let mut ui = Simulator::with_size(
                settings(),
                Size::new(width, height - 80.),
                page.docs_preview(id, &theme),
            );
            ui.draw(&theme);
            for label in labels {
                let bounds = ui.find(*label).expect("complete label").bounds();
                assert!(
                    bounds.x >= 0. && bounds.x + bounds.width <= width + 0.1,
                    "{width} {id} {label}: {bounds:?}"
                );
                assert!(
                    bounds.y >= 0. && bounds.y + bounds.height <= height - 79.9,
                    "{width} {id} {label}: {bounds:?}"
                );
            }
        }
    }
}

#[test]
fn header_receives_disabled_inheritance_before_composing_its_action() {
    let header = card::header(Slot::from(text::small("Title")))
        .description("Description")
        .action(button("Action").on_press(()));
    let mut slot: Slot<'_, ()> = header.into();
    slot.inherit(&Inherited::default().with_disabled(true));
    let mut ui = Simulator::with_size(settings(), Size::new(384., 100.), Element::from(slot));
    ui.click("Action").expect("visible action");
    assert_eq!(
        ui.drain().count(),
        0,
        "a disabled ancestor reaches the action"
    );
}

#[test]
fn footer_keeps_its_child_in_the_disabled_cascade() {
    let mut footer = card::footer(button("Save").on_press(()));
    footer.inherit(&Inherited::default().with_disabled(true));
    let mut ui = Simulator::with_size(settings(), Size::new(384., 100.), Element::from(footer));
    ui.click("Save").expect("visible footer action");
    assert_eq!(ui.drain().count(), 0);
}
