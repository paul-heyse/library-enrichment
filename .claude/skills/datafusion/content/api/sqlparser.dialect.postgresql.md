# `sqlparser::dialect::postgresql`

Crate `sqlparser` · 1 public items · structured records in [`model/sqlparser.dialect.postgresql.json`](../model/sqlparser.dialect.postgresql.json)

## PostgreSqlDialect

`struct` · `sqlparser::dialect::postgresql::PostgreSqlDialect`

Also reachable as `sqlparser::dialect::PostgreSqlDialect`

```rust
struct PostgreSqlDialect
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
fn allow_extract_custom(&self) -> bool
fn allow_extract_single_quotes(&self) -> bool
fn get_next_precedence(&self, parser: &Parser<'_>) -> Option<Result<u8, ParserError>>
fn identifier_quote_style(&self, _identifier: &str) -> Option<char>
fn is_custom_operator_part(&self, ch: char) -> bool
fn is_delimited_identifier_start(&self, ch: char) -> bool
fn is_identifier_part(&self, ch: char) -> bool
fn is_identifier_start(&self, ch: char) -> bool
fn is_reserved_for_identifier(&self, kw: Keyword) -> bool
fn prec_value(&self, prec: Precedence) -> u8
fn supports_alter_column_type_using(&self) -> bool
fn supports_array_typedef_with_brackets(&self) -> bool
fn supports_bitwise_shift_operators(&self) -> bool
fn supports_comma_separated_trim(&self) -> bool
fn supports_comment_on(&self) -> bool
fn supports_comment_optimizer_hint(&self) -> bool
fn supports_create_index_with_clause(&self) -> bool
fn supports_create_table_like_parenthesized(&self) -> bool
fn supports_empty_projections(&self) -> bool
fn supports_explain_with_utility_options(&self) -> bool
fn supports_factorial_operator(&self) -> bool
fn supports_filter_during_aggregation(&self) -> bool
fn supports_geometric_types(&self) -> bool
fn supports_group_by_expr(&self) -> bool
fn supports_insert_table_alias(&self) -> bool
fn supports_interval_options(&self) -> bool
fn supports_listen_notify(&self) -> bool
fn supports_load_extension(&self) -> bool
fn supports_named_fn_args_with_colon_operator(&self) -> bool
fn supports_named_fn_args_with_expr_name(&self) -> bool
fn supports_nested_comments(&self) -> bool
fn supports_notnull_operator(&self) -> bool
fn supports_numeric_literal_underscores(&self) -> bool
fn supports_select_wildcard_with_alias(&self) -> bool
fn supports_set_names(&self) -> bool
fn supports_string_escape_constant(&self) -> bool
fn supports_unicode_string_literal(&self) -> bool
fn supports_xml_expressions(&self) -> bool
```

[Full member, field, variant and typed contracts](../operations/sqlparser.dialect.postgresql.PostgreSqlDialect.md).


A [`Dialect`] for [PostgreSQL](https://www.postgresql.org/)

---
