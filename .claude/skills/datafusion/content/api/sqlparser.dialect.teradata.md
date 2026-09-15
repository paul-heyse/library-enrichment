# `sqlparser::dialect::teradata`

Crate `sqlparser` · 1 public items · structured records in [`model/sqlparser.dialect.teradata.json`](../model/sqlparser.dialect.teradata.json)

## TeradataDialect

`struct` · `sqlparser::dialect::teradata::TeradataDialect`

Also reachable as `sqlparser::dialect::TeradataDialect`

```rust
struct TeradataDialect
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
fn require_interval_qualifier(&self) -> bool
fn supports_boolean_literals(&self) -> bool
fn supports_comment_on(&self) -> bool
fn supports_create_table_select(&self) -> bool
fn supports_execute_immediate(&self) -> bool
fn supports_group_by_expr(&self) -> bool
fn supports_string_literal_concatenation(&self) -> bool
fn supports_top_before_distinct(&self) -> bool
fn supports_window_function_null_treatment_arg(&self) -> bool
```

A [`Dialect`] for [Teradata](https://docs.teradata.com/).

---
