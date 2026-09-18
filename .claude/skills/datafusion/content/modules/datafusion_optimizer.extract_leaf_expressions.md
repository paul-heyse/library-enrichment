# `datafusion_optimizer::extract_leaf_expressions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.extract_leaf_expressions.json).

<a id="op-8e21e6bfa2a18355815d8982"></a>
## extract_leaf_expressions

`module` · `datafusion_optimizer::extract_leaf_expressions` · datafusion-optimizer 55.1.0

```rust
mod extract_leaf_expressions
```

Source: `src/extract_leaf_expressions.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Two-pass optimizer pipeline that pushes cheap expressions (like struct field
access `user['status']`) closer to data sources, enabling early data reduction
and source-level optimizations (e.g., Parquet column pruning). See
[`ExtractLeafExpressions`](../operations/datafusion_optimizer.extract_leaf_expressions.ExtractLeafExpressions.md#op-0a731b241a2648730b745694) (pass 1) and [`PushDownLeafProjections`](../operations/datafusion_optimizer.extract_leaf_expressions.PushDownLeafProjections.md#op-4b118316543e73e6c579a135) (pass 2).
