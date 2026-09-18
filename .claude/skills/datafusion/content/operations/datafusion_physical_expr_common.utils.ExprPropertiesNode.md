# `datafusion_physical_expr_common::utils::ExprPropertiesNode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.utils.ExprPropertiesNode.json).

<a id="op-8cfd26574c5ee3a9b9f969e6"></a>
## ExprPropertiesNode

`type_alias` · `datafusion_physical_expr_common::utils::ExprPropertiesNode` · datafusion-physical-expr-common 55.1.0

```rust
type ExprPropertiesNode = tree_node::ExprContext<datafusion_expr_common::sort_properties::ExprProperties>
```

Source: `src/utils.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Represents a [`PhysicalExpr`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-fe8284c43330456b0d4e6af7) node with associated properties (order and
range) in a context where properties are tracked.
