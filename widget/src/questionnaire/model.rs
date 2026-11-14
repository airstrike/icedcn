use std::collections::BTreeMap;

/// Stored answers include optional questions that were explicitly skipped.
pub type Answers = BTreeMap<String, Answer>;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Answer {
    pub choices: Vec<String>,
    pub text: String,
    pub skipped: bool,
}

impl Answer {
    pub fn answered(&self) -> bool {
        !self.choices.is_empty() || !self.text.trim().is_empty()
    }

    pub fn contains(&self, value: &str) -> bool {
        self.choices.iter().any(|s| s == value)
    }
}

pub struct Choice {
    pub(super) value: String,
    pub(super) label: String,
    pub(super) description: Option<String>,
}

pub fn choice(value: impl Into<String>, label: impl Into<String>) -> Choice {
    Choice {
        value: value.into(),
        label: label.into(),
        description: None,
    }
}

impl Choice {
    pub fn description(mut self, text: impl Into<String>) -> Self {
        self.description = Some(text.into());
        self
    }
}

pub struct Item {
    pub(super) name: String,
    pub(super) title: String,
    pub(super) description: Option<String>,
    pub(super) choices: Vec<Choice>,
    pub(super) input: Option<(String, String)>,
    pub(super) multiple: bool,
    pub(super) required: bool,
    enabled_when: Option<Condition>,
}

pub fn item(name: impl Into<String>, title: impl Into<String>) -> Item {
    Item {
        name: name.into(),
        title: title.into(),
        description: None,
        choices: Vec::new(),
        input: None,
        multiple: false,
        required: false,
        enabled_when: None,
    }
}

impl Item {
    pub fn choices(mut self, choices: impl IntoIterator<Item = Choice>) -> Self {
        self.choices = choices.into_iter().collect();
        self
    }
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }
    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }
    pub fn input(mut self, label: impl Into<String>, placeholder: impl Into<String>) -> Self {
        self.input = Some((label.into(), placeholder.into()));
        self
    }
    pub fn enabled_when(mut self, predicate: impl Fn(&Answers) -> bool + 'static) -> Self {
        self.enabled_when = Some(Box::new(predicate));
        self
    }
    pub(super) fn enabled(&self, answers: &Answers) -> bool {
        self.enabled_when.as_ref().is_none_or(|test| test(answers))
    }
    pub(super) fn choose(&self, answer: &mut Answer, index: usize) {
        let Some(choice) = self.choices.get(index) else {
            return;
        };
        answer.skipped = false;
        if self.multiple && answer.contains(&choice.value) {
            answer.choices.retain(|s| s != &choice.value);
        } else {
            if !self.multiple {
                answer.choices.clear();
                answer.text.clear();
            }
            answer.choices.push(choice.value.clone());
        }
    }
    pub(super) fn write(&self, answer: &mut Answer, value: String) {
        answer.text = value;
        answer.skipped = false;
        if !self.multiple && !answer.text.is_empty() {
            answer.choices.clear();
        }
    }
}

type Condition = Box<dyn Fn(&Answers) -> bool>;
