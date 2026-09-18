# `datafusion_physical_expr_common::sort_expr::sort_exprs_try_from_proto`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.sort_expr.sort_exprs_try_from_proto.json).

<a id="op-df409e7b0d971d9e5453ed47"></a>
## sort_exprs_try_from_proto

`function` · `datafusion_physical_expr_common::sort_expr::sort_exprs_try_from_proto` · datafusion-physical-expr-common 55.1.0

```rust
fn sort_exprs_try_from_proto(nodes: &[datafusion_proto_models::protobuf::PhysicalSortExprNode], ctx: &physical_expr::proto_decode::PhysicalExprDecodeCtx<'_>) -> datafusion_common::Result<Vec<PhysicalSortExpr>>
```

Source: `src/sort_expr.rs:273`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Reconstruct a sequence of sort expressions from the flat
[`PhysicalSortExprNode`] list, the counterpart of
[`sort_exprs_try_to_proto`](../operations/datafusion_physical_expr_common.sort_expr.sort_exprs_try_to_proto.md#op-62860e2a89cf8493abbfd4b3).

Returns the expressions rather than a [`LexOrdering`](../operations/datafusion_physical_expr_common.sort_expr.LexOrdering.md#op-d19b6df9e9a4be59c4b4dfd1) or a
[`LexRequirement`](../operations/datafusion_physical_expr_common.sort_expr.LexRequirement.md#op-dbccd31ff3eaea7309fbb53a), because callers differ in what an empty list means:
`LexOrdering::new` / `LexRequirement::new` return `None` for it, which is
"no ordering declared" for a scan and an error for an operator that requires
one. Callers with the former convention can use
[`optional_ordering_try_from_proto`](../operations/datafusion_physical_expr_common.sort_expr.optional_ordering_try_from_proto.md#op-c4adf6fc6305131337120f0d) instead.

[`PhysicalSortExprNode`]: datafusion_proto_models::protobuf::PhysicalSortExprNode
