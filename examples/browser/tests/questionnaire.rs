//! Exercise the registered native previews, including host-controlled navigation.
use browser::Message;
use browser::component::questionnaire::QuestionnairePage;
use browser::page::{self, Page};
use iced::{Point, Size};
use iced_test::simulator::Simulator;
use icedcn::Theme;
use std::path::PathBuf;

type Ui<'a> = Simulator<'a, Message, Theme, iced::Renderer>;

fn preview<'a>(page: &'a dyn Page, id: &str) -> Ui<'a> {
    Simulator::with_size(Default::default(), Size::new(448., 680.), page.preview(id))
}

fn click(ui: &mut Ui<'_>, label: &str) {
    ui.click(label).unwrap();
    // A Field label activates its native control on the requested redraw.
    ui.draw(&Theme::light());
}

fn notice(ui: &mut Ui<'_>) -> (String, String) {
    ui.drain()
        .find_map(|m| match m {
            Message::Page(12, page::Message::Notice(title, description)) => {
                Some((title, description))
            }
            _ => None,
        })
        .expect("a successful submission reaches the native toaster")
}
#[test]
fn required_validation_skip_and_submission_retain_the_actual_answers() {
    let page = QuestionnairePage::new(12);
    let mut ui = preview(&*page, "demo");
    click(&mut ui, "Next");
    ui.find("Please answer this question.")
        .expect("required question");
    click(&mut ui, "Approval checkpoints");
    click(&mut ui, "Next");
    ui.find("Question 2 of 3").unwrap();
    click(&mut ui, "Skip");
    click(&mut ui, "Start now");
    click(&mut ui, "Save plan");
    let (title, detail) = notice(&mut ui);
    assert_eq!(title, "Agent plan saved");
    assert!(
        detail.contains("approvals")
            && detail.contains("Progress signals: None")
            && detail.contains("now"),
        "{detail}"
    );
}

#[test]
fn validation_returns_to_the_invalid_item_and_resubmission_succeeds() {
    let page = QuestionnairePage::new(12);
    let mut ui = preview(&*page, "validation");
    click(&mut ui, "Concise summary");
    click(&mut ui, "Next");
    click(&mut ui, "Public audience");
    click(&mut ui, "Validate answers");
    ui.find("Public answers need enough context. Choose a complete answer.")
        .unwrap();
    click(&mut ui, "Complete answer");
    click(&mut ui, "Next");
    click(&mut ui, "Validate answers");
    let (_, detail) = notice(&mut ui);
    assert!(detail.contains("complete") && detail.contains("public"));
}

#[test]
fn conditional_path_and_resume_use_saved_state() {
    let page = QuestionnairePage::new(12);
    let mut ui = preview(&*page, "conditional");
    ui.find("Question 1 of 2").unwrap();
    click(&mut ui, "Cloud workspace");
    ui.find("Question 1 of 3").unwrap();
    click(&mut ui, "Next");
    click(&mut ui, "Staging");
    click(&mut ui, "Previous");
    click(&mut ui, "Local workspace");
    click(&mut ui, "Next");
    click(&mut ui, "Before writing files");
    click(&mut ui, "Save execution plan");
    let (_, detail) = notice(&mut ui);
    assert!(detail.contains("local") && !detail.contains("staging"));
    let mut ui = preview(&*page, "resume");
    ui.find("Question 2 of 3").unwrap();
    click(&mut ui, "Next");
    ui.find("Keep the existing public API stable.").unwrap();
    click(&mut ui, "Reset changes");
    ui.find("Question 2 of 3").unwrap();
}

#[test]
fn shortcuts_select_answers_and_scoped_editors_consume_typed_characters() {
    let page = QuestionnairePage::new(12);
    let mut ui = preview(&*page, "multiple");
    ui.point_at(Point::new(20., 35.));
    ui.typewrite("ac");
    click(&mut ui, "Share context");
    let (_, detail) = notice(&mut ui);
    assert!(
        detail.contains("source") && detail.contains("docs"),
        "{detail}"
    );
    let mut ui = preview(&*page, "freeform");
    click(&mut ui, "Describe another approach…");
    ui.typewrite("abc");
    click(&mut ui, "Use this approach");
    let (_, detail) = notice(&mut ui);
    assert!(detail.contains("abc"), "{detail}");
}

#[test]
fn external_navigation_and_disabled_actions_are_live() {
    let page = QuestionnairePage::new(12);
    let mut ui = preview(&*page, "controlled");
    ui.find("Current checkpoint: Change scope").unwrap();
    click(&mut ui, "Only the target component");
    click(&mut ui, "Next");
    ui.find("Current checkpoint: Verification").unwrap();
    let mut ui = preview(&*page, "navigation-state");
    click(&mut ui, "Next");
    ui.find("What may the agent modify?").unwrap();
    assert!(ui.find("Please answer this question.").is_err());
    click(&mut ui, "Project files");
    click(&mut ui, "Next");
    ui.find("What must pass before completion?").unwrap();
}

#[test]
fn every_questionnaire_preview_renders_at_phone_width() {
    let page = QuestionnairePage::new(12);
    for id in [
        "demo",
        "multiple",
        "freeform",
        "skip",
        "shortcuts",
        "validation",
        "controlled",
        "resume",
        "conditional",
        "navigation-state",
        "progress",
        "animated",
        "card",
        "dialog",
    ] {
        let mut ui =
            Simulator::with_size(Default::default(), Size::new(375., 680.), page.preview(id));
        if id == "demo" {
            let placeholder = ui.find("Describe another feature…").unwrap();
            assert!(
                placeholder.bounds().width > 80.,
                "freeform placeholder stays readable on a phone"
            );
        }
        if let Some(label) = match id {
            "validation" => Some("Concise summary"),
            "card" => Some("Fix the failing tests"),
            _ => None,
        } {
            assert!(
                ui.find(label).unwrap().bounds().width > 80.,
                "the card leaves room for the choice label on a phone"
            );
        }
        if id == "dialog" {
            click(&mut ui, "Open clarification");
            ui = ui.rebuild(page.preview(id));
            ui.find("Which files are in scope?").unwrap();
        }
        if let Some(dir) = std::env::var_os("ICEDCN_QUESTIONNAIRE_SNAPSHOTS") {
            let file = PathBuf::from(dir).join(format!("questionnaire-{id}-375.png"));
            assert!(
                ui.snapshot(&Theme::light())
                    .unwrap()
                    .matches_image(file)
                    .unwrap()
            );
        } else {
            ui.draw(&Theme::light());
        }
    }
}
