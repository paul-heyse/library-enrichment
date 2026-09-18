# `datafusion_physical_plan::aggregates::concat_slices`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.aggregates.concat_slices.json).

<a id="op-13c7412e959895bcaa584b88"></a>
## concat_slices

`function` · `datafusion_physical_plan::aggregates::concat_slices` · datafusion-physical-plan 55.1.0

```rust
fn concat_slices<T: Clone>(lhs: &[T], rhs: &[T]) -> Vec<T>
```

Source: `src/aggregates/mod.rs:2748`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Concatenates the given slices.
