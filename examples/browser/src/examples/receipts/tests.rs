use iced::advanced::widget::operation::Outcome;
use iced::futures::{StreamExt, executor::block_on};
use iced::widget::Id;
use iced::{Element, Fill, Task};
use iced_test::core::Settings;
use iced_test::runtime::{Action, task};
use iced_test::selector::{self, Target};
use iced_test::simulator::Simulator;
use icedcn::widget::{button, column};

use super::sale::{self, edit, show};
use super::{Hotkey, Message, Screen, State, list};

fn edit_message(state: &mut State, message: edit::Message) -> Task<Message> {
    let id = match state.screen {
        Screen::Sale(_, id) => id,
        Screen::List => panic!("expected sale screen"),
    };
    state.update(Message::Sale(id, sale::Message::Edit(message)))
}

fn show_message(state: &mut State, message: show::Message) -> Task<Message> {
    let id = match state.screen {
        Screen::Sale(_, id) => id,
        Screen::List => panic!("expected sale screen"),
    };
    state.update(Message::Sale(id, sale::Message::Show(message)))
}

fn apply(ui: &mut Simulator<'_, Message, icedcn::Theme, iced::Renderer>, effect: Task<Message>) {
    let Some(mut actions) = task::into_stream(effect) else {
        return;
    };
    while let Some(action) = block_on(actions.next()) {
        if let Action::Widget(mut operation) = action {
            loop {
                ui.operate(&mut operation);
                match operation.finish() {
                    Outcome::Chain(next) => operation = next,
                    _ => break,
                }
            }
        }
    }
}

fn assert_focused(state: &State, effect: Task<Message>, id: Id) {
    let view: Element<'_, Message, icedcn::Theme> = column![button("Home"), state.view()]
        .width(Fill)
        .height(Fill)
        .into();
    let mut ui = Simulator::with_size(Settings::default(), iced::Size::new(1000., 800.), view);
    apply(&mut ui, effect);
    assert!(
        matches!(ui.find(selector::is_focused()), Ok(Target::Focusable { id: Some(actual), .. }) if actual == id)
    );
}

#[test]
fn original_empty_state_draft_workflow_and_focus_tasks_are_preserved() {
    let mut state = State::default();
    assert!(state.sales.is_empty());
    let mut ui = Simulator::with_size(
        Settings::default(),
        iced::Size::new(800., 600.),
        state.view(),
    );
    ui.click("Create your first sale →").unwrap();
    let new = ui.into_messages().next().unwrap();
    let effect = state.update(new);
    assert_focused(&state, effect, Id::from("receipts-sale-name"));
    assert!(state.draft.1.items.is_empty());
    let _ = edit_message(&mut state, edit::Message::NameInput("Lunch".into()));
    let _ = edit_message(&mut state, edit::Message::NameSubmit);
    assert_eq!(state.draft.1.items.len(), 1);
    let item_id = state.draft.1.items[0].id;
    assert_eq!(state.draft.1.items[0].quantity_string(), "");
    assert_eq!(state.draft.1.items[0].price_string(), "");
    let effect = edit_message(&mut state, edit::Message::SubmitItem(item_id));
    assert_focused(&state, effect, edit::form_id("name", item_id));
    let _ = edit_message(
        &mut state,
        edit::Message::UpdateItem(item_id, edit::Field::Name("Soup".into())),
    );
    let effect = edit_message(&mut state, edit::Message::SubmitItem(item_id));
    assert_focused(&state, effect, edit::form_id("quantity", item_id));
    let _ = edit_message(
        &mut state,
        edit::Message::UpdateItem(item_id, edit::Field::Quantity("2".into())),
    );
    let effect = edit_message(&mut state, edit::Message::SubmitItem(item_id));
    assert_focused(&state, effect, edit::form_id("price", item_id));
    let _ = edit_message(
        &mut state,
        edit::Message::UpdateItem(item_id, edit::Field::Price("3.50".into())),
    );
    let effect = edit_message(&mut state, edit::Message::SubmitItem(item_id));
    assert_eq!(state.draft.1.items.len(), 2);
    assert_focused(
        &state,
        effect,
        edit::form_id("name", state.draft.1.items[1].id),
    );
    let blank_id = state.draft.1.items[1].id;
    let _ = edit_message(&mut state, edit::Message::RemoveItem(blank_id));
    let _ = edit_message(&mut state, edit::Message::UpdateServiceCharge(10.));
    let _ = edit_message(&mut state, edit::Message::UpdateGratuity(1.));
    assert!((state.draft.1.calculate_total() - 9.26).abs() < 0.001);

    // Preserve the original fixed table widths and top actions/bottom totals.
    for (width, height) in [(800., 600.), (1200., 900.)] {
        let mut ui = Simulator::with_size(
            Settings::default(),
            iced::Size::new(width, height),
            state.view(),
        );
        let qty = ui
            .find(edit::form_id("quantity", item_id))
            .unwrap()
            .bounds();
        let price = ui.find(edit::form_id("price", item_id)).unwrap().bounds();
        assert_eq!(qty.width, 80.);
        assert_eq!(price.width, 100.);
        assert!(ui.find("Save").unwrap().bounds().y < 80.);
        assert!(ui.find("Subtotal").unwrap().bounds().y > height - 240.);
        ui.find("Tax Group").unwrap();
        ui.find("+ Add Item").unwrap();
    }
    let _ = edit_message(&mut state, edit::Message::Save);
    assert!(matches!(
        state.screen,
        Screen::Sale(sale::Mode::View, Some(1))
    ));
    let effect = show_message(&mut state, show::Message::StartEdit);
    assert_focused(&state, effect, Id::from("receipts-sale-name"));
    let _ = edit_message(&mut state, edit::Message::NameInput("Discard".into()));
    let _ = state.update(Message::Hotkey(Hotkey::Escape));
    assert!(
        matches!(state.screen, Screen::Sale(sale::Mode::Edit, _)),
        "Escape has no edit action in the original"
    );
    let _ = edit_message(&mut state, edit::Message::Cancel);
    assert_eq!(state.draft.1.name, "Lunch");
    let _ = state.update(Message::Hotkey(Hotkey::Escape));
    assert!(matches!(state.screen, Screen::List));
    let _ = state.update(Message::List(list::Message::NewSale));
    let _ = edit_message(&mut state, edit::Message::Cancel);
    assert!(
        matches!(state.screen, Screen::Sale(sale::Mode::View, None)),
        "Cancel on a new sale shows its blank draft"
    );
}

#[test]
fn save_keeps_the_original_unvalidated_blank_sale_behavior() {
    let mut state = State::default();
    let _ = state.update(Message::List(list::Message::NewSale));
    let _ = edit_message(&mut state, edit::Message::Save);
    assert_eq!(state.sales.len(), 1);
    assert!(state.sales[&1].name.is_empty());
    assert!(state.sales[&1].items.is_empty());
}
