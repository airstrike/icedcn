## Usage

```rust
use icedcn::widget::text;
```

Each typography level has its own constructor:

```rust
text::h1("Page Title")        // 30px, semibold
text::h2("Section Title")     // 20px, medium
text::h3("Subsection")        // 18px, medium
text::h4("Small Heading")     // 16px, medium

text::p("Body paragraph")     // 16px, normal
text::lead("Intro paragraph") // 20px, muted
text::large("Large text")     // 18px, semibold
text::small("Small text")     // 14px, normal
text::muted("Secondary text") // 14px, muted color

text::blockquote("Quote text")
text::inline_code("widget::alert_dialog")
```

Chain `.medium()`, `.semibold()`, `.bold()`, or `.italic()` to override the default weight/style. For an underlined heading, use `column![text::h2(...), rule::horizontal(1)].spacing(8)`.
