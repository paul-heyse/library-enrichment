# `sqlparser::dialect::spark::SparkSqlDialect`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.dialect.spark.SparkSqlDialect.json).

<a id="op-e75556154f9b25558f8dba0c"></a>
## SparkSqlDialect

`struct` · `sqlparser::dialect::spark::SparkSqlDialect` · sqlparser 0.62.0

```rust
struct SparkSqlDialect
```

Source: `src/dialect/spark.rs:31`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A [`Dialect`](../operations/sqlparser.dialect.Dialect.md#op-f184df0633eac2f7d134147b) for [Apache Spark SQL](https://spark.apache.org/docs/latest/sql-ref.html).

See <https://spark.apache.org/docs/latest/sql-ref-syntax.html>.

<a id="op-653cb0db7d92031f3dfc35c9"></a>
## clone

`function` · `sqlparser::dialect::spark::SparkSqlDialect::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> SparkSqlDialect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::spark::SparkSqlDialect", "path": "SparkSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 26], "end": [29, 31], "filename": "src/dialect/spark.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/dialect/spark.rs:29`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46ae463f694ea76f4014f522"></a>
## cmp

`function` · `sqlparser::dialect::spark::SparkSqlDialect::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &SparkSqlDialect) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::spark::SparkSqlDialect", "path": "SparkSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 72], "end": [29, 75], "filename": "src/dialect/spark.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/dialect/spark.rs:29`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5c51f9a9a3e9a14130c1149"></a>
## default

`function` · `sqlparser::dialect::spark::SparkSqlDialect::default` · sqlparser 0.62.0

```rust
fn default() -> SparkSqlDialect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::spark::SparkSqlDialect", "path": "SparkSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 17], "end": [29, 24], "filename": "src/dialect/spark.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/dialect/spark.rs:29`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7bff60faf63b1e86c2254227"></a>
## deserialize

`function` · `sqlparser::dialect::spark::SparkSqlDialect::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::spark::SparkSqlDialect", "path": "SparkSqlDialect"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 56], "end": [30, 74], "filename": "src/dialect/spark.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/dialect/spark.rs:30`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-863dd25232fbafd936b388cb"></a>
## eq

`function` · `sqlparser::dialect::spark::SparkSqlDialect::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &SparkSqlDialect) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::spark::SparkSqlDialect", "path": "SparkSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 39], "end": [29, 48], "filename": "src/dialect/spark.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/dialect/spark.rs:29`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0298b2f6d83c2ef17afb9bc7"></a>
## fmt

`function` · `sqlparser::dialect::spark::SparkSqlDialect::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::spark::SparkSqlDialect", "path": "SparkSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 10], "end": [29, 15], "filename": "src/dialect/spark.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/dialect/spark.rs:29`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8d255126a2c78438711a3b1"></a>
## hash

`function` · `sqlparser::dialect::spark::SparkSqlDialect::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::spark::SparkSqlDialect", "path": "SparkSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 54], "end": [29, 58], "filename": "src/dialect/spark.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/dialect/spark.rs:29`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e300413bfeab7aca85c72ea"></a>
## is_delimited_identifier_start

`function` · `sqlparser::dialect::spark::SparkSqlDialect::is_delimited_identifier_start` · sqlparser 0.62.0

```rust
fn is_delimited_identifier_start(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::spark::SparkSqlDialect", "path": "SparkSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [145, 2], "filename": "src/dialect/spark.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/spark.rs:35`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d26efbc220f2e321e9f34214"></a>
## is_identifier_part

`function` · `sqlparser::dialect::spark::SparkSqlDialect::is_identifier_part` · sqlparser 0.62.0

```rust
fn is_identifier_part(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::spark::SparkSqlDialect", "path": "SparkSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [145, 2], "filename": "src/dialect/spark.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/spark.rs:43`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-309c5e4a351bc48d73405cb1"></a>
## is_identifier_start

`function` · `sqlparser::dialect::spark::SparkSqlDialect::is_identifier_start` · sqlparser 0.62.0

```rust
fn is_identifier_start(&self, ch: char) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::spark::SparkSqlDialect", "path": "SparkSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [145, 2], "filename": "src/dialect/spark.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/spark.rs:39`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a24439010f4f2ea6b06565f"></a>
## parse_infix

`function` · `sqlparser::dialect::spark::SparkSqlDialect::parse_infix` · sqlparser 0.62.0

```rust
fn parse_infix(&self, parser: &mut Parser<'_>, expr: &Expr, _precedence: u8) -> Option<Result<Expr, ParserError>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::spark::SparkSqlDialect", "path": "SparkSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [145, 2], "filename": "src/dialect/spark.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/spark.rs:124`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Parse the `DIV` keyword as integer division.

Example: `SELECT 10 DIV 3` returns `3`.

See <https://spark.apache.org/docs/latest/sql-ref-functions-builtin-math.html>

<a id="op-35b73521bac06ff6bef2eaa0"></a>
## partial_cmp

`function` · `sqlparser::dialect::spark::SparkSqlDialect::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &SparkSqlDialect) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::spark::SparkSqlDialect", "path": "SparkSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 60], "end": [29, 70], "filename": "src/dialect/spark.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/dialect/spark.rs:29`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0fff76d93919ef1cea7cc63c"></a>
## require_interval_qualifier

`function` · `sqlparser::dialect::spark::SparkSqlDialect::require_interval_qualifier` · sqlparser 0.62.0

```rust
fn require_interval_qualifier(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::spark::SparkSqlDialect", "path": "SparkSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [145, 2], "filename": "src/dialect/spark.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/spark.rs:98`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b1a44b43c7e03d727c774a9"></a>
## serialize

`function` · `sqlparser::dialect::spark::SparkSqlDialect::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::spark::SparkSqlDialect", "path": "SparkSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 38], "end": [30, 54], "filename": "src/dialect/spark.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/dialect/spark.rs:30`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-22404f6b67c99e976b585b63"></a>
## supports_bang_not_operator

`function` · `sqlparser::dialect::spark::SparkSqlDialect::supports_bang_not_operator` · sqlparser 0.62.0

```rust
fn supports_bang_not_operator(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::spark::SparkSqlDialect", "path": "SparkSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [145, 2], "filename": "src/dialect/spark.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/spark.rs:102`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b1baa1b5819cb3237385b1b"></a>
## supports_create_table_using

`function` · `sqlparser::dialect::spark::SparkSqlDialect::supports_create_table_using` · sqlparser 0.62.0

```rust
fn supports_create_table_using(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::spark::SparkSqlDialect", "path": "SparkSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [145, 2], "filename": "src/dialect/spark.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/spark.rs:82`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://spark.apache.org/docs/latest/sql-ref-syntax-ddl-create-table-datasource.html>

<a id="op-75edeb026912a052b703ed6a"></a>
## supports_cte_without_as

`function` · `sqlparser::dialect::spark::SparkSqlDialect::supports_cte_without_as` · sqlparser 0.62.0

```rust
fn supports_cte_without_as(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::spark::SparkSqlDialect", "path": "SparkSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [145, 2], "filename": "src/dialect/spark.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/spark.rs:110`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed6f29a8623aa2e7870bc007"></a>
## supports_filter_during_aggregation

`function` · `sqlparser::dialect::spark::SparkSqlDialect::supports_filter_during_aggregation` · sqlparser 0.62.0

```rust
fn supports_filter_during_aggregation(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::spark::SparkSqlDialect", "path": "SparkSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [145, 2], "filename": "src/dialect/spark.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/spark.rs:48`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://spark.apache.org/docs/latest/sql-ref-functions-builtin-agg.html>

<a id="op-748e23c4722af4b3c9de88d4"></a>
## supports_group_by_expr

`function` · `sqlparser::dialect::spark::SparkSqlDialect::supports_group_by_expr` · sqlparser 0.62.0

```rust
fn supports_group_by_expr(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::spark::SparkSqlDialect", "path": "SparkSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [145, 2], "filename": "src/dialect/spark.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/spark.rs:53`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://spark.apache.org/docs/latest/sql-ref-syntax-qry-select-groupby.html>

<a id="op-18394b5c51871dc977d09c41"></a>
## supports_group_by_with_modifier

`function` · `sqlparser::dialect::spark::SparkSqlDialect::supports_group_by_with_modifier` · sqlparser 0.62.0

```rust
fn supports_group_by_with_modifier(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::spark::SparkSqlDialect", "path": "SparkSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [145, 2], "filename": "src/dialect/spark.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/spark.rs:58`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://spark.apache.org/docs/latest/sql-ref-syntax-qry-select-groupby.html>

<a id="op-9427ad0f1da61fba27a36fb1"></a>
## supports_lambda_functions

`function` · `sqlparser::dialect::spark::SparkSqlDialect::supports_lambda_functions` · sqlparser 0.62.0

```rust
fn supports_lambda_functions(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::spark::SparkSqlDialect", "path": "SparkSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [145, 2], "filename": "src/dialect/spark.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/spark.rs:63`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://spark.apache.org/docs/latest/sql-ref-functions-builtin-higher-order-func.html>

<a id="op-3e7c8761b34cb08061b23559"></a>
## supports_long_type_as_bigint

`function` · `sqlparser::dialect::spark::SparkSqlDialect::supports_long_type_as_bigint` · sqlparser 0.62.0

```rust
fn supports_long_type_as_bigint(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::spark::SparkSqlDialect", "path": "SparkSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [145, 2], "filename": "src/dialect/spark.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/spark.rs:89`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`LONG` is an alias for `BIGINT` in Spark SQL.

See <https://spark.apache.org/docs/latest/sql-ref-datatypes.html>

<a id="op-8b1a6b96e8b080417094a275"></a>
## supports_map_literal_with_angle_brackets

`function` · `sqlparser::dialect::spark::SparkSqlDialect::supports_map_literal_with_angle_brackets` · sqlparser 0.62.0

```rust
fn supports_map_literal_with_angle_brackets(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::spark::SparkSqlDialect", "path": "SparkSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [145, 2], "filename": "src/dialect/spark.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/spark.rs:115`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://spark.apache.org/docs/latest/sql-ref-datatypes.html>

<a id="op-e507f65637a17800f74aa8ab"></a>
## supports_nested_comments

`function` · `sqlparser::dialect::spark::SparkSqlDialect::supports_nested_comments` · sqlparser 0.62.0

```rust
fn supports_nested_comments(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::spark::SparkSqlDialect", "path": "SparkSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [145, 2], "filename": "src/dialect/spark.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/spark.rs:77`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-22d2187b840d8e8b7262838c"></a>
## supports_select_item_multi_column_alias

`function` · `sqlparser::dialect::spark::SparkSqlDialect::supports_select_item_multi_column_alias` · sqlparser 0.62.0

```rust
fn supports_select_item_multi_column_alias(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::spark::SparkSqlDialect", "path": "SparkSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [145, 2], "filename": "src/dialect/spark.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/spark.rs:106`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9dda511a7756ae0dfebff7df"></a>
## supports_select_wildcard_except

`function` · `sqlparser::dialect::spark::SparkSqlDialect::supports_select_wildcard_except` · sqlparser 0.62.0

```rust
fn supports_select_wildcard_except(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::spark::SparkSqlDialect", "path": "SparkSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [145, 2], "filename": "src/dialect/spark.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/spark.rs:68`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://spark.apache.org/docs/latest/sql-ref-syntax-qry-select.html>

<a id="op-f8667ee8cf840e48407c8853"></a>
## supports_struct_literal

`function` · `sqlparser::dialect::spark::SparkSqlDialect::supports_struct_literal` · sqlparser 0.62.0

```rust
fn supports_struct_literal(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::spark::SparkSqlDialect", "path": "SparkSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [145, 2], "filename": "src/dialect/spark.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/spark.rs:73`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://spark.apache.org/docs/latest/sql-ref-datatypes.html>

<a id="op-92204e31080c98aca96ea944"></a>
## supports_values_as_table_factor

`function` · `sqlparser::dialect::spark::SparkSqlDialect::supports_values_as_table_factor` · sqlparser 0.62.0

```rust
fn supports_values_as_table_factor(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::dialect::spark::SparkSqlDialect", "path": "SparkSqlDialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [145, 2], "filename": "src/dialect/spark.rs"}, "trait": {"args": null, "id": "sqlparser::dialect::Dialect", "path": "Dialect"}, "trait_path": "sqlparser::dialect::Dialect"}`

Source: `src/dialect/spark.rs:94`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://spark.apache.org/docs/latest/sql-ref-syntax-qry-select.html>
