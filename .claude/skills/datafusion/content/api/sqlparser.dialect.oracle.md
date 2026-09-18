# `sqlparser::dialect::oracle`

Crate `sqlparser` · 1 public items · structured records in [`model/sqlparser.dialect.oracle.json`](../model/sqlparser.dialect.oracle.json)

## OracleDialect

`struct` · `sqlparser::dialect::oracle::OracleDialect`

Also reachable as `sqlparser::dialect::OracleDialect`

```rust
struct OracleDialect
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
fn get_next_precedence(&self, parser: &Parser<'_>) -> Option<Result<u8, ParserError>>
fn get_reserved_keywords_for_select_item_operator(&self) -> &[Keyword]
fn identifier_quote_style(&self, _identifier: &str) -> Option<char>
fn is_delimited_identifier_start(&self, ch: char) -> bool
fn is_identifier_part(&self, ch: char) -> bool
fn is_identifier_start(&self, ch: char) -> bool
fn supports_boolean_literals(&self) -> bool
fn supports_comment_on(&self) -> bool
fn supports_comment_optimizer_hint(&self) -> bool
fn supports_connect_by(&self) -> bool
fn supports_create_table_select(&self) -> bool
fn supports_execute_immediate(&self) -> bool
fn supports_group_by_expr(&self) -> bool
fn supports_insert_table_alias(&self) -> bool
fn supports_insert_table_query(&self) -> bool
fn supports_match_recognize(&self) -> bool
fn supports_outer_join_operator(&self) -> bool
fn supports_quote_delimited_string(&self) -> bool
fn supports_set_stmt_without_operator(&self) -> bool
fn supports_window_function_null_treatment_arg(&self) -> bool
```

[Full member, field, variant and typed contracts](../operations/sqlparser.dialect.oracle.OracleDialect.md).


A [`Dialect`] for [Oracle Databases](https://docs.oracle.com/en/database/oracle/oracle-database/21/sqlrf/index.html)

---
