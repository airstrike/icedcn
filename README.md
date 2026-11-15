<img src="assets/logo.svg" width="80" alt="">

# icedcn

Widgets and themes for native Rust apps built with [iced](https://github.com/iced-rs/iced).

icedcn includes buttons, forms, menus, dialogs, tables, and layouts with shared
colors, spacing, and typography. Light and dark themes are built in. Widgets use
iced's usual `update` and `view` functions, and convert into `iced::Element`.

The library is under active development. APIs can change, and it currently uses
the `dev` branch of [airstrike/iced](https://github.com/airstrike/iced/tree/dev).

## Try it

With [Rust and rustup](https://rustup.rs/) installed:

```sh
git clone https://github.com/airstrike/icedcn.git
cd icedcn
cargo run --locked -p browser
```

Rustup uses the nightly pinned in [rust-toolchain.toml](rust-toolchain.toml).
The component browser has interactive previews and Rust snippets. For a full
screen of controls and theme settings, run `cargo run --locked -p palette`.

## Use it

Add these dependencies to your application's `Cargo.toml`:

```toml
[dependencies]
iced = "0.15.0-dev"
icedcn = { git = "https://github.com/airstrike/icedcn", branch = "master" }
```

Copy **both `[patch]` tables** from this repository's [Cargo.toml](Cargo.toml)
into your application's workspace root manifest, and use the same
[Rust toolchain](rust-toolchain.toml). These patches select the iced and
cosmic-text forks that icedcn uses; Cargo does not inherit patches from a
dependency. Keep your application's `Cargo.lock` in version control.

Here is a complete `src/main.rs`:

```rust
use iced::Element;
use icedcn::Theme;
use icedcn::widget::{button, center, column, text};

#[derive(Default)]
struct Counter {
    value: u32,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value += 1,
        }
    }

    fn view(&self) -> Element<'_, Message, Theme> {
        center(
            column![
                text::h1(self.value.to_string()),
                button("Add one").on_press(Message::Increment),
            ]
            .spacing(16)
            .align_x(iced::Center),
        )
        .into()
    }
}

fn main() -> iced::Result {
    iced::application(Counter::default, Counter::update, Counter::view)
        .title("Counter")
        .theme(|_: &Counter| Theme::light())
        .run()
}
```

Choose a variant with `.style(button::Outline)` or
`.style(button::Destructive)`. Use `Theme::dark()` for the dark theme.
Controls such as inputs and checkboxes can keep their own state; supply a value
and a change handler when your application needs to own it.

## Examples and development

The [component pages](examples/browser/src/component) pair previews with
[copyable examples](examples/browser/snippets). The
[palette app](examples/palette) shows widgets together in larger layouts.

```sh
cargo check --locked -p browser
cargo test --locked -p icedcn_widget
cargo test --locked -p icedcn_core
```

## License

MIT. Bundled assets retain their own license notices.
