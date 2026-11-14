```rust
use iced::widget::component::{Component, component};
use iced::{Element, Fill, Widget};
use icedcn::{Slot, Theme};
use icedcn::widget::{button, field, input, row, select};

use crate::Message;

pub(super) fn preview<'a>(extended: bool) -> Element<'a, Message, Theme> {
    component(Form { extended }).boxed()
}

struct Form { extended: bool }

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
enum Edit { Name(String), Email(String), Phone(String), Country(String), Address(String), Reset, Submit }

impl<'a> Component<'a, Message, Theme> for Form {
    type State = Values;
    type Event = Edit;

    fn update(&self, values: &mut Values, edit: Edit, _: &iced::Renderer) -> Option<Message> {
        match edit {
            Edit::Name(name) => values.name = name,
            Edit::Email(email) => values.email = email,
            Edit::Phone(phone) => values.phone = phone,
            Edit::Country(country) => values.country = Some(country),
            Edit::Address(address) => values.address = address,
            Edit::Reset => *values = Values::default(),
            Edit::Submit => values.submitted = true,
        }
        None
    }

    fn view(&self, values: &Values) -> impl Widget<Edit, Theme> + 'a {
        let (name_hint, email_hint, email_help, reset) = if self.extended {
            ("Evil Rabbit", "john@example.com", "We'll never share your email with anyone.", "Cancel")
        } else {
            ("Jordan Lee", "name@example.com", "We'll send updates to this address.", "Reset")
        };
        let mut name = field(input(name_hint).value(values.name.clone()).on_input(Edit::Name)).label("Name");
        if self.extended && values.submitted && values.name.trim().is_empty() {
            name = name.error("Please fill out this field.");
        }
        let mut fields: Vec<Slot<'a, Edit>> = vec![
            name.into(), field(input(email_hint).value(values.email.clone()).on_input(Edit::Email))
                .label("Email").description(email_help).into()
        ];
        if self.extended {
            let country = select([("us","United States"),("uk","United Kingdom"),("ca","Canada")]
                .map(|(value,label)| select::item(value,label)))
                .value(values.country.clone().unwrap_or_else(|| "us".into())).on_value_change(Edit::Country).width(Fill);
            fields.extend([
                row![field(input("+1 (555) 123-4567").value(values.phone.clone()).on_input(Edit::Phone)).label("Phone").width(Fill),
                    field(country).label("Country").width(Fill)].spacing(16).into(),
                field(input("123 Main St").value(values.address.clone()).on_input(Edit::Address)).label("Address").into(),
            ]);
        }
        fields.push(row![button(reset).style(button::Outline).on_press(Edit::Reset), button("Submit").on_press(Edit::Submit)].spacing(8).into());
        Element::from(field::group(fields).width(384))
    }
}
```
