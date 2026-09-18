# `sqlparser::dialect::bigquery`

Crate `sqlparser` · 1 public items · structured records in [`model/sqlparser.dialect.bigquery.json`](../model/sqlparser.dialect.bigquery.json)

## BigQueryDialect

`struct` · `sqlparser::dialect::bigquery::BigQueryDialect`

Also reachable as `sqlparser::dialect::BigQueryDialect`

```rust
struct BigQueryDialect
```

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`, `sqlparser::dialect::Dialect`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

**via `sqlparser::dialect::Dialect`**

```rust
fn is_column_alias(&self, kw: &Keyword, _parser: &mut Parser<'_>) -> bool
fn is_delimited_identifier_start(&self, ch: char) -> bool
fn is_identifier_part(&self, ch: char) -> bool
fn is_identifier_start(&self, ch: char) -> bool
fn parse_statement(&self, parser: &mut Parser<'_>) -> Option<Result<Statement, ParserError>>
fn require_interval_qualifier(&self) -> bool
fn supports_column_definition_trailing_commas(&self) -> bool
fn supports_comma_separated_trim(&self) -> bool
fn supports_create_table_multi_schema_info_sources(&self) -> bool
fn supports_execute_immediate(&self) -> bool
fn supports_group_by_expr(&self) -> bool
fn supports_parenthesized_set_variables(&self) -> bool
fn supports_pipe_operator(&self) -> bool
fn supports_projection_trailing_commas(&self) -> bool
fn supports_select_expr_star(&self) -> bool
fn supports_select_wildcard_except(&self) -> bool
fn supports_select_wildcard_replace(&self) -> bool
fn supports_string_literal_backslash_escape(&self) -> bool
fn supports_struct_literal(&self) -> bool
fn supports_table_versioning(&self) -> bool
fn supports_triple_quoted_string(&self) -> bool
fn supports_window_clause_named_window_reference(&self) -> bool
fn supports_window_function_null_treatment_arg(&self) -> bool
```

[Full member, field, variant and typed contracts](../operations/sqlparser.dialect.bigquery.BigQueryDialect.md).


A [`Dialect`] for [Google Bigquery](https://cloud.google.com/bigquery/)

---
