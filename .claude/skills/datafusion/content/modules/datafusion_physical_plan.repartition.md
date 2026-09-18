# `datafusion_physical_plan::repartition`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.repartition.json).

<a id="op-046ecfd37d2c1782e3b5de1b"></a>
## repartition

`module` · `datafusion_physical_plan::repartition` · datafusion-physical-plan 55.1.0

```rust
mod repartition
```

Source: `src/repartition/mod.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

This file implements the [`RepartitionExec`](../operations/datafusion_physical_plan.repartition.RepartitionExec.md#op-a0cd55fc5d1d094b433c884b) operator, which maps N input
partitions to M output partitions based on a partitioning scheme, optionally
maintaining the order of the input rows in the output.
