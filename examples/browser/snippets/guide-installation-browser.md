## Run the browser

To explore the component examples, run the browser from your icedcn checkout:

```sh
cd ../icedcn
cargo run --locked -p browser
```

This command assumes you are in the `my-app` directory created above. The browser has its own dependencies and features; you do not need to run it to use icedcn in your application.

Keep the checkout's `rust-toolchain.toml` and `Cargo.lock` when reproducing a build. The examples and default font are bundled; selecting an additional font in the font chooser can require a network connection.
