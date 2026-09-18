# `sqlparser::ast::Expr::UnaryOp`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Expr.UnaryOp.json).

<a id="op-dbaba537025a04d79b8e7d17"></a>
## expr

`struct_field` · `sqlparser::ast::Expr::UnaryOp::expr` · sqlparser 0.62.0

```rust
expr: Box<Expr>
```

Source: `src/ast/mod.rs:1065`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Operand expression.

<a id="op-29bbe38eb1802a605a526c05"></a>
## op

`struct_field` · `sqlparser::ast::Expr::UnaryOp::op` · sqlparser 0.62.0

```rust
op: UnaryOperator
```

Source: `src/ast/mod.rs:1063`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The unary operator (e.g., `NOT`, `-`).
