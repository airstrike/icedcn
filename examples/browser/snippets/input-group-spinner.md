```rust
stack([
    input_group(input("Searching...")).addon(end(spinner())).into(),
    input_group(input("Processing...")).addon(input_group::addon(spinner())).into(),
    input_group(input("Saving changes...")).addon(end(row![input_group::text("Saving..."),spinner()].spacing(8))).into(),
    input_group(input("Refreshing data...")).addon(input_group::addon(spinner())).addon(end(input_group::text("Please wait..."))).into(),
])
```
