## Installation

Clone the icedcn checkout beside your application, then add the local dependency from your application directory.

```sh
git clone --branch feat/style-api https://github.com/airstrike/icedcn.git ../icedcn
cargo add icedcn --path ../icedcn --features svg
```

Copy the checkout’s Cargo patch tables to your workspace root manifest. Add the same `iced` dependency the checkout uses.

The icon examples use Lucide. Add the font generator to your application:

```sh
cargo add --build --git https://github.com/airstrike/iced_lucide --rev 85fd2f6c533d0a88512a6a74d7db99b5de00b985 iced_lucide
```

Generate the icon module in `build.rs`.

```rust
fn main() {
    iced_lucide::build_all("icon").expect("generate Lucide icons");
}
```

Declare `mod icon;` in your crate root and register the font with `.fonts([icon::FONT])` on your iced application.
