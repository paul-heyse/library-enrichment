# `sqlparser::ast::query::TableFactor::OpenJsonTable`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.TableFactor.OpenJsonTable.json).

<a id="op-a958b1694585d62dbbd645e5"></a>
## alias

`struct_field` · `sqlparser::ast::query::TableFactor::OpenJsonTable::alias` · sqlparser 0.62.0

```rust
alias: Option<TableAlias>
```

Source: `src/ast/query.rs:1598`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The alias for the table.

<a id="op-ecd13034abbb437b034fa132"></a>
## columns

`struct_field` · `sqlparser::ast::query::TableFactor::OpenJsonTable::columns` · sqlparser 0.62.0

```rust
columns: Vec<OpenJsonTableColumn>
```

Source: `src/ast/query.rs:1596`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The columns to be extracted from each element of the array or object.
Each column must have a name and a type.

<a id="op-a544cd1d40b4835814d96537"></a>
## json_expr

`struct_field` · `sqlparser::ast::query::TableFactor::OpenJsonTable::json_expr` · sqlparser 0.62.0

```rust
json_expr: Expr
```

Source: `src/ast/query.rs:1590`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The JSON expression to be evaluated. It must evaluate to a json string

<a id="op-e9e1b2d890c7f76eed7e7b1a"></a>
## json_path

`struct_field` · `sqlparser::ast::query::TableFactor::OpenJsonTable::json_path` · sqlparser 0.62.0

```rust
json_path: Option<ValueWithSpan>
```

Source: `src/ast/query.rs:1593`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The path to the array or object to be iterated over.
It must evaluate to a json array or object.
