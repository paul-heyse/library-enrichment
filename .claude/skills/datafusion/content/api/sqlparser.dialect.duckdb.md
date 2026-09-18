# `sqlparser::dialect::duckdb`

Crate `sqlparser` · 1 public items · structured records in [`model/sqlparser.dialect.duckdb.json`](../model/sqlparser.dialect.duckdb.json)

## DuckDbDialect

`struct` · `sqlparser::dialect::duckdb::DuckDbDialect`

Also reachable as `sqlparser::dialect::DuckDbDialect`

```rust
struct DuckDbDialect
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
fn is_identifier_part(&self, ch: char) -> bool
fn is_identifier_start(&self, ch: char) -> bool
fn support_map_literal_syntax(&self) -> bool
fn supports_array_typedef_with_brackets(&self) -> bool
fn supports_bitwise_shift_operators(&self) -> bool
fn supports_comma_separated_trim(&self) -> bool
fn supports_detach(&self) -> bool
fn supports_dictionary_syntax(&self) -> bool
fn supports_explain_with_utility_options(&self) -> bool
fn supports_filter_during_aggregation(&self) -> bool
fn supports_from_first_select(&self) -> bool
fn supports_group_by_expr(&self) -> bool
fn supports_install(&self) -> bool
fn supports_lambda_functions(&self) -> bool
fn supports_load_extension(&self) -> bool
fn supports_named_fn_args_with_assignment_operator(&self) -> bool
fn supports_named_fn_args_with_eq_operator(&self) -> bool
fn supports_notnull_operator(&self) -> bool
fn supports_order_by_all(&self) -> bool
fn supports_select_wildcard_exclude(&self) -> bool
fn supports_select_wildcard_replace(&self) -> bool
fn supports_trailing_commas(&self) -> bool
```

[Full member, field, variant and typed contracts](../operations/sqlparser.dialect.duckdb.DuckDbDialect.md).


A [`Dialect`] for [DuckDB](https://duckdb.org/)

---
