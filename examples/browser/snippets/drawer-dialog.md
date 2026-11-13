```rust
let trigger = |d: &dialog::Handle| d.trigger(button("Edit Profile").style(button::Outline));
let form = |d: &dialog::Handle| {
    column![
        field(input("").default_value("shadcn@example.com")).label("Email"),
        field(input("").default_value("@shadcn")).label("Username"),
        d.close(button("Save changes").width(Fill))
    ]
    .spacing(24)
};
let description = "Make changes to your profile here. Click save when you're done.";
if desktop {
    dialog(trigger, move |d| {
        column![dialog::header("Edit profile", description), form(d)]
            .spacing(16)
            .into()
    })
    .max_width(425.)
    .into()
} else {
    drawer(trigger, move |d| {
        column![
            drawer::header("Edit profile", description),
            container(form(d)).padding([0, 16]),
            drawer::footer([d.close(button("Cancel").width(Fill).style(button::Outline))])
        ]
        .into()
    })
    .into()
}
```
