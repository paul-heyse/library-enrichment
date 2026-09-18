# `sqlparser::dialect::databricks`

Crate `sqlparser` · 1 public items · structured records in [`model/sqlparser.dialect.databricks.json`](../model/sqlparser.dialect.databricks.json)

## DatabricksDialect

`struct` · `sqlparser::dialect::databricks::DatabricksDialect`

Also reachable as `sqlparser::dialect::DatabricksDialect`

```rust
struct DatabricksDialect
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
fn is_delimited_identifier_start(&self, ch: char) -> bool
fn is_identifier_part(&self, ch: char) -> bool
fn is_identifier_start(&self, ch: char) -> bool
fn require_interval_qualifier(&self) -> bool
fn supports_bang_not_operator(&self) -> bool
fn supports_cte_without_as(&self) -> bool
fn supports_filter_during_aggregation(&self) -> bool
fn supports_group_by_expr(&self) -> bool
fn supports_group_by_with_modifier(&self) -> bool
fn supports_lambda_functions(&self) -> bool
fn supports_nested_comments(&self) -> bool
fn supports_numeric_prefix(&self) -> bool
fn supports_optimize_table(&self) -> bool
fn supports_select_item_multi_column_alias(&self) -> bool
fn supports_select_wildcard_except(&self) -> bool
fn supports_struct_literal(&self) -> bool
fn supports_table_versioning(&self) -> bool
fn supports_values_as_table_factor(&self) -> bool
```

[Full member, field, variant and typed contracts](../operations/sqlparser.dialect.databricks.DatabricksDialect.md).


A [`Dialect`] for [Databricks SQL](https://www.databricks.com/)

See <https://docs.databricks.com/en/sql/language-manual/index.html>.

---
