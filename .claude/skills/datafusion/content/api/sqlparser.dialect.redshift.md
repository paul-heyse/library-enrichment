# `sqlparser::dialect::redshift`

Crate `sqlparser` · 1 public items · structured records in [`model/sqlparser.dialect.redshift.json`](../model/sqlparser.dialect.redshift.json)

## RedshiftSqlDialect

`struct` · `sqlparser::dialect::redshift::RedshiftSqlDialect`

Also reachable as `sqlparser::dialect::RedshiftSqlDialect`

```rust
struct RedshiftSqlDialect
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
fn allow_extract_single_quotes(&self) -> bool
fn convert_type_before_value(&self) -> bool
fn is_identifier_part(&self, ch: char) -> bool
fn is_identifier_start(&self, ch: char) -> bool
fn is_nested_delimited_identifier_start(&self, ch: char) -> bool
fn peek_nested_delimited_identifier_quotes(&self, chars: Peekable<Chars<'_>>) -> Option<(char, Option<char>)>
fn supports_array_typedef_with_brackets(&self) -> bool
fn supports_bitwise_shift_operators(&self) -> bool
fn supports_connect_by(&self) -> bool
fn supports_create_table_like_parenthesized(&self) -> bool
fn supports_geometric_types(&self) -> bool
fn supports_partiql(&self) -> bool
fn supports_select_exclude(&self) -> bool
fn supports_select_wildcard_exclude(&self) -> bool
fn supports_select_wildcard_with_alias(&self) -> bool
fn supports_string_escape_constant(&self) -> bool
fn supports_string_literal_backslash_escape(&self) -> bool
fn supports_string_literal_concatenation_with_newline(&self) -> bool
fn supports_top_before_distinct(&self) -> bool
fn supports_window_function_null_treatment_arg(&self) -> bool
```

[Full member, field, variant and typed contracts](../operations/sqlparser.dialect.redshift.RedshiftSqlDialect.md).


A [`Dialect`] for [RedShift](https://aws.amazon.com/redshift/)

---
