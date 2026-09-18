# `datafusion_physical_expr_common::sort_expr::sort_exprs_try_to_proto`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.sort_expr.sort_exprs_try_to_proto.json).

<a id="op-62860e2a89cf8493abbfd4b3"></a>
## sort_exprs_try_to_proto

`function` · `datafusion_physical_expr_common::sort_expr::sort_exprs_try_to_proto` · datafusion-physical-expr-common 55.1.0

```rust
fn sort_exprs_try_to_proto<E: std::borrow::Borrow<PhysicalSortExpr>>(exprs: impl IntoIterator<Item = E>, ctx: &physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>) -> datafusion_common::Result<Vec<datafusion_proto_models::protobuf::PhysicalSortExprNode>>
```

Source: `src/sort_expr.rs:250`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Serialize a sequence of sort expressions into the flat
[`PhysicalSortExprNode`] list the wire format uses for an ordering.

Accepts anything that yields [`PhysicalSortExpr`](../operations/datafusion_physical_expr_common.sort_expr.PhysicalSortExpr.md#op-80fd045cf8466b7fe98e5ea8)s by value or by reference,
so a [`LexOrdering`](../operations/datafusion_physical_expr_common.sort_expr.LexOrdering.md#op-d19b6df9e9a4be59c4b4dfd1), a `&[PhysicalSortExpr]`, or a [`LexRequirement`](../operations/datafusion_physical_expr_common.sort_expr.LexRequirement.md#op-dbccd31ff3eaea7309fbb53a)
mapped through [`PhysicalSortExpr::from`](../operations/datafusion_physical_expr_common.sort_expr.PhysicalSortExpr.md#op-eef7fc040f096affd53ec53c) all work:

```ignore
let nodes = sort_exprs_try_to_proto(ordering.iter(), ctx)?;
let nodes = sort_exprs_try_to_proto(
    requirement.iter().map(|req| PhysicalSortExpr::from(req.clone())),
    ctx,
)?;
```

The `PhysicalSortExprNodeCollection` message some plans use is just this
list in a wrapper, so those callers wrap the result themselves rather than
this function guessing which shape they mean.

[`PhysicalSortExprNode`]: datafusion_proto_models::protobuf::PhysicalSortExprNode
