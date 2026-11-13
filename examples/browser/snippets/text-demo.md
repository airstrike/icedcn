```rust
use iced::{Element, Fill, Widget};
use iced::widget::column;
use icedxdy::Theme;
use icedxdy::widget::text;
use sweeten::widget::table;

fn table_card<'a, Message: 'a>() -> iced::Element<'a, Message, Theme> {
    let rows = &[
        ("Empty", "Overflowing"),
        ("Modest", "Satisfied"),
        ("Full", "Ecstatic"),
    ];

    let header = |label| Element::from(text::p(label).bold());

    let columns = [
        table::column(Some(header("King's Treasury")), |row: &(_, _)| {
            Element::from(text::p(row.0))
        })
        .width(Fill),
        table::column(Some(header("People's happiness")), |row: &(_, _)| {
            Element::from(text::p(row.1))
        })
        .width(Fill),
    ];

    table::table(columns, rows)
        .width(Fill)
        .padding_x(16)
        .padding_y(8)
        .separator(1)
        .border(1)
        .boxed()
}

column![
    Element::from(text::h1("Taxing Laughter: The Joke Tax Chronicles")),
    Element::from(text::lead(
        "Once upon a time, in a far-off land, there was a very lazy king \
         who spent all day lounging on his throne. One day, his advisors \
         came to him with a problem: the kingdom was running out of money."
    )),
    Element::from(text::h2("The King's Plan")),
    Element::from(text::p(
        "The king thought long and hard, and finally came up with a \
         brilliant plan: he would tax the jokes in the kingdom."
    )),
    Element::from(text::blockquote(
        "\"After all,\" he said, \"everyone enjoys a good joke, so it's \
         only fair that they should pay for the privilege.\""
    )),
    Element::from(text::h3("The Joke Tax")),
    Element::from(text::p(
        "The king's subjects were not amused. They grumbled and \
         complained, but the king was firm:"
    )),
    Element::from(text::list([
        "1st level of puns: 5 gold coins",
        "2nd level of jokes: 10 gold coins",
        "3rd level of one-liners : 20 gold coins",
    ])),
    Element::from(text::p(
        "As a result, people stopped telling jokes, and the kingdom \
         fell into a gloom. But there was one person who refused to let \
         the king's foolishness get him down: a court jester named \
         Jokester."
    )),
    Element::from(text::h3("Jokester's Revolt")),
    Element::from(text::p(
        "Jokester began sneaking into the castle in the middle of the \
         night and leaving jokes all over the place: under the king's \
         pillow, in his soup, even in the royal toilet. The king was \
         furious, but he couldn't seem to stop Jokester."
    )),
    Element::from(text::p(
        "And then, one day, the people of the kingdom discovered that \
         the jokes left by Jokester were so funny that they couldn't \
         help but laugh. And once they started laughing, they couldn't \
         stop."
    )),
    Element::from(text::h3("The People's Rebellion")),
    Element::from(text::p(
        "The people of the kingdom, feeling uplifted by the laughter, \
         started to tell jokes and puns again, and soon the entire \
         kingdom was in on the joke."
    )),
    table_card(),
    Element::from(text::p(
        "The king, seeing how much happier his subjects were, realized \
         the error of his ways and repealed the joke tax. Jokester was \
         declared a hero, and the kingdom lived happily ever after."
    )),
    Element::from(text::p(
        "The moral of the story is: never underestimate the power of a \
         good laugh and always be careful of bad ideas."
    )),
]
.spacing(16)
.width(Fill)
.boxed()
```
