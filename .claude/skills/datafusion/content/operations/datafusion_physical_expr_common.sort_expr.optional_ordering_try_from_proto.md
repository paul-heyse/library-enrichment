# `datafusion_physical_expr_common::sort_expr::optional_ordering_try_from_proto`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.sort_expr.optional_ordering_try_from_proto.json).

<a id="op-c4adf6fc6305131337120f0d"></a>
## optional_ordering_try_from_proto

`function` · `datafusion_physical_expr_common::sort_expr::optional_ordering_try_from_proto` · datafusion-physical-expr-common 55.1.0

```rust
fn optional_ordering_try_from_proto(nodes: &[datafusion_proto_models::protobuf::PhysicalSortExprNode], ctx: &physical_expr::proto_decode::PhysicalExprDecodeCtx<'_>) -> datafusion_common::Result<Option<LexOrdering>>
```

Source: `src/sort_expr.rs:295`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Counterpart of [`optional_ordering_try_to_proto`](../operations/datafusion_physical_expr_common.sort_expr.optional_ordering_try_to_proto.md#op-087c4b202990c1141f0db53b): an empty list decodes
as `None`.
