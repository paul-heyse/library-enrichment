# `sqlparser::ast::Expr::AllOp`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Expr.AllOp.json).

<a id="op-12f743418e90aea550693056"></a>
## compare_op

`struct_field` · `sqlparser::ast::Expr::AllOp::compare_op` · sqlparser 0.62.0

```rust
compare_op: BinaryOperator
```

Source: `src/ast/mod.rs:1055`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Comparison operator.

<a id="op-3a30b9a7abb1ed660a2ec1aa"></a>
## left

`struct_field` · `sqlparser::ast::Expr::AllOp::left` · sqlparser 0.62.0

```rust
left: Box<Expr>
```

Source: `src/ast/mod.rs:1053`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Left operand.

<a id="op-ce2c58a43b450d0968139edb"></a>
## right

`struct_field` · `sqlparser::ast::Expr::AllOp::right` · sqlparser 0.62.0

```rust
right: Box<Expr>
```

Source: `src/ast/mod.rs:1057`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Right-hand subquery expression.
