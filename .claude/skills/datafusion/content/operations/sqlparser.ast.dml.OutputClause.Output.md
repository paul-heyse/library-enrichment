# `sqlparser::ast::dml::OutputClause::Output`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.dml.OutputClause.Output.json).

<a id="op-d7e1f5c17a54beba20ea73b2"></a>
## into_table

`struct_field` · `sqlparser::ast::dml::OutputClause::Output::into_table` · sqlparser 0.62.0

```rust
into_table: Option<super::SelectInto>
```

Source: `src/ast/dml.rs:766`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `INTO` table to direct the output

<a id="op-11bd63faf2440413897b14b6"></a>
## output_token

`struct_field` · `sqlparser::ast::dml::OutputClause::Output::output_token` · sqlparser 0.62.0

```rust
output_token: super::helpers::attached_token::AttachedToken
```

Source: `src/ast/dml.rs:762`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `OUTPUT` token that starts the sub-expression.

<a id="op-1ff3b37b18b09fc4a84eb2df"></a>
## select_items

`struct_field` · `sqlparser::ast::dml::OutputClause::Output::select_items` · sqlparser 0.62.0

```rust
select_items: Vec<super::SelectItem>
```

Source: `src/ast/dml.rs:764`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The select items to output
