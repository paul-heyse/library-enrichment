# `sqlparser::dialect::mysql`

Crate `sqlparser` · 1 public items · structured records in [`model/sqlparser.dialect.mysql.json`](../model/sqlparser.dialect.mysql.json)

## MySqlDialect

`struct` · `sqlparser::dialect::mysql::MySqlDialect`

Also reachable as `sqlparser::dialect::MySqlDialect`

```rust
struct MySqlDialect
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
fn identifier_quote_style(&self, _identifier: &str) -> Option<char>
fn ignores_wildcard_escapes(&self) -> bool
fn is_delimited_identifier_start(&self, ch: char) -> bool
fn is_identifier_part(&self, ch: char) -> bool
fn is_identifier_start(&self, ch: char) -> bool
fn is_table_factor_alias(&self, explicit: bool, kw: &Keyword, _parser: &mut Parser<'_>) -> bool
fn parse_infix(&self, parser: &mut parser::Parser<'_>, expr: &ast::Expr, _precedence: u8) -> Option<Result<ast::Expr, ParserError>>
fn parse_statement(&self, parser: &mut Parser<'_>) -> Option<Result<Statement, ParserError>>
fn require_interval_qualifier(&self) -> bool
fn requires_single_line_comment_whitespace(&self) -> bool
fn supports_binary_kw_as_cast(&self) -> bool
fn supports_bitwise_shift_operators(&self) -> bool
fn supports_comma_separated_set_assignments(&self) -> bool
fn supports_comment_optimizer_hint(&self) -> bool
fn supports_constraint_keyword_without_name(&self) -> bool
fn supports_create_table_select(&self) -> bool
fn supports_cross_join_constraint(&self) -> bool
fn supports_data_type_signed_suffix(&self) -> bool
fn supports_double_ampersand_operator(&self) -> bool
fn supports_insert_set(&self) -> bool
fn supports_key_column_option(&self) -> bool
fn supports_limit_comma(&self) -> bool
fn supports_match_against(&self) -> bool
fn supports_multiline_comment_hints(&self) -> bool
fn supports_numeric_prefix(&self) -> bool
fn supports_select_modifiers(&self) -> bool
fn supports_set_names(&self) -> bool
fn supports_string_literal_backslash_escape(&self) -> bool
fn supports_string_literal_concatenation(&self) -> bool
fn supports_table_hints(&self) -> bool
fn supports_update_order_by(&self) -> bool
fn supports_user_host_grantee(&self) -> bool
```

A [`Dialect`] for [MySQL](https://www.mysql.com/)

---
