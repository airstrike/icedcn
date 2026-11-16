## Rust as you type

Playground runs a whole Rust program inside the component browser. Edit the program in the textarea and its `main` runs again on every change, in an embedded [rawr](https://github.com/airstrike/rawr) engine, both natively and on the web. The Output card shows what `main` printed, or the error rustc would give in a destructive alert.

Open [the interactive example](https://icedcn.airstrike.fyi/examples/playground) to try it. It starts from a program that prints a table of tips for a few bills. Use the surrounding Examples navigation to return to the gallery.

## Run locally

From an icedcn checkout:

```sh
cargo run -p browser
```

Choose **Examples → Playground**. The implementation lives in `examples/browser/src/examples/playground.rs`. For setting up your own application, start with [Installation](icedcn://guide/installation); for adapting widget state and messages, read [Components](icedcn://guide/components).
