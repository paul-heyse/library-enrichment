# `datafusion_physical_expr::window::window_expr::PartitionBatches`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.window.window_expr.PartitionBatches.json).

<a id="op-81d8459c793f5c56479bc66c"></a>
## PartitionBatches

`type_alias` · `datafusion_physical_expr::window::window_expr::PartitionBatches` · datafusion-physical-expr 55.1.0

```rust
type PartitionBatches = indexmap::IndexMap<PartitionKey, datafusion_expr::window_state::PartitionBatchState, datafusion_common::hash_utils::RandomState>
```

Source: `src/window/window_expr.rs:703`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

The IndexMap (i.e. an ordered HashMap) where record batches are separated for each partition.
