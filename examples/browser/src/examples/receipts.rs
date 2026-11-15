//! A working remake of airstrike/iced_receipts with icedcn controls.
mod receipt;
mod view;

use iced::widget::operation;
use iced::{Element, Task};
use icedcn::Theme;

use receipt::{Item, Receipt, Tax};

pub struct State {
    sales: Vec<Receipt>,
    screen: Screen,
    next_id: u64,
    search: String,
    error: Option<&'static str>,
}

enum Screen {
    List,
    Show(u64),
    Edit {
        original: Option<u64>,
        draft: Receipt,
    },
}

#[derive(Debug, Clone)]
pub enum Message {
    New,
    Show(u64),
    Edit,
    Back,
    Save,
    Search(String),
    Name(String),
    AddItem,
    RemoveItem(u64),
    ItemName(u64, String),
    Quantity(u64, String),
    Price(u64, String),
    Tax(u64, String),
    Service(String),
    Gratuity(String),
    NextField,
}

impl Default for State {
    fn default() -> Self {
        Self {
            sales: Receipt::samples(),
            screen: Screen::List,
            next_id: 3,
            search: String::new(),
            error: None,
        }
    }
}

impl State {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::New => {
                self.error = None;
                self.screen = Screen::Edit {
                    original: None,
                    draft: Receipt::blank(self.next_id),
                };
                return operation::focus_next();
            }
            Message::Show(id) if self.sales.iter().any(|sale| sale.id == id) => {
                self.screen = Screen::Show(id);
            }
            Message::Edit => {
                if let Screen::Show(id) = self.screen
                    && let Some(sale) = self.sales.iter().find(|sale| sale.id == id)
                {
                    self.error = None;
                    self.screen = Screen::Edit {
                        original: Some(id),
                        draft: sale.clone(),
                    };
                    return operation::focus_next();
                }
            }
            Message::Back => {
                self.error = None;
                self.screen = match self.screen {
                    Screen::Edit {
                        original: Some(id), ..
                    } => Screen::Show(id),
                    _ => Screen::List,
                };
            }
            Message::Save => {
                if let Screen::Edit { original, draft } = &mut self.screen {
                    self.error = draft.error();
                    if self.error.is_some() {
                        return Task::none();
                    }
                    draft.name = draft.name.trim().into();
                    draft.recalculate();
                    let id = draft.id;
                    if let Some(sale) = self
                        .sales
                        .iter_mut()
                        .find(|sale| Some(sale.id) == *original)
                    {
                        *sale = draft.clone();
                    } else {
                        self.sales.push(draft.clone());
                        self.next_id += 1;
                    }
                    self.screen = Screen::Show(id);
                }
            }
            Message::Search(value) => self.search = value,
            Message::NextField => return operation::focus_next(),
            message => {
                if let Screen::Edit { draft, .. } = &mut self.screen {
                    match message {
                        Message::Name(value) => draft.name = value,
                        Message::Service(value) => draft.service = value,
                        Message::Gratuity(value) => draft.gratuity = value,
                        Message::AddItem => {
                            let id = draft
                                .items
                                .iter()
                                .map(|item| item.id)
                                .max()
                                .map_or(0, |id| id + 1);
                            draft.items.push(Item::blank(id));
                        }
                        Message::RemoveItem(id) => draft.items.retain(|item| item.id != id),
                        Message::ItemName(id, value) => {
                            if let Some(item) = draft.items.iter_mut().find(|item| item.id == id) {
                                item.name = value;
                            }
                        }
                        Message::Quantity(id, value) => {
                            if let Some(item) = draft.items.iter_mut().find(|item| item.id == id) {
                                item.quantity = value;
                            }
                        }
                        Message::Price(id, value) => {
                            if let Some(item) = draft.items.iter_mut().find(|item| item.id == id) {
                                item.price = value;
                            }
                        }
                        Message::Tax(id, value) => {
                            if let Some(item) = draft.items.iter_mut().find(|item| item.id == id)
                                && let Some(tax) =
                                    Tax::ALL.into_iter().find(|tax| tax.key() == value)
                            {
                                item.tax = tax;
                            }
                        }
                        _ => {}
                    }
                    draft.recalculate();
                    if self.error.is_some() {
                        self.error = draft.error();
                    }
                }
            }
        }
        Task::none()
    }

    pub fn view(&self, width: f32) -> Element<'_, Message, Theme> {
        match &self.screen {
            Screen::List => view::list(&self.sales, &self.search),
            Screen::Show(id) => match self.sales.iter().find(|sale| sale.id == *id) {
                Some(sale) => view::show(sale, width),
                None => view::list(&self.sales, &self.search),
            },
            Screen::Edit { original, draft } => {
                view::edit(draft, original.is_some(), self.error, width)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn draft(state: &State) -> &Receipt {
        match &state.screen {
            Screen::Edit { draft, .. } => draft,
            _ => panic!("expected editor"),
        }
    }

    #[test]
    fn visible_form_fields_create_a_receipt_on_desktop_and_mobile() {
        use iced_test::core::Settings;
        use iced_test::simulator::Simulator;

        for width in [375., 1100.] {
            let mut state = State::default();
            let _ = state.update(Message::New);
            for (label, value) in [
                ("Sale name", "Breakfast"),
                ("Item", "Coffee"),
                ("Unit price ($)", "3.50"),
            ] {
                let mut ui = Simulator::with_size(
                    Settings::default(),
                    iced::Size::new(width, 2000.),
                    state.view(width),
                );
                ui.click(label).unwrap();
                ui.typewrite(value);
                let messages: Vec<_> = ui.into_messages().collect();
                for message in messages {
                    let _ = state.update(message);
                }
            }
            let mut ui = Simulator::with_size(
                Settings::default(),
                iced::Size::new(width, 2000.),
                state.view(width),
            );
            ui.click("Save receipt").unwrap();
            let messages: Vec<_> = ui.into_messages().collect();
            for message in messages {
                let _ = state.update(message);
            }
            assert!(matches!(state.screen, Screen::Show(3)));
            assert_eq!(state.sales[2].name, "Breakfast");
            assert_eq!(state.sales[2].items[0].name, "Coffee");
            assert_eq!(state.sales[2].totals.total, 378);
        }
    }

    #[test]
    fn create_edit_cancel_and_reopen_preserve_saved_receipts() {
        let mut state = State::default();
        let _ = state.update(Message::New);
        let _ = state.update(Message::Name("Breakfast".into()));
        let _ = state.update(Message::ItemName(0, "Coffee".into()));
        let _ = state.update(Message::Price(0, "3.50".into()));
        let _ = state.update(Message::Quantity(0, "2".into()));
        let _ = state.update(Message::Service("10".into()));
        let _ = state.update(Message::Gratuity("1.00".into()));
        assert_eq!(draft(&state).totals.total, 926);
        let _ = state.update(Message::Save);
        assert!(matches!(state.screen, Screen::Show(3)));
        let _ = state.update(Message::Edit);
        let _ = state.update(Message::Name("Discard me".into()));
        let _ = state.update(Message::Price(0, "99.00".into()));
        let _ = state.update(Message::Back);
        assert_eq!(state.sales[2].name, "Breakfast");
        assert_eq!(state.sales[2].totals.total, 926);
        let _ = state.update(Message::Back);
        let _ = state.update(Message::Show(3));
        let _ = state.update(Message::Edit);
        assert_eq!(draft(&state).items[0].price, "3.50");
        let _ = state.update(Message::Price(0, "4.00".into()));
        let _ = state.update(Message::Save);
        assert_eq!(state.sales.len(), 3);
        assert_eq!(state.sales[2].totals.total, 1044);
    }

    #[test]
    fn invalid_numbers_stay_editable_and_do_not_overwrite_a_sale() {
        let mut state = State::default();
        let _ = state.update(Message::Show(1));
        let _ = state.update(Message::Edit);
        let original = state.sales[0].totals;
        for invalid in ["-1", "NaN", "1.234", "100000000000000000000"] {
            let _ = state.update(Message::Price(0, invalid.into()));
            let _ = state.update(Message::Save);
            assert!(state.error.is_some());
            assert_eq!(draft(&state).items[0].price, invalid);
            assert_eq!(state.sales[0].totals, original);
        }
    }

    #[test]
    fn every_tax_group_and_item_removal_recalculate_the_total() {
        let mut state = State::default();
        let _ = state.update(Message::New);
        let _ = state.update(Message::Price(0, "10.00".into()));
        for (tax, total) in [
            ("food", 1080),
            ("alcohol", 1100),
            ("exempt", 1000),
            ("other", 1080),
        ] {
            let _ = state.update(Message::Tax(0, tax.into()));
            assert_eq!(draft(&state).totals.total, total);
        }
        let _ = state.update(Message::AddItem);
        let _ = state.update(Message::RemoveItem(0));
        assert_eq!(draft(&state).items.len(), 1);
        assert_eq!(draft(&state).totals.total, 0);
        let _ = state.update(Message::Back);
        assert!(matches!(state.screen, Screen::List));
        assert_eq!(state.sales.len(), 2);
    }
}
