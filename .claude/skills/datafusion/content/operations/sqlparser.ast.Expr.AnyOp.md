# `sqlparser::ast::Expr::AnyOp`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Expr.AnyOp.json).

<a id="op-1e564d3f450ac43609afd26a"></a>
## compare_op

`struct_field` · `sqlparser::ast::Expr::AnyOp::compare_op` · sqlparser 0.62.0

```rust
compare_op: BinaryOperator
```

Source: `src/ast/mod.rs:1043`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Comparison operator.

<a id="op-bf92b756e76caa79dd8c42aa"></a>
## is_some

`struct_field` · `sqlparser::ast::Expr::AnyOp::is_some` · sqlparser 0.62.0

```rust
is_some: bool
```

Source: `src/ast/mod.rs:1047`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ANY and SOME are synonymous: <https://docs.cloudera.com/cdw-runtime/cloud/using-hiveql/topics/hive_comparison_predicates.html>

<a id="op-08ba62cad4a638ac5218970e"></a>
## left

`struct_field` · `sqlparser::ast::Expr::AnyOp::left` · sqlparser 0.62.0

```rust
left: Box<Expr>
```

Source: `src/ast/mod.rs:1041`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Left operand.

<a id="op-d3b42971368bbfb8cdd32ea0"></a>
## right

`struct_field` · `sqlparser::ast::Expr::AnyOp::right` · sqlparser 0.62.0

```rust
right: Box<Expr>
```

Source: `src/ast/mod.rs:1045`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Right-hand subquery expression.
