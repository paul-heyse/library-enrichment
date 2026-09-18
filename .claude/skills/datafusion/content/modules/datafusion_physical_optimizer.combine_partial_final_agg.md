# `datafusion_physical_optimizer::combine_partial_final_agg`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.combine_partial_final_agg.json).

<a id="op-5bfdf300bed0c928c31c41d0"></a>
## combine_partial_final_agg

`module` · `datafusion_physical_optimizer::combine_partial_final_agg` · datafusion-physical-optimizer 55.1.0

```rust
mod combine_partial_final_agg
```

Source: `src/combine_partial_final_agg.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

CombinePartialFinalAggregate optimizer rule checks the adjacent Partial and Final AggregateExecs
and try to combine them if necessary
