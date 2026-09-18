# `sqlparser::ast::ddl::OperatorClassItem::Function`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.OperatorClassItem.Function.json).

<a id="op-0909a606ef4266c3e5d0d240"></a>
## argument_types

`struct_field` · `sqlparser::ast::ddl::OperatorClassItem::Function::argument_types` · sqlparser 0.62.0

```rust
argument_types: Vec<ast::DataType>
```

Source: `src/ast/ddl.rs:4892`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Function argument types for the support function.

<a id="op-1ba52988079c6ae9f8026ae4"></a>
## function_name

`struct_field` · `sqlparser::ast::ddl::OperatorClassItem::Function::function_name` · sqlparser 0.62.0

```rust
function_name: ast::ObjectName
```

Source: `src/ast/ddl.rs:4890`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The function name implementing the support function.

<a id="op-f29d76eaebcbb3701a42c075"></a>
## op_types

`struct_field` · `sqlparser::ast::ddl::OperatorClassItem::Function::op_types` · sqlparser 0.62.0

```rust
op_types: Option<Vec<ast::DataType>>
```

Source: `src/ast/ddl.rs:4888`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional function argument types for the operator class.

<a id="op-82dd1264110701ab0aec5ebb"></a>
## support_number

`struct_field` · `sqlparser::ast::ddl::OperatorClassItem::Function::support_number` · sqlparser 0.62.0

```rust
support_number: u64
```

Source: `src/ast/ddl.rs:4886`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Support function number for this entry.
