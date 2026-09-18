# `sqlparser::ast::Expr::Prefixed`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Expr.Prefixed.json).

<a id="op-c91ca4bd442f391a79198122"></a>
## prefix

`struct_field` · `sqlparser::ast::Expr::Prefixed::prefix` · sqlparser 0.62.0

```rust
prefix: Ident
```

Source: `src/ast/mod.rs:1227`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The prefix identifier (introducer or projection prefix).

<a id="op-751ba01b2c26f2c23ccc53f0"></a>
## value

`struct_field` · `sqlparser::ast::Expr::Prefixed::value` · sqlparser 0.62.0

```rust
value: Box<Expr>
```

Source: `src/ast/mod.rs:1230`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The value expression being prefixed.
Hint: you can unwrap the string value using `value.into_string()`.
