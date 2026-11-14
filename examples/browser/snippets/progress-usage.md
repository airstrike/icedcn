## Usage

```rust
use iced::Fill;
use icedxdy::widget::progress;
```

```rust
progress(self.value).width(Fill)
```

Values are clamped to `0.0..=100.0`. The bar self-animates: every value change eases over 150ms via `cubic-bezier(0.4, 0, 0.2, 1)`. Use `.on_idle(...)` to publish a message with the final value when the animation settles.
