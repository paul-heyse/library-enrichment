# `sqlparser::ast::ddl::OperatorFamilyItem::Operator`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.OperatorFamilyItem.Operator.json).

<a id="op-e789e0af4e285f79cea95cca"></a>
## op_types

`struct_field` · `sqlparser::ast::ddl::OperatorFamilyItem::Operator::op_types` · sqlparser 0.62.0

```rust
op_types: Vec<ast::DataType>
```

Source: `src/ast/ddl.rs:5112`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Operator argument types.

<a id="op-a7f2bb865096826b5fc5ac96"></a>
## operator_name

`struct_field` · `sqlparser::ast::ddl::OperatorFamilyItem::Operator::operator_name` · sqlparser 0.62.0

```rust
operator_name: ast::ObjectName
```

Source: `src/ast/ddl.rs:5110`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Operator name referenced by this entry.

<a id="op-c279bfa8e6f6f943a415d816"></a>
## purpose

`struct_field` · `sqlparser::ast::ddl::OperatorFamilyItem::Operator::purpose` · sqlparser 0.62.0

```rust
purpose: Option<OperatorPurpose>
```

Source: `src/ast/ddl.rs:5114`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional purpose such as `FOR SEARCH` or `FOR ORDER BY`.

<a id="op-422b5a5b03b0aa4fa270ecfc"></a>
## strategy_number

`struct_field` · `sqlparser::ast::ddl::OperatorFamilyItem::Operator::strategy_number` · sqlparser 0.62.0

```rust
strategy_number: u64
```

Source: `src/ast/ddl.rs:5108`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Strategy number for the operator.
