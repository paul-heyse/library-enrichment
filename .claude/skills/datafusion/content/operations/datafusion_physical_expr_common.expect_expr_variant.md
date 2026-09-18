# `datafusion_physical_expr_common::expect_expr_variant`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.expect_expr_variant.json).

<a id="op-9270e53dda58fabafb20a77f"></a>
## expect_expr_variant

`macro` · `datafusion_physical_expr_common::expect_expr_variant` · datafusion-physical-expr-common 55.1.0

```rust
macro_rules! expect_expr_variant
```

Source: `src/physical_expr.rs:648`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Open the outer [`PhysicalExprNode`](../operations/datafusion_proto_models.generated.datafusion.PhysicalExprNode.md#op-48370551e402edd403f77491) and assert it carries the expected
`ExprType` variant, returning the inner payload (auto-derefs through
`Box`) or bailing with an `Internal` error.

Every `try_from_proto` starts with the same six-line `match`:

```ignore
let try_cast = match &node.expr_type {
    Some(protobuf::physical_expr_node::ExprType::TryCast(x)) => x.as_ref(),
    _ => return internal_err!("PhysicalExprNode is not a TryCastExpr"),
};
```

With this macro that collapses to:

```ignore
let try_cast = expect_expr_variant!(
    node,
    protobuf::physical_expr_node::ExprType::TryCast,
    "TryCastExpr",
);
```

Pass the variant as a `::` path so the macro stays agnostic to how
the caller imports the proto types.
