```rust
use icedcn::Horizontal;
use icedcn::Slot;
use icedcn::widget::{field, radio_group};

fn environment<'a, M: Clone + 'static>() -> Slot<'a, M> {
    let environments = [
        (
            "kubernetes",
            "Kubernetes",
            "Run GPU workloads on a K8s cluster.",
        ),
        (
            "vm",
            "Virtual Machine",
            "Access a cluster to run GPU workloads.",
        ),
    ];
    field::set(
        field::label("Compute Environment"),
        Some("Select the compute environment for your cluster."),
        radio_group(move |g| {
            field::group(environments.map(|(id, name, detail)| {
                field(g.item(id))
                    .width(iced::Fill)
                    .orientation(Horizontal)
                    .control_last()
                    .label(name)
                    .description(detail)
                    .card()
                    .into()
            }))
            .width(320)
            .spacing(8)
            .into()
        })
        .default_value("kubernetes"),
    )
    .max_width(320.)
    .into()
}
```
