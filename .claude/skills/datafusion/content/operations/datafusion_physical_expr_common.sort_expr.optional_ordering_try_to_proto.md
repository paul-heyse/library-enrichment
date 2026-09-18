# `datafusion_physical_expr_common::sort_expr::optional_ordering_try_to_proto`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.sort_expr.optional_ordering_try_to_proto.json).

<a id="op-087c4b202990c1141f0db53b"></a>
## optional_ordering_try_to_proto

`function` · `datafusion_physical_expr_common::sort_expr::optional_ordering_try_to_proto` · datafusion-physical-expr-common 55.1.0

```rust
fn optional_ordering_try_to_proto(ordering: Option<&LexOrdering>, ctx: &physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> datafusion_common::Result<Vec<datafusion_proto_models::protobuf::PhysicalSortExprNode>>
```

Source: `src/sort_expr.rs:285`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Serialize an optional [`LexOrdering`](../operations/datafusion_physical_expr_common.sort_expr.LexOrdering.md#op-d19b6df9e9a4be59c4b4dfd1), encoding `None` as an empty list.
