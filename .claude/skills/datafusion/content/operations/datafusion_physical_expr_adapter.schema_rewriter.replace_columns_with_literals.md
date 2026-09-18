# `datafusion_physical_expr_adapter::schema_rewriter::replace_columns_with_literals`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_adapter.schema_rewriter.replace_columns_with_literals.json).

<a id="op-c25b062649f7c614e6bac900"></a>
## replace_columns_with_literals

`function` · `datafusion_physical_expr_adapter::schema_rewriter::replace_columns_with_literals` · datafusion-physical-expr-adapter 55.1.0

```rust
fn replace_columns_with_literals<K, V>(expr: std::sync::Arc<dyn PhysicalExpr>, replacements: &std::collections::HashMap<K, V>) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>> where K: Borrow<str> + Eq + Hash, V: Borrow<datafusion_common::ScalarValue>
```

Source: `src/schema_rewriter.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-adapter/55.1.0/json).

Replace column references in the given physical expression with literal values.

Some use cases for this include:
- Partition column pruning: When scanning partitioned data, partition column references
  can be replaced with their literal values for the specific partition being scanned.
- Constant folding: In some cases, columns that can be proven to be constant
  from statistical analysis may be replaced with their literal values to optimize expression evaluation.
- Filling in non-null default values: in a custom [`PhysicalExprAdapter`](../operations/datafusion_physical_expr_adapter.schema_rewriter.PhysicalExprAdapter.md#op-3d7f05fc0e38d49de65f327a) implementation,
  column references can be replaced with default literal values instead of nulls.

# Arguments
- `expr`: The physical expression in which to replace column references.
- `replacements`: A mapping from column names to their corresponding literal `ScalarValue`s.
  Accepts various HashMap types including `HashMap<&str, &ScalarValue>`,
  `HashMap<String, ScalarValue>`, `HashMap<String, &ScalarValue>`, etc.

# Returns
- `Result<Arc<dyn PhysicalExpr>>`: The rewritten physical expression with columns replaced by literals.
