## Add it to an application

For a sibling application beside the checkout, use a path dependency:

```toml
[dependencies]
iced = "0.15.0-dev"
icedcn = { path = "../icedcn" }
```

Copy the `[patch.crates-io]` and cosmic-text patch tables from the [workspace manifest](https://github.com/airstrike/icedcn/blob/HEAD/Cargo.toml) into the application's root manifest. Cargo patches are resolved at the workspace root; a dependency's patch tables do not configure your application. The library and your application must resolve to the same iced forks and revisions.

Start with the default feature set and add features for the widgets you use. The [Cargo guide](icedcn://guide/cargo#feature-flags) describes `svg`, `markdown`, `highlighter`, `spinner`, and `sweeten`. A git dependency can replace the path once you have chosen a repository revision to pin.
