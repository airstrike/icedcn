## Installation

Add `icedcn` from the checkout beside your application.

```sh
cargo add icedcn --path ../icedcn
```

Use the checkout’s iced dependency and copy its Cargo patch tables to your workspace manifest. The Actions example uses the Lucide font. Add its generator with `cargo add --build --git https://github.com/airstrike/iced_lucide --rev 85fd2f6c533d0a88512a6a74d7db99b5de00b985 iced_lucide`, call `iced_lucide::build_all("icon")` from your build script, declare `mod icon;` in your crate root, and register `icon::FONT` with the application’s `.font(...)` builder.
