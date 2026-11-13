```rust
drawer(
    move |d| d.trigger(button(label).style(button::Outline)),
    |d| {
        let body = column::Column::with_children((0..10).map(|_| text(LOREM).size(14).into()))
            .spacing(16)
            .padding([0, 16]);
        column![
            drawer::header("Move Goal", "Set your daily activity goal."),
            scrollable(Slot::from(body).compose()).height(Fill).boxed(),
            drawer::footer([
                d.close(button("Submit").width(Fill)),
                d.close(button("Cancel").style(button::Outline).width(Fill))
            ])
        ]
        .width(Fill)
        .into()
    },
)
.side(side)
.height_fraction(fraction)
.into()
```
