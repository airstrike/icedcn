use iced::keyboard::Modifiers;
use iced::{Element, Task, Widget};
use icedcn::Theme;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

mod action;
mod list;
mod operation;
mod sale;
mod tax;

pub use action::Action;
const ROOT: &str = "receipts-viewport";
use sale::Sale;

#[derive(Debug)]
enum Screen {
    List,
    Sale(sale::Mode, Option<usize>),
}

#[derive(Debug, Clone)]
pub enum Message {
    List(list::Message),
    Sale(Option<usize>, sale::Message),
    Hotkey(Hotkey),
}

#[derive(Debug)]
enum Instruction {
    Sale(Option<usize>, sale::Instruction),
}

pub struct State {
    screen: Screen,
    sales: HashMap<usize, sale::Sale>,
    draft: (Option<usize>, sale::Sale),
    next_sale_id: AtomicUsize,
}

impl State {
    pub fn title(&self) -> String {
        match self.screen {
            Screen::List => "iced Receipts".to_string(),
            Screen::Sale(mode, id) => {
                let sale_name = if self.draft.0 == id {
                    self.draft.1.name.clone()
                } else {
                    self.sales[&id.expect("saved sale has an id")].name.clone()
                };

                let sale_name = format!(
                    "{} {}",
                    if sale_name.is_empty() {
                        "Untitled sale"
                    } else {
                        &sale_name
                    },
                    id.map_or("".to_string(), |id| format!("(#{id})"))
                );

                match mode {
                    sale::Mode::View => {
                        format!("iced Receipts • {}", sale_name)
                    }
                    sale::Mode::Edit => {
                        format!("iced Receipts • {} • Edit", sale_name)
                    }
                }
            }
        }
    }

    pub fn new() -> (Self, Task<Message>) {
        let initial_id = 0;
        (
            Self {
                screen: Screen::List,
                sales: HashMap::new(),
                draft: (None, Sale::default()),
                next_sale_id: AtomicUsize::new(initial_id + 1),
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::List(list::Message::NewSale) => {
                self.draft = (None, Sale::default());
                self.screen = Screen::Sale(sale::Mode::Edit, None);
                return operation::focus_next();
            }
            Message::List(list::Message::SelectSale(id)) => {
                self.screen = Screen::Sale(sale::Mode::View, Some(id));
            }
            Message::Hotkey(hotkey) => match self.screen {
                Screen::List => {}
                Screen::Sale(mode, sale_id) => {
                    let sale = if self.draft.0 == sale_id {
                        &mut self.draft.1
                    } else {
                        self.sales
                            .get_mut(&sale_id.expect("saved sale has an id"))
                            .expect("Sale should exist")
                    };

                    let action = sale::handle_hotkey(sale, mode, hotkey)
                        .map_instruction(move |o| Instruction::Sale(sale_id, o))
                        .map(move |m| Message::Sale(sale_id, m));

                    let instruction_task = if let Some(instruction) = action.instruction {
                        self.perform(instruction)
                    } else {
                        Task::none()
                    };

                    return instruction_task.chain(action.task);
                }
            },
            Message::Sale(sale_id, msg) => {
                let sale = if self.draft.0 == sale_id {
                    &mut self.draft.1
                } else {
                    self.sales
                        .get_mut(&sale_id.expect("saved sale has an id"))
                        .expect("Sale should exist")
                };

                let action = sale::update(sale, msg)
                    .map_instruction(move |o| Instruction::Sale(sale_id, o))
                    .map(move |m| Message::Sale(sale_id, m));

                let instruction_task = if let Some(instruction) = action.instruction {
                    self.perform(instruction)
                } else {
                    Task::none()
                };

                return instruction_task.chain(action.task);
            }
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message, Theme> {
        match &self.screen {
            Screen::List => list::view(&self.sales).map(Message::List).boxed(),
            Screen::Sale(mode, id) => {
                let sale = if self.draft.0 == *id {
                    &self.draft.1
                } else {
                    &self.sales[&id.expect("saved sale has an id")]
                };
                sale::view(sale, *mode)
                    .map(|msg| Message::Sale(*id, msg))
                    .boxed()
            }
        }
    }

    fn perform(&mut self, instruction: Instruction) -> Task<Message> {
        match instruction {
            Instruction::Sale(sale_id, instruction) => match instruction {
                sale::Instruction::Back => match self.screen {
                    Screen::List => {}
                    Screen::Sale(mode, _) => match mode {
                        sale::Mode::Edit => self.screen = Screen::Sale(sale::Mode::View, sale_id),
                        sale::Mode::View => self.screen = Screen::List,
                    },
                },
                sale::Instruction::Save => {
                    let final_id = match self.draft.0 {
                        Some(id) => {
                            // Editing existing sale
                            self.sales.insert(id, self.draft.1.clone());
                            id
                        }
                        None => {
                            // Creating new sale
                            let new_id = self.next_sale_id.fetch_add(1, Ordering::SeqCst);
                            self.sales.insert(new_id, std::mem::take(&mut self.draft.1));
                            self.draft.1 = Sale::default();
                            new_id
                        }
                    };
                    self.screen = Screen::Sale(sale::Mode::View, Some(final_id));
                }
                sale::Instruction::StartEdit => {
                    if let Some(id) = sale_id {
                        // Start editing existing sale
                        self.draft = (Some(id), self.sales[&id].clone());
                    }
                    self.screen = Screen::Sale(sale::Mode::Edit, sale_id);
                }
                sale::Instruction::Cancel => {
                    match sale_id {
                        Some(id) => {
                            // Restore draft from original sale
                            self.draft = (Some(id), self.sales[&id].clone());
                        }
                        None => {
                            // Reset to blank draft
                            self.draft = (None, Sale::default());
                        }
                    }
                    self.screen = Screen::Sale(sale::Mode::View, sale_id);
                }
            },
        }
        Task::none()
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new().0
    }
}

#[derive(Debug, Clone)]
pub enum Hotkey {
    Escape,
    Tab(Modifiers),
}

#[cfg(test)]
mod tests;
