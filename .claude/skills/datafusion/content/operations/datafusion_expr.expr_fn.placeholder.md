# `datafusion_expr::expr_fn::placeholder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_fn.placeholder.json).

<a id="op-3a49ef86bd04c49c6f0072a6"></a>
## placeholder

`function` · `datafusion_expr::expr_fn::placeholder` · datafusion-expr 55.1.0

```rust
fn placeholder(id: impl Into<String>) -> Expr
```

Source: `src/expr_fn.rs:125`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create placeholder value that will be filled in (such as `$1`)

Note the parameter type can be inferred using [`Expr::infer_placeholder_types`](../operations/datafusion_expr.expr.Expr.md#op-db4bc32e974f4349ede47fce)

# Example

```rust
# use datafusion_expr::{placeholder};
let p = placeholder("$1"); // $1, refers to parameter 1
assert_eq!(p.to_string(), "$1")
```
