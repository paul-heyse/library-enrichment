# `sqlparser::ast::Expr::BinaryOp`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Expr.BinaryOp.json).

<a id="op-35adafa9b3d3853fcc7fe92b"></a>
## left

`struct_field` · `sqlparser::ast::Expr::BinaryOp::left` · sqlparser 0.62.0

```rust
left: Box<Expr>
```

Source: `src/ast/mod.rs:981`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Left operand.

<a id="op-f5476cdccff6e9ff706a23d9"></a>
## op

`struct_field` · `sqlparser::ast::Expr::BinaryOp::op` · sqlparser 0.62.0

```rust
op: BinaryOperator
```

Source: `src/ast/mod.rs:983`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Operator between operands.

<a id="op-929a14ed4ebc7958a1be62c6"></a>
## right

`struct_field` · `sqlparser::ast::Expr::BinaryOp::right` · sqlparser 0.62.0

```rust
right: Box<Expr>
```

Source: `src/ast/mod.rs:985`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Right operand.
