//! Exercise the independent controls rendered by the installed Attachment recipes.
use std::borrow::Cow;

use browser::component::attachment;
use browser::icon;
use iced::keyboard::key::Named;
use iced::{Point, Rectangle, Size, Vector};
use iced_test::core::Settings;
use iced_test::simulator::{self, Simulator};
use iced_widget::core::widget::{Id, Operation, operation};

fn glyph(name: &str) -> &'static str {
    icon::ALL_ICONS
        .iter()
        .find(|(key, _)| *key == name)
        .unwrap()
        .1
}

fn settings() -> Settings {
    let mut settings = Settings::default();
    settings.fonts.push(Cow::Borrowed(icon::FONT));
    settings
}

#[test]
fn the_states_preview_has_five_distinct_rows_and_retry_keeps_its_file() {
    let page = attachment::AttachmentPage::new(0);
    let mut ui = Simulator::with_size(settings(), Size::new(460., 680.), page.preview("states"));
    for file in [
        "selected-file.pdf",
        "design-system.zip",
        "market-research.pdf",
        "financial-model.xlsx",
        "uploaded-report.pdf",
    ] {
        ui.find(file).unwrap();
    }
    ui.find("Upload failed. Try again.").unwrap();
    ui.click(glyph("refresh_cw")).unwrap();
    ui.find("Uploaded").unwrap();
    ui.find("financial-model.xlsx").unwrap();
    assert!(ui.find("Upload failed. Try again.").is_err());
}

#[test]
fn remove_does_not_open_the_preview_dialog_and_the_trigger_accepts_enter() {
    let page = attachment::AttachmentPage::new(0);
    let mut ui = Simulator::with_size(settings(), Size::new(460., 680.), page.preview("trigger"));
    ui.click(glyph("x")).unwrap();
    assert!(ui.find("research-summary.pdf").is_err());

    let mut ui = Simulator::with_size(settings(), Size::new(460., 680.), page.preview("trigger"));
    ui.operate(&mut Focus {
        target: 0,
        index: 0,
    });
    ui.tap_key(Named::Enter);
    ui.find("The attachment trigger fills the card and opens the dialog, while the actions stay independently clickable above it.").unwrap();
}

struct Focus {
    target: usize,
    index: usize,
}

impl Operation for Focus {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }
    fn focusable(&mut self, _: Option<&Id>, _: Rectangle, state: &mut dyn operation::Focusable) {
        if self.index == self.target {
            state.focus();
        } else {
            state.unfocus();
        }
        self.index += 1;
    }
}

#[derive(Default)]
struct Scroll {
    x: f32,
    end: f32,
}
impl Operation for Scroll {
    fn traverse(&mut self, visit: &mut dyn FnMut(&mut dyn Operation)) {
        visit(self);
    }
    fn scrollable(
        &mut self,
        _: Option<&Id>,
        bounds: Rectangle,
        content: Size,
        offset: Vector,
        _: &mut dyn operation::Scrollable,
    ) {
        self.x = offset.x;
        self.end = content.width - bounds.width;
    }
}

#[test]
fn keyboard_navigation_reveals_group_content_in_both_directions() {
    let page = attachment::AttachmentPage::new(0);
    let mut ui = Simulator::with_size(settings(), Size::new(320., 500.), page.preview("group"));
    let mut scroll = Scroll::default();
    ui.operate(&mut Focus {
        target: 0,
        index: 0,
    });
    ui.tap_key(Named::ArrowRight);
    ui.operate(&mut scroll);
    assert_eq!(scroll.x, 40.);
    ui.tap_key(Named::End);
    ui.operate(&mut scroll);
    assert_eq!(scroll.x, scroll.end);
    ui.tap_key(Named::Home);
    ui.operate(&mut scroll);
    assert_eq!(scroll.x, 0.);

    // The first tab stop is the group, followed by each file action.
    ui.operate(&mut Focus {
        target: 4,
        index: 0,
    });
    ui.operate(&mut scroll);
    assert!(
        scroll.x > 650.,
        "last file's remove button is now on screen"
    );
    ui.operate(&mut Focus {
        target: 1,
        index: 0,
    });
    ui.operate(&mut scroll);
    assert!(
        scroll.x < 245.,
        "first file's remove button is back in view"
    );
}

#[test]
fn the_full_card_border_opens_the_preview_without_swallowing_actions() {
    let page = attachment::AttachmentPage::new(0);
    let mut ui = Simulator::with_size(settings(), Size::new(384., 600.), page.preview("trigger"));
    ui.point_at(Point::new(1., 77.));
    ui.simulate(simulator::click());
    ui.find("The attachment trigger fills the card and opens the dialog, while the actions stay independently clickable above it.").unwrap();

    let mut ui = Simulator::with_size(settings(), Size::new(384., 600.), page.preview("trigger"));
    ui.click(glyph("copy")).unwrap();
    assert!(
        ui.drain()
            .any(|message| matches!(message, browser::Message::CopyText(_)))
    );
    assert!(ui.find("The attachment trigger fills the card and opens the dialog, while the actions stay independently clickable above it.").is_err());
}

