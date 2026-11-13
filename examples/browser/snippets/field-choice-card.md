```rust
field::set(text::label("Compute Environment"), Some("Select the compute environment for your cluster."),
    radio_group(|g| field::group([
        field(g.item("kubernetes")).orientation(Horizontal).control_last().label("Kubernetes").description("Run GPU workloads on a K8s cluster.").card().into(),
        field(g.item("vm")).orientation(Horizontal).control_last().label("Virtual Machine").description("Access a cluster to run GPU workloads.").card().into(),
    ]).spacing(8).into()).default_value("kubernetes")).max_width(320.)
```
