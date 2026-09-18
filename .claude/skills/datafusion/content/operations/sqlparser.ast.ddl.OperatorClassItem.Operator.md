# `sqlparser::ast::ddl::OperatorClassItem::Operator`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.OperatorClassItem.Operator.json).

<a id="op-561b85b3bad95bf488f985c4"></a>
## op_types

`struct_field` · `sqlparser::ast::ddl::OperatorClassItem::Operator::op_types` · sqlparser 0.62.0

```rust
op_types: Option<OperatorArgTypes>
```

Source: `src/ast/ddl.rs:4879`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional operator argument types.

<a id="op-237079e2d8a2b5535242e463"></a>
## operator_name

`struct_field` · `sqlparser::ast::ddl::OperatorClassItem::Operator::operator_name` · sqlparser 0.62.0

```rust
operator_name: ast::ObjectName
```

Source: `src/ast/ddl.rs:4877`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The operator name referenced by this clause.

<a id="op-a5bd3a0e1a2ee2a1e6d6df50"></a>
## purpose

`struct_field` · `sqlparser::ast::ddl::OperatorClassItem::Operator::purpose` · sqlparser 0.62.0

```rust
purpose: Option<OperatorPurpose>
```

Source: `src/ast/ddl.rs:4881`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional purpose such as `FOR SEARCH` or `FOR ORDER BY`.

<a id="op-fab6c37853180ed5e732dc97"></a>
## strategy_number

`struct_field` · `sqlparser::ast::ddl::OperatorClassItem::Operator::strategy_number` · sqlparser 0.62.0

```rust
strategy_number: u64
```

Source: `src/ast/ddl.rs:4875`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Strategy number identifying the operator position in the opclass.
