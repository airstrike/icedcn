## Installation

From an application in this workspace, add the component library and enable the spinner used by the upload examples.

```sh
cargo add icedxdy --path ../.. --features spinner
```

This command assumes an application under `examples/`. From another project, point `--path` at your icedxdy checkout. Use the workspace’s iced dependency and patches when building an external application.
