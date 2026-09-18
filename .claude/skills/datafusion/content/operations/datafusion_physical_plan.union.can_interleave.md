# `datafusion_physical_plan::union::can_interleave`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.union.can_interleave.json).

<a id="op-4d0987193c0b2588d14a76dc"></a>
## can_interleave

`function` · `datafusion_physical_plan::union::can_interleave` · datafusion-physical-plan 55.1.0

```rust
fn can_interleave<T: Borrow<std::sync::Arc<dyn ExecutionPlan>>>(inputs: impl Iterator<Item = T>) -> bool
```

Source: `src/union.rs:899`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns true if all inputs have the same [`Partitioning::Hash`](../operations/datafusion_physical_expr.partitioning.Partitioning.md#op-2e61f3626627abaac1462fec) or [`Partitioning::Range`](../operations/datafusion_physical_expr.partitioning.Partitioning.md#op-416521ebc0d82658601f1876)
spec, making them safe to interleave. Two inputs are interleave-compatible when partition
`k` covers the identical key range or hash bucket across every input.

Note: compatibility is checked sequentially against the first input, so
`InputDistributionRequirements::co_partitioned` is not needed here.

It might be too strict here in the case that the input partition specs are compatible but not exactly the same.
For example one input partition has the partition spec Hash('a','b','c') and
other has the partition spec Hash('a'), It is safe to derive the out partition with the spec Hash('a','b','c').
