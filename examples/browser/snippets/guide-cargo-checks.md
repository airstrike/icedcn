## Build and verify

Use the browser as the main integration check, then run tests for the code you changed:

```sh
cargo check --locked -p browser
cargo test --locked -p icedcn_widget
cargo test --locked -p icedcn_core
cargo clippy --locked -p browser
cargo fmt --all --check
```

For a component's browser integration tests, select its test target, for example `cargo test --locked -p browser --test command`.

The workspace patches iced and cosmic-text to development forks. Keep `Cargo.lock` checked in and inspect dependency changes before updating those forks. If an external application reports incompatible widget or renderer types, compare its resolved iced dependencies with this workspace before adding conversions. `cargo tree -d` helps locate duplicate package versions.
