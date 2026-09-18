# `sqlparser::ast::Expr::Floor`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Expr.Floor.json).

<a id="op-15c9477abccb2cec5bd68540"></a>
## expr

`struct_field` · `sqlparser::ast::Expr::Floor::expr` · sqlparser 0.62.0

```rust
expr: Box<Expr>
```

Source: `src/ast/mod.rs:1146`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Expression to floor.

<a id="op-61364bb21ad888ca65844c74"></a>
## field

`struct_field` · `sqlparser::ast::Expr::Floor::field` · sqlparser 0.62.0

```rust
field: CeilFloorKind
```

Source: `src/ast/mod.rs:1148`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The CEIL/FLOOR kind (datetime field or scale).
