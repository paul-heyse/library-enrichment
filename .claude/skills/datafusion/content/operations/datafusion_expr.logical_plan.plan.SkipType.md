# `datafusion_expr::logical_plan::plan::SkipType`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.plan.SkipType.json).

<a id="op-77b06957c2a7dc03f4b80cba"></a>
## SkipType

`enum` · `datafusion_expr::logical_plan::plan::SkipType` · datafusion-expr 55.1.0

```rust
enum SkipType
```

Source: `src/logical_plan/plan.rs:3682`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Different types of skip expression in Limit plan.

<a id="op-511313f1845e9dbc7faa4868"></a>
## Literal

`variant` · `datafusion_expr::logical_plan::plan::SkipType::Literal` · datafusion-expr 55.1.0

```rust
Literal
```

Source: `src/logical_plan/plan.rs:3684`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The skip expression is a literal value.

<a id="op-466ed7e55251b177006168f1"></a>
## UnsupportedExpr

`variant` · `datafusion_expr::logical_plan::plan::SkipType::UnsupportedExpr` · datafusion-expr 55.1.0

```rust
UnsupportedExpr
```

Source: `src/logical_plan/plan.rs:3686`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Currently only supports expressions that can be folded into constants.
