# `sqlparser::dialect::mssql`

Crate `sqlparser` · 1 public items · structured records in [`model/sqlparser.dialect.mssql.json`](../model/sqlparser.dialect.mssql.json)

## MsSqlDialect

`struct` · `sqlparser::dialect::mssql::MsSqlDialect`

Also reachable as `sqlparser::dialect::MsSqlDialect`

```rust
struct MsSqlDialect
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
fn convert_type_before_value(&self) -> bool
fn get_next_precedence(&self, parser: &Parser<'_>) -> Option<Result<u8, ParserError>>
fn get_reserved_grantees_types(&self) -> &[GranteesType]
fn identifier_quote_style(&self, _identifier: &str) -> Option<char>
fn is_delimited_identifier_start(&self, ch: char) -> bool
fn is_identifier_part(&self, ch: char) -> bool
fn is_identifier_start(&self, ch: char) -> bool
fn is_select_item_alias(&self, explicit: bool, kw: &Keyword, parser: &mut Parser<'_>) -> bool
fn is_table_factor_alias(&self, explicit: bool, kw: &Keyword, parser: &mut Parser<'_>) -> bool
fn parse_statement(&self, parser: &mut Parser<'_>) -> Option<Result<Statement, ParserError>>
fn supports_boolean_literals(&self) -> bool
fn supports_connect_by(&self) -> bool
fn supports_dollar_as_money_prefix(&self) -> bool
fn supports_end_transaction_modifier(&self) -> bool
fn supports_eq_alias_assignment(&self) -> bool
fn supports_named_fn_args_with_colon_operator(&self) -> bool
fn supports_named_fn_args_with_expr_name(&self) -> bool
fn supports_named_fn_args_with_rarrow_operator(&self) -> bool
fn supports_nested_comments(&self) -> bool
fn supports_object_name_double_dot_notation(&self) -> bool
fn supports_outer_join_operator(&self) -> bool
fn supports_set_stmt_without_operator(&self) -> bool
fn supports_start_transaction_modifier(&self) -> bool
fn supports_table_versioning(&self) -> bool
fn supports_try_convert(&self) -> bool
```

A [`Dialect`] for [Microsoft SQL Server](https://www.microsoft.com/en-us/sql-server/)

---
