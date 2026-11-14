```rust
use iced::Fill;
use icedcn::Slot;
use icedcn::widget::{button, field, input, row, select};

#[derive(Default)]
struct Contact {
    name: String, email: String, phone: String,
    country: Option<String>, address: String, submitted: bool,
}

#[derive(Clone, Debug)]
enum Edit { Name(String), Email(String), Phone(String), Country(String), Address(String), Reset, Submit }

impl Contact {
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

    fn view(&self) -> Slot<'static, Edit> {
        let entry = |label: &'static str, hint: &'static str, value: &str, edit: fn(String) -> Edit| {
            field(input(hint).width(Fill).value(value.to_owned()).on_input(edit).on_submit(Edit::Submit))
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
        let country = select([("us", "United States"), ("uk", "United Kingdom"), ("ca", "Canada")]
            .map(|(value, label)| select::item(value, label)))
            .value(self.country.as_deref().unwrap_or("us").to_owned())
            .on_value_change(Edit::Country).width(Fill);
        field::group([
            name.into(), email.into(),
            row![
                entry("Phone", "+1 (555) 123-4567", &self.phone, Edit::Phone).width(Fill),
                field(country).label("Country").width(Fill)
            ].spacing(16).width(Fill).into(),
            entry("Address", "123 Main St", &self.address, Edit::Address).into(),
            row![button("Cancel").style(button::Outline).on_press(Edit::Reset), button("Submit").on_press(Edit::Submit)]
                .spacing(8).into(),
        ]).width(384).into()
    }
}

fn valid_email(email: &str) -> bool {
    email.split_once('@').is_some_and(|(local, domain)| {
        !local.is_empty() && !domain.is_empty() && !domain.contains('@') && !email.contains(char::is_whitespace)
    })
}
```
