## Installation

Clone icedcn beside your application and run these commands from the application directory.

```sh
git clone --branch feat/style-api https://github.com/airstrike/icedcn.git ../icedcn
cargo add icedcn --path ../icedcn
```

Copy the checkout’s iced dependency and both dependency patch tables (including `[patch.crates-io]`) into your workspace root manifest. Cargo reads dependency patches at the workspace root.

The Alert examples use Lucide’s circle-check, info and circle-alert icons. Add its build dependency and generate the public icon helpers.

```sh
cargo add --build --git https://github.com/airstrike/iced_lucide --rev 85fd2f6c533d0a88512a6a74d7db99b5de00b985 iced_lucide
```

```rust
fn main() {
    iced_lucide::build_all("icon").expect("generate Lucide icons");
}
```

Place this function in `build.rs` and declare `mod icon;` in `src/main.rs`. To use the same body typeface as these examples, copy the bundled Geist font and its license into your application.

```sh
mkdir -p fonts
cp ../icedcn/examples/browser/fonts/Geist.ttf ../icedcn/examples/browser/fonts/Geist-OFL.txt fonts/
```

On your iced application, register both fonts and select Geist for body text.

```rust
iced::application(App::new, App::update, App::view)
    .fonts([icon::FONT, include_bytes!("../fonts/Geist.ttf").as_slice()])
    .font(iced::Font::new("Geist"))
```
