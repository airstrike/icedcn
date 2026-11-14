| Variant | Rust style | Description |
|---|---|---|
| `default` | Default constructor | A strong primary bubble, usually for the current user. |
| `secondary` | `bubble::Secondary` | The standard neutral bubble for conversation content. |
| `muted` | `bubble::Muted` | A lower-emphasis bubble for quiet supporting content. |
| `tinted` | `bubble::Tinted` | A subtle primary-tinted bubble. |
| `outline` | `bubble::Outline` | A bordered bubble for secondary or rich content. |
| `ghost` | `bubble::Ghost` | Unframed content for assistant text or rich content. |
| `destructive` | `bubble::Destructive` | A destructive bubble for error or failed actions. |

A bubble sizes to its content, up to 80% of the container width. The `ghost` variant removes the max-width so assistant text and rich content can span the full row.
