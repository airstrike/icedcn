```rust
field::group([
    field(input("Max Leiter")).label("Username").description("Choose a unique username for your account.").into(),
    field(input("••••••••").secure(true)).label("Password").description("Must be at least 8 characters long.").description_above().into(),
]).width(320)
```
