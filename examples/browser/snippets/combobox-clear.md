```rust
use icedxdy::widget::combobox;

const FRAMEWORKS: [&str; 5] = ["Next.js", "SvelteKit", "Nuxt.js", "Remix", "Astro"];

combobox(FRAMEWORKS)
    .placeholder("Select a framework")
    .icon(icon::chevron_down)
    .clearable(true)
    .default_value("Next.js")
```
