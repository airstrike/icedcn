use iced::widget::component::{Component, component};
use iced::{Element, Fill, Widget};
use icedcn::widget::{button, field, input, row, select};
use icedcn::{Slot, Theme};

use crate::Message;

pub(super) fn group(text_size: f32) -> Element<'static, Message, Theme> {
    component(Example::Group(text_size)).boxed()
}

pub(super) fn form(text_size: f32) -> Element<'static, Message, Theme> {
    component(Example::Form(text_size)).boxed()
}

enum Example {
    Group(f32),
    Form(f32),
}

#[derive(Default)]
struct Values {
    name: String,
    email: String,
    phone: String,
    country: Option<String>,
    address: String,
    submitted: bool,
}

#[derive(Clone, Debug)]
enum Edit {
    Name(String),
    Email(String),
    Phone(String),
    Country(String),
    Address(String),
    Reset,
    Submit,
}

impl Values {
    fn update(&mut self, edit: Edit) {
        match edit {
            Edit::Name(name) => self.name = name,
            Edit::Email(email) => self.email = email,
            Edit::Phone(phone) => self.phone = phone,
            Edit::Country(country) => self.country = Some(country),
            Edit::Address(address) => self.address = address,
            Edit::Reset => *self = Self::default(),
            Edit::Submit => self.submitted = true,
        }
    }

    fn group<'a>(&self, text_size: f32) -> Slot<'a, Edit> {
        field::group([
            field(
                input("Jordan Lee")
                    .text_size(text_size)
                    .width(Fill)
                    .value(self.name.clone())
                    .on_input(Edit::Name),
            )
            .label("Name")
            .into(),
            field(
                input("name@example.com")
                    .text_size(text_size)
                    .width(Fill)
                    .value(self.email.clone())
                    .on_input(Edit::Email),
            )
            .label("Email")
            .description("We'll send updates to this address.")
            .into(),
            row![
                button("Reset").style(button::Outline).on_press(Edit::Reset),
                button("Submit").on_press(Edit::Submit)
            ]
            .spacing(8)
            .into(),
        ])
        .width(320)
        .into()
    }

    fn form<'a>(&self, text_size: f32) -> Slot<'a, Edit> {
        let entry = |label: &'a str, hint: &'a str, value: &str, edit: fn(String) -> Edit| {
            field(
                input(hint)
                    .text_size(text_size)
                    .width(Fill)
                    .value(value.to_owned())
                    .on_input(edit)
                    .on_submit(Edit::Submit),
            )
            .label(label)
        };
        let mut name = entry("Name", "Evil Rabbit", &self.name, Edit::Name);
        let mut email = entry("Email", "john@example.com", &self.email, Edit::Email)
            .description("We'll never share your email with anyone.");
        if self.submitted && self.name.is_empty() {
            name = name.error("Please fill out this field.");
        } else if self.submitted && !self.email.is_empty() && !valid_email(&self.email) {
            email = email.error("Enter an email address.");
        }
        let country = select(
            [
                ("us", "United States"),
                ("uk", "United Kingdom"),
                ("ca", "Canada"),
            ]
            .map(|(value, label)| select::item(value, label)),
        )
        .value(self.country.as_deref().unwrap_or("us").to_owned())
        .on_value_change(Edit::Country)
        .width(Fill);
        field::group([
            name.into(),
            email.into(),
            row![
                entry("Phone", "+1 (555) 123-4567", &self.phone, Edit::Phone).width(Fill),
                field(country).label("Country").width(Fill)
            ]
            .spacing(16)
            .width(Fill)
            .into(),
            entry("Address", "123 Main St", &self.address, Edit::Address).into(),
            row![
                button("Cancel")
                    .style(button::Outline)
                    .on_press(Edit::Reset),
                button("Submit").on_press(Edit::Submit)
            ]
            .spacing(8)
            .into(),
        ])
        .width(384)
        .into()
    }
}

fn valid_email(email: &str) -> bool {
    email.split_once('@').is_some_and(|(local, domain)| {
        !local.is_empty()
            && !domain.is_empty()
            && !domain.contains('@')
            && !email.contains(char::is_whitespace)
    })
}

impl<'a> Component<'a, Message, Theme> for Example {
    type State = Values;
    type Event = Edit;

    fn update(&self, values: &mut Values, edit: Edit, _: &iced::Renderer) -> Option<Message> {
        values.update(edit);
        None
    }

    fn view(&self, values: &Values) -> impl Widget<Edit, Theme> + 'a {
        Element::from(match self {
            Self::Group(size) => values.group(*size),
            Self::Form(size) => values.form(*size),
        })
    }
}
