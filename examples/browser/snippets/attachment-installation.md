## Installation

From an application in this workspace, add the component library and enable the spinner used by the upload examples.

```sh
cargo add icedcn --path ../.. --features spinner
```

This command assumes an application under `examples/`. From another project, point `--path` at your icedcn checkout. Use the workspace’s iced dependency and patches when building an external application.
