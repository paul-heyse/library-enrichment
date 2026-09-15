# `datafusion_physical_expr_common`

Crate `datafusion-physical-expr-common` · 1 public items · structured records in [`model/datafusion_physical_expr_common.json`](../model/datafusion_physical_expr_common.json)

## expect_expr_variant

`macro` · `datafusion_physical_expr_common::expect_expr_variant`

Also reachable as `datafusion::physical_expr_common::expect_expr_variant`, `datafusion_physical_expr_common::physical_expr::proto_decode::expect_expr_variant`

```rust
macro_rules! expect_expr_variant
```

Open the outer [`PhysicalExprNode`] and assert it carries the expected
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

---
