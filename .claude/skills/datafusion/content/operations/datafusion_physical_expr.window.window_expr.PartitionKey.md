# `datafusion_physical_expr::window::window_expr::PartitionKey`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.window.window_expr.PartitionKey.json).

<a id="op-aee8bf37c402eaf49326bb6e"></a>
## PartitionKey

`type_alias` · `datafusion_physical_expr::window::window_expr::PartitionKey` · datafusion-physical-expr 55.1.0

```rust
type PartitionKey = Vec<datafusion_common::ScalarValue>
```

Source: `src/window/window_expr.rs:623`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Key for IndexMap for each unique partition

For instance, if window frame is `OVER(PARTITION BY a,b)`,
PartitionKey would consist of unique `[a,b]` pairs
