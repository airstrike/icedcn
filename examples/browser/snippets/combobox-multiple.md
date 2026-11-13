```rust
use icedxdy::widget::combobox;

const FRAMEWORKS: [&str; 5] = ["Next.js", "SvelteKit", "Nuxt.js", "Remix", "Astro"];

combobox(FRAMEWORKS)
    .multiple(true)
    .auto_highlight(true)
    .default_values(["Next.js"])
    .width(320)
```
