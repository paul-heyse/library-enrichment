# `sqlparser::ast::Expr::JsonAccess`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Expr.JsonAccess.json).

<a id="op-27a44427d3033e7e2399dde8"></a>
## path

`struct_field` · `sqlparser::ast::Expr::JsonAccess::path` · sqlparser 0.62.0

```rust
path: JsonPath
```

Source: `src/ast/mod.rs:909`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The path to the data to extract.

<a id="op-d93d00655a46627bbbc4b788"></a>
## value

`struct_field` · `sqlparser::ast::Expr::JsonAccess::value` · sqlparser 0.62.0

```rust
value: Box<Expr>
```

Source: `src/ast/mod.rs:907`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The value being queried.
