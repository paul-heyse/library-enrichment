# `sqlparser::ast::query::SetExpr::SetOperation`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.SetExpr.SetOperation.json).

<a id="op-5aec4632c210b54969469e3f"></a>
## left

`struct_field` · `sqlparser::ast::query::SetExpr::SetOperation::left` · sqlparser 0.62.0

```rust
left: Box<SetExpr>
```

Source: `src/ast/query.rs:160`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Left operand of the set operation.

<a id="op-acea87f010c0ccee760eda9c"></a>
## op

`struct_field` · `sqlparser::ast::query::SetExpr::SetOperation::op` · sqlparser 0.62.0

```rust
op: SetOperator
```

Source: `src/ast/query.rs:162`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The set operator used (e.g. `UNION`, `EXCEPT`).

<a id="op-39f08e3f4582f20f8b6200d4"></a>
## right

`struct_field` · `sqlparser::ast::query::SetExpr::SetOperation::right` · sqlparser 0.62.0

```rust
right: Box<SetExpr>
```

Source: `src/ast/query.rs:166`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Right operand of the set operation.

<a id="op-340b5afbf8eb43b1a7344ce8"></a>
## set_quantifier

`struct_field` · `sqlparser::ast::query::SetExpr::SetOperation::set_quantifier` · sqlparser 0.62.0

```rust
set_quantifier: SetQuantifier
```

Source: `src/ast/query.rs:164`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional quantifier (`ALL`, `DISTINCT`, etc.).
