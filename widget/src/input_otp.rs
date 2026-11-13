//! One-time code entry with a single editable value and individually painted slots.

use crate::compose::{Compose, Inherited, Slot};
use crate::text_input::{TextInput, element};
use iced_core::Element;
use iced_widget::{Renderer, Widget};
use icedxdy_core::Theme;
use icedxdy_core::theme::text_input;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Pattern {
    /// The default accepts printable ASCII, as in the unrestricted demo.
    #[default]
    Any,
    Digits,
    Alphanumeric,
}

#[derive(Clone, Debug)]
pub(crate) struct Slots {
    pub(crate) count: usize,
    pub(crate) group: usize,
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) pattern: Pattern,
}

impl Slots {
    pub(crate) fn x(&self, index: usize) -> f32 {
        index as f32 * self.width + (index / self.group) as f32 * 24.
    }

    pub(crate) fn total_width(&self) -> f32 {
        self.x(self.count - 1) + self.width
    }

    pub(crate) fn hit(&self, x: f32) -> usize {
        (0..self.count)
            .find(|&i| x < self.x(i) + self.width * 0.5)
            .unwrap_or(self.count)
    }

    pub(crate) fn accepts(&self, c: char) -> bool {
        match self.pattern {
            Pattern::Any => c.is_ascii_graphic() || c == ' ',
            Pattern::Digits => c.is_ascii_digit(),
            Pattern::Alphanumeric => c.is_ascii_alphanumeric(),
        }
    }

    pub(crate) fn normalize(&self, text: &str) -> String {
        text.chars()
            .filter(|c| self.accepts(*c))
            .take(self.count)
            .collect()
    }
}

pub struct InputOtp<'a, Message> {
    slots: Slots,
    value: Option<String>,
    default_value: String,
    disabled: bool,
    invalid: bool,
    on_change: Option<Box<dyn Fn(String) -> Message + 'a>>,
}

/// Create `length` slots; zero requests are clamped to one.
pub fn input_otp<'a, Message>(length: usize) -> InputOtp<'a, Message> {
    let count = length.max(1);
    InputOtp {
        slots: Slots {
            count,
            group: count,
            width: 32.,
            height: 32.,
            pattern: Pattern::Any,
        },
        value: None,
        default_value: String::new(),
        disabled: false,
        invalid: false,
        on_change: None,
    }
}

impl<'a, Message> InputOtp<'a, Message> {
    /// Separate groups of this many slots with a dash.
    pub fn group_size(mut self, count: usize) -> Self {
        self.slots.group = count.max(1);
        self
    }
    pub fn cell_size(mut self, width: f32, height: f32) -> Self {
        self.slots.width = width.max(1.);
        self.slots.height = height.max(1.);
        self
    }
    pub fn pattern(mut self, pattern: Pattern) -> Self {
        self.slots.pattern = pattern;
        self
    }
    pub fn default_value(mut self, value: impl Into<String>) -> Self {
        self.default_value = value.into();
        self
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn on_value_change(mut self, change: impl Fn(String) -> Message + 'a) -> Self {
        self.on_change = Some(Box::new(change));
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }
}

impl<'a, Message: Clone + 'a> Compose<Message> for InputOtp<'a, Message> {
    fn inherit(&mut self, props: &Inherited) {
        self.disabled |= props.is_disabled();
    }
    fn compose<'b>(self: Box<Self>) -> Element<'b, Message, Theme, Renderer>
    where
        Self: 'b,
    {
        let InputOtp {
            slots,
            value,
            default_value,
            disabled,
            invalid,
            on_change,
        } = *self;
        let value = value.map(|v| slots.normalize(&v));
        let mut edit = match value {
            Some(value) => TextInput::new("", &value),
            None => TextInput::new("", "").uncontrolled(&slots.normalize(&default_value)),
        };
        if let Some(change) = on_change {
            edit = edit.on_input(change);
        }
        if invalid {
            edit =
                edit.style(|theme, status| element::from_core(text_input::invalid(theme, status)));
        }
        edit.disabled(disabled).slots(slots).boxed()
    }
}

impl<'a, Message: Clone + 'a> From<InputOtp<'a, Message>> for Slot<'a, Message> {
    fn from(input: InputOtp<'a, Message>) -> Self {
        Slot::new(input)
    }
}
impl<'a, Message: Clone + 'a> From<InputOtp<'a, Message>>
    for Element<'a, Message, Theme, Renderer>
{
    fn from(input: InputOtp<'a, Message>) -> Self {
        Box::new(input).compose()
    }
}
