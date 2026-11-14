//! Exercise the independent controls rendered by the installed Attachment recipes.
use std::borrow::Cow;

use browser::component::attachment;
use browser::icon;
use iced::keyboard::key::Named;
use iced::{Rectangle, Size, Vector};
use iced_test::core::Settings;
use iced_test::simulator::Simulator;
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
    ui.click(glyph("refresh_ccw")).unwrap();
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
