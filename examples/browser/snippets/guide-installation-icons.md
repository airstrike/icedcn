## Icons

Examples that import `crate::icon` use a generated Lucide font module. Add the generator to your application's `Cargo.toml`:

```toml
[build-dependencies]
iced_lucide = { git = "https://github.com/airstrike/iced_lucide", branch = "codex/configurable-source" }
```

Create `build.rs` beside that manifest:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    iced_lucide::build_all("icon")?;
    Ok(())
}
```

The build writes `src/icon.rs` and `fonts/lucide.ttf`. Declare the generated module at your crate root with `mod icon;`. Then add `.fonts([icon::FONT])` to your `iced::application(...)` builder before `.run()`. Registering the font is required for the glyphs to display correctly.

Call the generated functions as widget content: `button(icon::plus())`. In another module, import `crate::icon` first. The generated module uses iced text widgets; icedcn's composition layer accepts them as children.

`build_all` includes every icon, so examples can share the same setup. For an application that needs a smaller font, `iced_lucide::build("fonts/icons.toml")` generates only the icons listed in that configuration. The browser's `fonts/icons.toml` and `build.rs` provide a working subset configuration.
