```rust
field(select([
    ("engineering", "Engineering"), ("design", "Design"), ("marketing", "Marketing"),
    ("sales", "Sales"), ("support", "Customer Support"), ("hr", "Human Resources"),
    ("finance", "Finance"), ("operations", "Operations"),
].map(|(value, label)| select::item(value, label))).placeholder("Choose department"))
.label("Department").description("Select your department or area of work.").width(320)
```
