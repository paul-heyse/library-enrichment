# `sqlparser::ast::query::TableFactor::JsonTable`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.TableFactor.JsonTable.json).

<a id="op-13ca1e14a42721f8d100ab93"></a>
## alias

`struct_field` · `sqlparser::ast::query::TableFactor::JsonTable::alias` · sqlparser 0.62.0

```rust
alias: Option<TableAlias>
```

Source: `src/ast/query.rs:1577`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The alias for the table.

<a id="op-cd93781a9fb3f01a681b504b"></a>
## columns

`struct_field` · `sqlparser::ast::query::TableFactor::JsonTable::columns` · sqlparser 0.62.0

```rust
columns: Vec<JsonTableColumn>
```

Source: `src/ast/query.rs:1575`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The columns to be extracted from each element of the array or object.
Each column must have a name and a type.

<a id="op-57d09903347722eaff3f983f"></a>
## json_expr

`struct_field` · `sqlparser::ast::query::TableFactor::JsonTable::json_expr` · sqlparser 0.62.0

```rust
json_expr: Expr
```

Source: `src/ast/query.rs:1569`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The JSON expression to be evaluated. It must evaluate to a json string

<a id="op-957ff81a365f0c47f68ea62f"></a>
## json_path

`struct_field` · `sqlparser::ast::query::TableFactor::JsonTable::json_path` · sqlparser 0.62.0

```rust
json_path: ValueWithSpan
```

Source: `src/ast/query.rs:1572`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The path to the array or object to be iterated over.
It must evaluate to a json array or object.
