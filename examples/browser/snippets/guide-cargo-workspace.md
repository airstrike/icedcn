## Workspace packages

The repository separates theme data from widget behavior:

| Package | Responsibility |
| --- | --- |
| `icedcn` | Public re-exports and root helpers. |
| `icedcn_core` | Theme, palettes, styles, and constants. |
| `icedcn_widget` | Controls, composition, and operations. |
| `browser` | This documentation application. |
| `palette` | Interactive palette example. |
| `highlight` | Syntax highlighting example. |

The checkout currently selects `nightly-2026-07-11` in `rust-toolchain.toml`. Run commands from the repository root so Rust's toolchain selection and Cargo's workspace configuration apply consistently.
