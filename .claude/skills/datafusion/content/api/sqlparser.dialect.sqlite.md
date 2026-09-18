# `sqlparser::dialect::sqlite`

Crate `sqlparser` · 1 public items · structured records in [`model/sqlparser.dialect.sqlite.json`](../model/sqlparser.dialect.sqlite.json)

## SQLiteDialect

`struct` · `sqlparser::dialect::sqlite::SQLiteDialect`

Also reachable as `sqlparser::dialect::SQLiteDialect`

```rust
struct SQLiteDialect
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
fn is_delimited_identifier_start(&self, ch: char) -> bool
fn is_identifier_part(&self, ch: char) -> bool
fn is_identifier_start(&self, ch: char) -> bool
fn parse_infix(&self, parser: &mut parser::Parser<'_>, expr: &ast::Expr, _precedence: u8) -> Option<Result<ast::Expr, ParserError>>
fn parse_statement(&self, parser: &mut Parser<'_>) -> Option<Result<Statement, ParserError>>
fn supports_asc_desc_in_column_definition(&self) -> bool
fn supports_comma_separated_trim(&self) -> bool
fn supports_dollar_placeholder(&self) -> bool
fn supports_filter_during_aggregation(&self) -> bool
fn supports_in_empty_list(&self) -> bool
fn supports_limit_comma(&self) -> bool
fn supports_notnull_operator(&self) -> bool
fn supports_start_transaction_modifier(&self) -> bool
```

[Full member, field, variant and typed contracts](../operations/sqlparser.dialect.sqlite.SQLiteDialect.md).


A [`Dialect`] for [SQLite](https://www.sqlite.org)

This dialect allows columns in a
[`CREATE TABLE`](https://sqlite.org/lang_createtable.html) statement with no
type specified, as in `CREATE TABLE t1 (a)`. In the AST, these columns will
have the data type [`Unspecified`](crate::ast::DataType::Unspecified).

---
