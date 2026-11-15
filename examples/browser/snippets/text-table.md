```rust
use iced::{Fill, Length, Pixels, Widget};
use icedcn::theme::container::Style;
use icedcn::widget::{column, container, row, rule, text};

let data = [
    ["King's Treasury", "People's happiness"], ["Empty", "Overflowing"],
    ["Modest", "Satisfied"], ["Full", "Ecstatic"],
];
let rows = data.into_iter().enumerate().map(|(i, [left, right])| {
    let cell = |value, share| {
        let label = text::p(value).line_height(Pixels(24.));
        container(if i == 0 { label.bold() } else { label })
            .padding([8, 16]).width(Length::FillPortion(share))
    };
    container(row![cell(left, 5), rule::vertical(1).boxed(), cell(right, 6)].width(Fill))
        .width(Fill).style(move |t| Style {
            background: Some((if i == 2 { t.muted.base } else { t.neutral.base }).into()),
            ..Default::default()
        }).into()
});
container(column(rows).spacing(1).width(Fill)).padding(1).width(Fill)
    .style(|t| Style { background: Some(t.neutral.border.into()), ..Default::default() })
```
