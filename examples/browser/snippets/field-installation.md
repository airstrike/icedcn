## Installation

Clone icedcn beside your application and run these commands from the application directory.

```sh
git clone --branch feat/style-api https://github.com/airstrike/icedcn.git ../icedcn
cargo add icedcn --path ../icedcn
```

Copy the checkout’s iced dependency and both dependency patch tables (including `[patch.crates-io]`) into your workspace root manifest. Cargo reads dependency patches at the workspace root.
