# `sqlparser::dialect::databricks::DatabricksDialect`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.dialect.databricks.DatabricksDialect.json).

<a id="op-f4f7bb4e46a88fbc974c3162"></a>
## DatabricksDialect

`struct` · `sqlparser::dialect::databricks::DatabricksDialect` · sqlparser 0.62.0

```rust
struct DatabricksDialect
```

Source: `src/dialect/databricks.rs:25`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A [`Dialect`](../operations/sqlparser.dialect.Dialect.md#op-f184df0633eac2f7d134147b) for [Databricks SQL](https://www.databricks.com/)

See <https://docs.databricks.com/en/sql/language-manual/index.html>.

<a id="op-d138c59f88ca6303eb02b815"></a>
## clone

`function` · `sqlparser::dialect::databricks::DatabricksDialect::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> DatabricksDialect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::databricks::DatabricksDialect", "path": "DatabricksDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 26], "end": [23, 31], "filename": "src/dialect/databricks.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/dialect/databricks.rs:23`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b87670ce53f913722052c38f"></a>
## cmp

`function` · `sqlparser::dialect::databricks::DatabricksDialect::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &DatabricksDialect) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::databricks::DatabricksDialect", "path": "DatabricksDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 72], "end": [23, 75], "filename": "src/dialect/databricks.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/dialect/databricks.rs:23`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a79d36a6a8141d13981e273c"></a>
## default

`function` · `sqlparser::dialect::databricks::DatabricksDialect::default` · sqlparser 0.62.0

```rust
fn default() -> DatabricksDialect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::databricks::DatabricksDialect", "path": "DatabricksDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 17], "end": [23, 24], "filename": "src/dialect/databricks.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/dialect/databricks.rs:23`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b687c26fe6a3448125733a7"></a>
## deserialize

`function` · `sqlparser::dialect::databricks::DatabricksDialect::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::databricks::DatabricksDialect", "path": "DatabricksDialect"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [24, 56], "end": [24, 74], "filename": "src/dialect/databricks.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/dialect/databricks.rs:24`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d66924edc39160fe1ed0571"></a>
## eq

`function` · `sqlparser::dialect::databricks::DatabricksDialect::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &DatabricksDialect) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::databricks::DatabricksDialect", "path": "DatabricksDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 39], "end": [23, 48], "filename": "src/dialect/databricks.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/dialect/databricks.rs:23`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f7892f8b5a6d0e45c28c3604"></a>
## fmt

`function` · `sqlparser::dialect::databricks::DatabricksDialect::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::databricks::DatabricksDialect", "path": "DatabricksDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 10], "end": [23, 15], "filename": "src/dialect/databricks.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/dialect/databricks.rs:23`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca10181ecff3848a06d22c9c"></a>
## hash

`function` · `sqlparser::dialect::databricks::DatabricksDialect::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::databricks::DatabricksDialect", "path": "DatabricksDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 54], "end": [23, 58], "filename": "src/dialect/databricks.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/dialect/databricks.rs:23`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69a7ff844fea0ab8b0f97c7f"></a>
## is_delimited_identifier_start

`function` · `sqlparser::dialect::databricks::DatabricksDialect::is_delimited_identifier_start` · sqlparser 0.62.0

```rust
fn is_delimited_identifier_start(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::databricks::DatabricksDialect", "path": "DatabricksDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 1], "end": [111, 2], "filename": "src/dialect/databricks.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/databricks.rs:30`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34e719cbd646aba2b2ae281a"></a>
## is_identifier_part

`function` · `sqlparser::dialect::databricks::DatabricksDialect::is_identifier_part` · sqlparser 0.62.0

```rust
fn is_identifier_part(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::databricks::DatabricksDialect", "path": "DatabricksDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 1], "end": [111, 2], "filename": "src/dialect/databricks.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/databricks.rs:38`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d1122a65d78cea92821b3ee"></a>
## is_identifier_start

`function` · `sqlparser::dialect::databricks::DatabricksDialect::is_identifier_start` · sqlparser 0.62.0

```rust
fn is_identifier_start(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::databricks::DatabricksDialect", "path": "DatabricksDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 1], "end": [111, 2], "filename": "src/dialect/databricks.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/databricks.rs:34`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-885d37d39b66ec6176528808"></a>
## partial_cmp

`function` · `sqlparser::dialect::databricks::DatabricksDialect::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &DatabricksDialect) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::databricks::DatabricksDialect", "path": "DatabricksDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 60], "end": [23, 70], "filename": "src/dialect/databricks.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/dialect/databricks.rs:23`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ba23ed0a63f724301685818"></a>
## require_interval_qualifier

`function` · `sqlparser::dialect::databricks::DatabricksDialect::require_interval_qualifier` · sqlparser 0.62.0

```rust
fn require_interval_qualifier(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::databricks::DatabricksDialect", "path": "DatabricksDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 1], "end": [111, 2], "filename": "src/dialect/databricks.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/databricks.rs:69`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a34338857719c8759c88a100"></a>
## serialize

`function` · `sqlparser::dialect::databricks::DatabricksDialect::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::databricks::DatabricksDialect", "path": "DatabricksDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [24, 38], "end": [24, 54], "filename": "src/dialect/databricks.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/dialect/databricks.rs:24`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a3e37a123578a76cae53512"></a>
## supports_bang_not_operator

`function` · `sqlparser::dialect::databricks::DatabricksDialect::supports_bang_not_operator` · sqlparser 0.62.0

```rust
fn supports_bang_not_operator(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::databricks::DatabricksDialect", "path": "DatabricksDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 1], "end": [111, 2], "filename": "src/dialect/databricks.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/databricks.rs:99`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://docs.databricks.com/aws/en/sql/language-manual/functions/bangsign>

<a id="op-c73f31d558f9c7ec486af1bd"></a>
## supports_cte_without_as

`function` · `sqlparser::dialect::databricks::DatabricksDialect::supports_cte_without_as` · sqlparser 0.62.0

```rust
fn supports_cte_without_as(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::databricks::DatabricksDialect", "path": "DatabricksDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 1], "end": [111, 2], "filename": "src/dialect/databricks.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/databricks.rs:104`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://docs.databricks.com/aws/en/sql/language-manual/sql-ref-syntax-qry-select-cte>

<a id="op-3099f9703074677c2819d66a"></a>
## supports_filter_during_aggregation

`function` · `sqlparser::dialect::databricks::DatabricksDialect::supports_filter_during_aggregation` · sqlparser 0.62.0

```rust
fn supports_filter_during_aggregation(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::databricks::DatabricksDialect", "path": "DatabricksDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 1], "end": [111, 2], "filename": "src/dialect/databricks.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/databricks.rs:46`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-94c7f4020594f1efb91d61d5"></a>
## supports_group_by_expr

`function` · `sqlparser::dialect::databricks::DatabricksDialect::supports_group_by_expr` · sqlparser 0.62.0

```rust
fn supports_group_by_expr(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::databricks::DatabricksDialect", "path": "DatabricksDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 1], "end": [111, 2], "filename": "src/dialect/databricks.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/databricks.rs:51`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-566affe4b3c09d4a6f5b90dd"></a>
## supports_group_by_with_modifier

`function` · `sqlparser::dialect::databricks::DatabricksDialect::supports_group_by_with_modifier` · sqlparser 0.62.0

```rust
fn supports_group_by_with_modifier(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::databricks::DatabricksDialect", "path": "DatabricksDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 1], "end": [111, 2], "filename": "src/dialect/databricks.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/databricks.rs:84`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://docs.databricks.com/en/sql/language-manual/sql-ref-syntax-qry-select-groupby.html>

<a id="op-fdb40153c84a15b1020073b3"></a>
## supports_lambda_functions

`function` · `sqlparser::dialect::databricks::DatabricksDialect::supports_lambda_functions` · sqlparser 0.62.0

```rust
fn supports_lambda_functions(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::databricks::DatabricksDialect", "path": "DatabricksDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 1], "end": [111, 2], "filename": "src/dialect/databricks.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/databricks.rs:60`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ebe9b48e9a8afaa87fde5e9f"></a>
## supports_nested_comments

`function` · `sqlparser::dialect::databricks::DatabricksDialect::supports_nested_comments` · sqlparser 0.62.0

```rust
fn supports_nested_comments(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::databricks::DatabricksDialect", "path": "DatabricksDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 1], "end": [111, 2], "filename": "src/dialect/databricks.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/databricks.rs:79`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://docs.databricks.com/aws/en/sql/language-manual/sql-ref-syntax-comment>

<a id="op-f9cacc25d2ef2d801ffe4412"></a>
## supports_numeric_prefix

`function` · `sqlparser::dialect::databricks::DatabricksDialect::supports_numeric_prefix` · sqlparser 0.62.0

```rust
fn supports_numeric_prefix(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::databricks::DatabricksDialect", "path": "DatabricksDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 1], "end": [111, 2], "filename": "src/dialect/databricks.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/databricks.rs:42`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b529297ee7f45285c3fd82d3"></a>
## supports_optimize_table

`function` · `sqlparser::dialect::databricks::DatabricksDialect::supports_optimize_table` · sqlparser 0.62.0

```rust
fn supports_optimize_table(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::databricks::DatabricksDialect", "path": "DatabricksDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 1], "end": [111, 2], "filename": "src/dialect/databricks.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/databricks.rs:94`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://docs.databricks.com/en/sql/language-manual/delta-optimize.html>

<a id="op-d2ddf057a7dcbcd9f39904ba"></a>
## supports_select_item_multi_column_alias

`function` · `sqlparser::dialect::databricks::DatabricksDialect::supports_select_item_multi_column_alias` · sqlparser 0.62.0

```rust
fn supports_select_item_multi_column_alias(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::databricks::DatabricksDialect", "path": "DatabricksDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 1], "end": [111, 2], "filename": "src/dialect/databricks.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/databricks.rs:108`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e0b1d5ebf16863c113889bfc"></a>
## supports_select_wildcard_except

`function` · `sqlparser::dialect::databricks::DatabricksDialect::supports_select_wildcard_except` · sqlparser 0.62.0

```rust
fn supports_select_wildcard_except(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::databricks::DatabricksDialect", "path": "DatabricksDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 1], "end": [111, 2], "filename": "src/dialect/databricks.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/databricks.rs:65`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cdb3eedff465d65ae19725a6"></a>
## supports_struct_literal

`function` · `sqlparser::dialect::databricks::DatabricksDialect::supports_struct_literal` · sqlparser 0.62.0

```rust
fn supports_struct_literal(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::databricks::DatabricksDialect", "path": "DatabricksDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 1], "end": [111, 2], "filename": "src/dialect/databricks.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/databricks.rs:74`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-113062bf230830171116090f"></a>
## supports_table_versioning

`function` · `sqlparser::dialect::databricks::DatabricksDialect::supports_table_versioning` · sqlparser 0.62.0

```rust
fn supports_table_versioning(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::databricks::DatabricksDialect", "path": "DatabricksDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 1], "end": [111, 2], "filename": "src/dialect/databricks.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/databricks.rs:56`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

<https://docs.databricks.com/gcp/en/delta/history#delta-time-travel-syntax>

<a id="op-b4c1f180d46efe4717e30292"></a>
## supports_values_as_table_factor

`function` · `sqlparser::dialect::databricks::DatabricksDialect::supports_values_as_table_factor` · sqlparser 0.62.0

```rust
fn supports_values_as_table_factor(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::databricks::DatabricksDialect", "path": "DatabricksDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 1], "end": [111, 2], "filename": "src/dialect/databricks.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/databricks.rs:89`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://docs.databricks.com/en/sql/language-manual/sql-ref-syntax-qry-select-values.html>
