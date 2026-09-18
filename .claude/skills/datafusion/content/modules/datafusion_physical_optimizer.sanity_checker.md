# `datafusion_physical_optimizer::sanity_checker`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.sanity_checker.json).

<a id="op-cba550a59b3cabf10ebee296"></a>
## sanity_checker

`module` · `datafusion_physical_optimizer::sanity_checker` · datafusion-physical-optimizer 55.1.0

```rust
mod sanity_checker
```

Source: `src/sanity_checker.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

The [SanityCheckPlan](../operations/datafusion_physical_optimizer.sanity_checker.SanityCheckPlan.md#op-02a18f59b304936d153b65a1) rule ensures that a given plan can
accommodate its infinite sources, if there are any. It will reject
non-runnable query plans that use pipeline-breaking operators on
infinite input(s). In addition, it will check if all order and
distribution requirements of a plan are satisfied by its children.
