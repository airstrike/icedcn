use super::{h1, h2_with_border, h3, paragraph, table_card};
use crate::{Message, page};
use iced::font::{Tag, Variation};
use iced::widget::{rich_text, span};
use iced::{Element, Fill, Pixels, Widget};
use icedcn::Theme;
use icedcn::widget::{column, container, text};

pub(super) fn view<'a>(index: usize) -> Element<'a, Message, Theme> {
    let blocks = [
        (0, h1("Taxing Laughter: The Joke Tax Chronicles").into()),
        (24, text::lead("Once upon a time, in a far-off land, there was a very lazy king who spent all \
                 day lounging on his throne. One day, his advisors came to him with a problem: \
                 the kingdom was running out of money.").line_height(Pixels(28.)).into()),
        (40, h2_with_border("The King's Plan").into()),
        (24, plan_link(index)),
        (24, text::blockquote("\"After all,\" he said, \"everyone enjoys a good joke, so it's only fair that \
                 they should pay for the privilege.\"").into()),
        (32, h3("The Joke Tax").into()),
        (24, paragraph("The king's subjects were not amused. They grumbled and complained, but the king was firm:").into()),
        (24, text::list([
                    "1st level of puns: 5 gold coins",
                    "2nd level of jokes: 10 gold coins",
                    "3rd level of one-liners : 20 gold coins",
                ]).into()),
        (24, paragraph("As a result, people stopped telling jokes, and the kingdom fell into a gloom. \
                 But there was one person who refused to let the king's foolishness get him \
                 down: a court jester named Jokester.").into()),
        (32, h3("Jokester's Revolt").into()),
        (24, paragraph("Jokester began sneaking into the castle in the middle of the night and \
                 leaving jokes all over the place: under the king's pillow, in his soup, even \
                 in the royal toilet. The king was furious, but he couldn't seem to stop \
                 Jokester.").into()),
        (24, paragraph("And then, one day, the people of the kingdom discovered that the jokes left \
                 by Jokester were so funny that they couldn't help but laugh. And once they \
                 started laughing, they couldn't stop.").into()),
        (32, h3("The People's Rebellion").into()),
        (24, paragraph("The people of the kingdom, feeling uplifted by the laughter, started to tell \
                 jokes and puns again, and soon the entire kingdom was in on the joke.").into()),
        (24, table_card().into()),
        (24, paragraph("The king, seeing how much happier his subjects were, realized the error of \
                 his ways and repealed the joke tax. Jokester was declared a hero, and the \
                 kingdom lived happily ever after.").into()),
        (24, paragraph("The moral of the story is: never underestimate the power of a good laugh and \
                 always be careful of bad ideas.").into()),
    ];
    column(
        blocks
            .into_iter()
            .map(|(top, body): (u16, Element<'a, Message, Theme>)| {
                container(body).width(Fill).padding([top, 0, 0, 0]).into()
            }),
    )
    .width(Fill)
    .into()
}

fn plan_link<'a>(index: usize) -> Element<'a, Message, Theme> {
    rich_text([
        span("The king thought long and hard, and finally came up with "),
        span("a brilliant plan")
            .font_variation(Variation::new(Tag::new(b"wght"), 500.))
            .link(())
            .underline(true),
        span(": he would tax the jokes in the kingdom."),
    ])
    .size(16)
    .line_height(Pixels(28.))
    .width(Fill)
    .selectable(true)
    .on_link_click(move |()| Message::Page(index, page::Message::ScrollTo(0)))
    .boxed()
}
