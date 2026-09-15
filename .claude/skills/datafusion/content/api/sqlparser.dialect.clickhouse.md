# `sqlparser::dialect::clickhouse`

Crate `sqlparser` · 1 public items · structured records in [`model/sqlparser.dialect.clickhouse.json`](../model/sqlparser.dialect.clickhouse.json)

## ClickHouseDialect

`struct` · `sqlparser::dialect::clickhouse::ClickHouseDialect`

Also reachable as `sqlparser::dialect::ClickHouseDialect`

```rust
struct ClickHouseDialect
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
fn describe_requires_table_keyword(&self) -> bool
fn is_identifier_part(&self, ch: char) -> bool
fn is_identifier_start(&self, ch: char) -> bool
fn require_interval_qualifier(&self) -> bool
fn supports_array_join_syntax(&self) -> bool
fn supports_comma_separated_trim(&self) -> bool
fn supports_dictionary_syntax(&self) -> bool
fn supports_from_first_select(&self) -> bool
fn supports_group_by_expr(&self) -> bool
fn supports_group_by_with_modifier(&self) -> bool
fn supports_insert_format(&self) -> bool
fn supports_insert_table_function(&self) -> bool
fn supports_interpolate(&self) -> bool
fn supports_lambda_functions(&self) -> bool
fn supports_limit_by(&self) -> bool
fn supports_limit_comma(&self) -> bool
fn supports_nested_comments(&self) -> bool
fn supports_numeric_literal_underscores(&self) -> bool
fn supports_optimize_table(&self) -> bool
fn supports_order_by_all(&self) -> bool
fn supports_partition_by_after_order_by(&self) -> bool
fn supports_prewhere(&self) -> bool
fn supports_select_format(&self) -> bool
fn supports_select_wildcard_except(&self) -> bool
fn supports_select_wildcard_replace(&self) -> bool
fn supports_settings(&self) -> bool
fn supports_string_literal_backslash_escape(&self) -> bool
fn supports_with_fill(&self) -> bool
```

A [`Dialect`] for [ClickHouse](https://clickhouse.com/).

---
