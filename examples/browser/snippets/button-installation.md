## Installation

Clone the icedcn checkout beside your application, then add the local dependency from your application directory.

```sh
git clone --branch feat/style-api https://github.com/airstrike/icedcn.git ../icedcn
cargo add icedcn --path ../icedcn --features svg,spinner
```

Copy the checkout’s `[patch.crates-io]` table to your workspace root manifest so Cargo uses the same iced and cosmic-text revisions. Use the checkout’s iced dependency with its `svg` feature for the With Icon example.

The arrow and menu examples use Lucide icons. Add the font generator to your application:

```sh
cargo add --build --git https://github.com/airstrike/iced_lucide --rev 85fd2f6c533d0a88512a6a74d7db99b5de00b985 iced_lucide
```

In `build.rs`, generate the icon module:

```rust
fn main() {
    iced_lucide::build_all("icon").expect("generate Lucide icons");
}
```

Declare `mod icon;` in your crate root and register the font with `.font(icon::FONT)` on your iced application. For the With Icon example, save the [Git Branch SVG](https://raw.githubusercontent.com/tabler/tabler-icons/v3.31.0/icons/outline/git-branch.svg) beside the Rust file containing the example.