#[test]
fn a_disabled_ancestor_disables_the_full_card_target() {
    use icedxdy::widget::attachment;
    use icedxdy::{Inherited, Slot};
    let mut card: Slot<'static, bool> = attachment("Disabled file")
        .trigger(|target| attachment::trigger(target).on_press(true).into())
        .into();
    card.inherit(&Inherited::default().with_disabled(true));
    let mut ui = Simulator::with_size(settings(), Size::new(200., 80.), card.compose());
    ui.point_at(Point::new(1., 10.));
    ui.simulate(simulator::click());
    ui.operate(&mut Focus {
        target: 0,
        index: 0,
    });
    ui.tap_key(Named::Enter);
    assert_eq!(ui.drain().count(), 0);
}

#[test]
fn repeated_escape_and_close_restore_the_attachment_trigger() {
    use iced_test::selector;
    use icedxdy::Theme;
    use std::time::Duration;
    let page = attachment::AttachmentPage::new(0);
    let mut ui: Simulator<'_, browser::Message, Theme, iced::Renderer> =
        Simulator::with_size(settings(), Size::new(384., 600.), page.preview("trigger"));
    for (key, close) in [
        (Named::Enter, false),
        (Named::Space, true),
        (Named::Enter, false),
    ] {
        ui.operate(&mut Focus {
            target: 0,
            index: 0,
        });
        ui.tap_key(key);
        ui = ui.rebuild(page.preview("trigger"));
        ui.find("The attachment trigger fills the card and opens the dialog, while the actions stay independently clickable above it.").unwrap();
        if close {
            ui.click("×").unwrap();
        } else {
            ui.tap_key(Named::Escape);
        }
        ui = ui.rebuild(page.preview("trigger"));
        std::thread::sleep(Duration::from_millis(170));
        ui.draw(&Theme::light());
        ui = ui.rebuild(page.preview("trigger"));
        assert!(ui.find("The attachment trigger fills the card and opens the dialog, while the actions stay independently clickable above it.").is_err());
        assert_eq!(
            ui.find(selector::is_focused()).unwrap().bounds().width,
            384.
        );
    }
}

#[test]
fn a_wheel_gesture_settles_on_the_next_card_with_scroll_padding() {
    use iced::mouse::ScrollDelta;
    use icedxdy::Theme;
    use std::time::Duration;
    let page = attachment::AttachmentPage::new(0);
    let mut ui: Simulator<'_, browser::Message, Theme, iced::Renderer> =
        Simulator::with_size(settings(), Size::new(384., 600.), page.preview("group"));
    ui.point_at(Point::new(150., 80.));
    ui.scroll(ScrollDelta::Pixels { x: -230., y: 0. });
    for _ in 0..10 {
        std::thread::sleep(Duration::from_millis(60));
        ui.draw(&Theme::light());
        ui = ui.rebuild(page.preview("group"));
    }
    let mut scroll = Scroll::default();
    ui.operate(&mut scroll);
    assert_eq!(scroll.x, 264., "matches the measured Chromium gesture");
    ui = ui.resize(Size::new(260., 600.));
    ui.draw(&Theme::light());
    ui.operate(&mut scroll);
    assert_eq!(scroll.x, 264.);
}

#[test]
fn arrow_keys_scroll_while_a_file_action_keeps_focus() {
    use icedxdy::Theme;
    use std::time::Duration;
    let page = attachment::AttachmentPage::new(0);
    let mut ui: Simulator<'_, browser::Message, Theme, iced::Renderer> =
        Simulator::with_size(settings(), Size::new(384., 600.), page.preview("group"));
    ui.operate(&mut Focus {
        target: 1,
        index: 0,
    });
    for (key, expected) in [
        (Named::ArrowRight, 264.),
        (Named::ArrowRight, 532.),
        (Named::ArrowLeft, 264.),
    ] {
        ui.tap_key(key);
        for _ in 0..10 {
            std::thread::sleep(Duration::from_millis(60));
            ui.draw(&Theme::light());
        }
        let mut scroll = Scroll::default();
        ui.operate(&mut scroll);
        assert_eq!(scroll.x, expected);
    }
}

#[test]
fn image_preview_and_remove_targets_publish_independent_messages() {
    let page = attachment::AttachmentPage::new(0);
    let mut ui = Simulator::with_size(settings(), Size::new(384., 600.), page.preview("image"));
    ui.click("workspace.png").unwrap();
    assert!(ui.drain().any(|message| matches!(message, browser::Message::OpenUrl("https://images.unsplash.com/photo-1497366754035-f200968a6e72?w=900&auto=format&fit=crop&q=80"))));
    ui.click(glyph("x")).unwrap();
    assert!(ui.find("workspace.png").is_err());
    assert_eq!(ui.drain().count(), 0);
}
