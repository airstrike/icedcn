```rust
const LABELS: [&str; 6] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun"];
const DESKTOP: [f32; 6] = [186.0, 305.0, 237.0, 73.0, 209.0, 214.0];
const MOBILE: [f32; 6] = [80.0, 200.0, 120.0, 190.0, 130.0, 140.0];
const BLUE: Color = Color::from_rgb8(37, 99, 235);
const LIGHT_BLUE: Color = Color::from_rgb8(96, 165, 250);
fn monthly() -> Chart<'static> {
    chart(
        &LABELS,
        [
            Series::new("Desktop", BLUE, &DESKTOP),
            Series::new("Mobile", LIGHT_BLUE, &MOBILE),
        ],
    )
}

monthly().grid(true).axis(true)
```
