# `datafusion_expr::logical_plan::plan::FetchType`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.plan.FetchType.json).

<a id="op-95bd54bdfc2ce4ed640a37e3"></a>
## FetchType

`enum` · `datafusion_expr::logical_plan::plan::FetchType` · datafusion-expr 55.1.0

```rust
enum FetchType
```

Source: `src/logical_plan/plan.rs:3690`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Different types of fetch expression in Limit plan.

<a id="op-4c2ef514504a5a3ae9fc83cb"></a>
## Literal

`variant` · `datafusion_expr::logical_plan::plan::FetchType::Literal` · datafusion-expr 55.1.0

```rust
Literal
```

Source: `src/logical_plan/plan.rs:3693`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The fetch expression is a literal value.
`Literal(None)` means the fetch expression is not provided.

<a id="op-73f1156aaff8cb1c2c65ed20"></a>
## UnsupportedExpr

`variant` · `datafusion_expr::logical_plan::plan::FetchType::UnsupportedExpr` · datafusion-expr 55.1.0

```rust
UnsupportedExpr
```

Source: `src/logical_plan/plan.rs:3695`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Currently only supports expressions that can be folded into constants.
