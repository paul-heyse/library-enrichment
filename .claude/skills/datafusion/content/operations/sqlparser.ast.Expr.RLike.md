# `sqlparser::ast::Expr::RLike`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Expr.RLike.json).

<a id="op-f527daa525b2883e0d626089"></a>
## expr

`struct_field` · `sqlparser::ast::Expr::RLike::expr` · sqlparser 0.62.0

```rust
expr: Box<Expr>
```

Source: `src/ast/mod.rs:1031`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Expression to test.

<a id="op-d93122af5d6734077be8b2d4"></a>
## negated

`struct_field` · `sqlparser::ast::Expr::RLike::negated` · sqlparser 0.62.0

```rust
negated: bool
```

Source: `src/ast/mod.rs:1029`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`true` when `NOT` is present.

<a id="op-15bb434138b9a4230ae254b4"></a>
## pattern

`struct_field` · `sqlparser::ast::Expr::RLike::pattern` · sqlparser 0.62.0

```rust
pattern: Box<Expr>
```

Source: `src/ast/mod.rs:1033`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Pattern expression.

<a id="op-3afb113324bd14d307ef4da2"></a>
## regexp

`struct_field` · `sqlparser::ast::Expr::RLike::regexp` · sqlparser 0.62.0

```rust
regexp: bool
```

Source: `src/ast/mod.rs:1035`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

true for REGEXP, false for RLIKE (no difference in semantics)
