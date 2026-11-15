use std::collections::HashSet;

use super::route::Route;
use browser_routing::Route as _;
use iced_test::core::Settings;
use iced_test::simulator::Simulator;

use super::not_found;
use crate::{Browser, Message, examples};

#[test]
fn every_registered_page_has_a_unique_round_trip_path() {
    let (browser, _) = Browser::new();
    let app = browser.app;
    let mut paths = HashSet::new();
    for (index, definition) in app.routes.routes().take(app.pages.len()).enumerate() {
        let path = definition.path();
        assert!(paths.insert(path), "duplicate route: {path}");
        assert_eq!(Route::parse(path, &app.routes), Route::Page(index));
    }
}

#[test]
fn history_restores_components_guides_gallery_and_receipts() {
    let (browser, _) = Browser::new();
    let mut app = browser.app;
    let _ = app.update(Message::Example(examples::Event::OpenReceipts));
    assert_eq!(app.route(), Route::Receipts);
    let _ = app.update(Message::Example(examples::Event::Gallery));
    assert_eq!(app.route(), Route::Examples);
    let _ = app.update(Message::History("/examples/receipts".into()));
    assert_eq!(app.route(), Route::Receipts);
    let _ = app.update(Message::History("/examples/".into()));
    assert_eq!(app.route(), Route::Examples);
    let _ = app.update(Message::History("/docs/installation".into()));
    assert_eq!(app.title(), "Installation — icedcn");
    let _ = app.update(Message::History("/components/button".into()));
    assert_eq!(app.title(), "Button — icedcn");
    let selected = app.route();
    let _ = app.update(Message::SelectPage(app.current_page));
    assert_eq!(app.route(), selected);
}

#[test]
fn not_found_survives_history_and_recovers_through_each_destination() {
    let (browser, _) = Browser::new();
    let mut app = browser.app;
    for path in [
        "/bad-route",
        "/components/missing/",
        "/docs/missing",
        "/examples/receipts/extra",
    ] {
        for (destination, title) in [
            ("/", "Accordion — icedcn"),
            ("/docs/components", "Components — icedcn"),
            ("/examples", "Examples — icedcn"),
        ] {
            let _ = app.update(Message::History(path.into()));
            assert_eq!(app.route(), Route::NotFound(path.into()));
            assert_eq!(app.title(), "Page not found — icedcn");
            let _ = app.update(Message::ToggleTheme);
            let _ = app.update(Message::Search(true));
            let _ = app.update(Message::Search(false));
            assert_eq!(app.route(), Route::NotFound(path.into()));
            assert_eq!(app.route().path(&app.routes), Some(path));
            let _ = app.update(Message::GoTo(destination));
            assert_eq!(app.title(), title);
            assert!(app.not_found.is_none());
            let _ = app.update(Message::History(path.into()));
            assert_eq!(app.route(), Route::NotFound(path.into()));
        }
    }
}

#[test]
fn not_found_displays_the_path_as_text_and_offers_working_recovery_buttons() {
    let path = "/<script>alert(1)</script>";
    for (label, destination) in [
        ("Home", "/"),
        ("Components", "/docs/components"),
        ("Examples", "/examples"),
    ] {
        let mut ui = Simulator::with_size(
            Settings::default(),
            iced::Size::new(375., 700.),
            not_found::view(path),
        );
        ui.find("Page not found").unwrap();
        ui.find(path).unwrap();
        ui.click(label).unwrap();
        assert!(
            ui.into_messages()
                .any(|message| matches!(message, Message::GoTo(path) if path == destination))
        );
    }
}

#[test]
fn long_bad_paths_keep_recovery_buttons_in_the_viewport() {
    let path = format!("/{}", "missing/".repeat(1000));
    let mut ui = Simulator::with_size(
        Settings::default(),
        iced::Size::new(375., 700.),
        not_found::view(&path),
    );
    for label in ["Home", "Components", "Examples"] {
        let bounds = ui.find(label).unwrap().bounds();
        assert!(bounds.y >= 0. && bounds.y + bounds.height <= 700.);
        assert!(bounds.x >= 0. && bounds.x + bounds.width <= 375.);
    }
}
