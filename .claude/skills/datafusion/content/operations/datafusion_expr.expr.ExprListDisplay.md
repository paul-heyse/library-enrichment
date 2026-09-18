# `datafusion_expr::expr::ExprListDisplay`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr.ExprListDisplay.json).

<a id="op-00539b285b66cb6d7e8c2a7e"></a>
## ExprListDisplay

`struct` · `datafusion_expr::expr::ExprListDisplay` · datafusion-expr 55.1.0

```rust
struct ExprListDisplay<'a>
```

Source: `src/expr.rs:3491`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Formats a list of `&Expr` with a custom separator using SQL display format

<a id="op-cd96d6ef4681efdcc01a377d"></a>
## comma_separated

`function` · `datafusion_expr::expr::ExprListDisplay::comma_separated` · datafusion-expr 55.1.0

```rust
fn comma_separated(exprs: &'a [Expr]) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_expr::expr::ExprListDisplay", "path": "ExprListDisplay"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3496, 1], "end": [3506, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:3503`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new display struct with comma-space separator

<a id="op-294cf8a1ba7ee50baea0b721"></a>
## fmt

`function` · `datafusion_expr::expr::ExprListDisplay::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_expr::expr::ExprListDisplay", "path": "ExprListDisplay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3508, 1], "end": [3520, 2], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/expr.rs:3509`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d697331460dbffeceff61c6"></a>
## new

`function` · `datafusion_expr::expr::ExprListDisplay::new` · datafusion-expr 55.1.0

```rust
fn new(exprs: &'a [Expr], sep: &'a str) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_expr::expr::ExprListDisplay", "path": "ExprListDisplay"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3496, 1], "end": [3506, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:3498`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new display struct with the given expressions and separator
