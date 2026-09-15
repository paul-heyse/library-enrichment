# `sqlparser::dialect::spark`

Crate `sqlparser` · 1 public items · structured records in [`model/sqlparser.dialect.spark.json`](../model/sqlparser.dialect.spark.json)

## SparkSqlDialect

`struct` · `sqlparser::dialect::spark::SparkSqlDialect`

Also reachable as `sqlparser::dialect::SparkSqlDialect`

```rust
struct SparkSqlDialect
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
fn parse_infix(&self, parser: &mut Parser<'_>, expr: &Expr, _precedence: u8) -> Option<Result<Expr, ParserError>>
fn require_interval_qualifier(&self) -> bool
fn supports_bang_not_operator(&self) -> bool
fn supports_create_table_using(&self) -> bool
fn supports_cte_without_as(&self) -> bool
fn supports_filter_during_aggregation(&self) -> bool
fn supports_group_by_expr(&self) -> bool
fn supports_group_by_with_modifier(&self) -> bool
fn supports_lambda_functions(&self) -> bool
fn supports_long_type_as_bigint(&self) -> bool
fn supports_map_literal_with_angle_brackets(&self) -> bool
fn supports_nested_comments(&self) -> bool
fn supports_select_item_multi_column_alias(&self) -> bool
fn supports_select_wildcard_except(&self) -> bool
fn supports_struct_literal(&self) -> bool
fn supports_values_as_table_factor(&self) -> bool
```

A [`Dialect`] for [Apache Spark SQL](https://spark.apache.org/docs/latest/sql-ref.html).

See <https://spark.apache.org/docs/latest/sql-ref-syntax.html>.

---
