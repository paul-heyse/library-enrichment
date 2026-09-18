# `sqlparser::ast::Expr::Collate`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Expr.Collate.json).

<a id="op-946238ca89bdf9539e7ced66"></a>
## collation

`struct_field` · `sqlparser::ast::Expr::Collate::collation` · sqlparser 0.62.0

```rust
collation: ObjectName
```

Source: `src/ast/mod.rs:1216`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The collation name to apply to the expression.

<a id="op-096df22624a033114522a17a"></a>
## expr

`struct_field` · `sqlparser::ast::Expr::Collate::expr` · sqlparser 0.62.0

```rust
expr: Box<Expr>
```

Source: `src/ast/mod.rs:1214`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The expression being collated.
