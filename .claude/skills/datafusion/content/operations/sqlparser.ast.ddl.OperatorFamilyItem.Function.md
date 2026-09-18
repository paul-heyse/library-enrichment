# `sqlparser::ast::ddl::OperatorFamilyItem::Function`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.OperatorFamilyItem.Function.json).

<a id="op-9185e0eca114d0c0d50d20d7"></a>
## argument_types

`struct_field` · `sqlparser::ast::ddl::OperatorFamilyItem::Function::argument_types` · sqlparser 0.62.0

```rust
argument_types: Vec<ast::DataType>
```

Source: `src/ast/ddl.rs:5125`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Function argument types.

<a id="op-f6c24414c678e647ed3358bf"></a>
## function_name

`struct_field` · `sqlparser::ast::ddl::OperatorFamilyItem::Function::function_name` · sqlparser 0.62.0

```rust
function_name: ast::ObjectName
```

Source: `src/ast/ddl.rs:5123`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Function name for the support function.

<a id="op-e370c50940c5836fcc4f31d1"></a>
## op_types

`struct_field` · `sqlparser::ast::ddl::OperatorFamilyItem::Function::op_types` · sqlparser 0.62.0

```rust
op_types: Option<Vec<ast::DataType>>
```

Source: `src/ast/ddl.rs:5121`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional operator argument types for the function.

<a id="op-0dafec9fd08f5551cc750474"></a>
## support_number

`struct_field` · `sqlparser::ast::ddl::OperatorFamilyItem::Function::support_number` · sqlparser 0.62.0

```rust
support_number: u64
```

Source: `src/ast/ddl.rs:5119`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Support function number.
