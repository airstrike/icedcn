## A Rust scratchpad

Playground holds one Rust program in an editable code block. Its `main` runs again on every edit, in an embedded [rawr](https://github.com/airstrike/rawr) engine, natively and on the web. Output shows what `main` printed, or the error rustc gives.

Open [the interactive example](https://icedcn.airstrike.fyi/examples/playground) to try it. It starts from a program that prints a table of tips for a few bills. Use the surrounding Examples navigation to return to the gallery.

## Run locally

From an icedcn checkout:

```sh
cargo run -p browser
```

Choose **Examples → Playground**. The implementation lives in `examples/browser/src/examples/playground.rs`. For setting up your own application, start with [Installation](icedcn://guide/installation); for adapting widget state and messages, read [Components](icedcn://guide/components).
