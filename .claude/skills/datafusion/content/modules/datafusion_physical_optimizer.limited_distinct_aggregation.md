# `datafusion_physical_optimizer::limited_distinct_aggregation`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.limited_distinct_aggregation.json).

<a id="op-d70e50024de4a2825e5012fb"></a>
## limited_distinct_aggregation

`module` · `datafusion_physical_optimizer::limited_distinct_aggregation` · datafusion-physical-optimizer 55.1.0

```rust
mod limited_distinct_aggregation
```

Source: `src/limited_distinct_aggregation.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

A special-case optimizer rule that pushes limit into a grouped aggregation
which has no aggregate expressions or sorting requirements
